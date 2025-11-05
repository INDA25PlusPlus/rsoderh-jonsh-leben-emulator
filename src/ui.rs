use std::{io, sync::mpsc, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, KeyCode},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use tui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    instruction::{Data16, Register, Register8, Register16},
    machine::Machine,
    ui::memory_view::MemoryView,
};

mod memory_view;

struct Ui {
    machine: Machine,
    quit_sender: mpsc::Sender<()>,
}

impl Ui {
    fn new(machine: Machine, quit_sender: mpsc::Sender<()>) -> Self {
        Self {
            machine,
            quit_sender,
        }
    }

    fn tick(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> anyhow::Result<()> {
        self.draw(terminal)
    }

    fn draw(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
        terminal.draw(|f| {
            static REGISTERS_HEIGHT: u16 = 5 + 2;
            let registers_instructions_area_height = Constraint::Ratio(2, 5)
                .apply(f.size().height)
                .max(REGISTERS_HEIGHT);

            let mut memory_area = f.size();
            memory_area.height -= registers_instructions_area_height;

            let mut registers_instructions_area = f.size();
            registers_instructions_area.height = registers_instructions_area_height;
            registers_instructions_area.y = memory_area.bottom();

            let [registers_area, instructions_area]: [Rect; 2] = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(28 + 2), Constraint::Ratio(1, 1)].as_ref())
                .split(registers_instructions_area)
                .try_into()
                .expect("We created two areas");

            self.draw_memory(f, memory_area);

            self.draw_registers(f, registers_area);
            self.draw_instructions(f, instructions_area);
        })?;
        Ok(())
    }

    fn draw_memory(&self, f: &mut Frame<'_, CrosstermBackend<io::Stdout>>, area: Rect) {
        let block = Block::default()
            .title(Span::styled(
                "Memory",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::all())
            .border_type(BorderType::Rounded);
        let widget_area = block.inner(area).inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });
        f.render_widget(block, area);

        let memory_view = MemoryView::new(self.machine.memory().as_raw())
            .shown_address(Data16::from(0))
            .label_style(Style::default())
            .address_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(memory_view, widget_area);
    }

    fn draw_registers(&self, f: &mut Frame<'_, CrosstermBackend<io::Stdout>>, area: Rect) {
        let block = Block::default()
            .title(Span::styled(
                "Registers",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::all())
            .border_type(BorderType::Rounded);
        let list_area = block.inner(area).inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });
        f.render_widget(block, area);

        let column_areas: [Rect; 3] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(7 + 1),
                    Constraint::Length(7 + 1),
                    Constraint::Length(10 + 1),
                ]
                .as_ref(),
            )
            .split(list_area)
            .try_into()
            .expect("We created 3 areas");

        const ROWS: usize = 5;
        let register_grid: [[Option<Register>; ROWS]; 3] = [
            [
                Some(Register::Register8(Register8::B)),
                Some(Register::Register8(Register8::D)),
                Some(Register::Register8(Register8::H)),
                Some(Register::Register8(Register8::M)),
                Some(Register::Register8(Register8::A)),
            ],
            [
                Some(Register::Register8(Register8::C)),
                Some(Register::Register8(Register8::E)),
                Some(Register::Register8(Register8::L)),
                None,
                None,
            ],
            [
                Some(Register::Register16(Register16::Bc)),
                Some(Register::Register16(Register16::De)),
                Some(Register::Register16(Register16::Hl)),
                Some(Register::Register16(Register16::Sp)),
                None,
            ],
        ];

        for (col_index, rows) in register_grid.into_iter().enumerate() {
            let areas: [Rect; ROWS] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1); ROWS].as_ref())
                .split(column_areas[col_index])
                .try_into()
                .expect("We created LENGTH areas");

            for (row_index, register) in rows.into_iter().enumerate() {
                let Some(register) = register else {
                    continue;
                };
                let value_string = match register {
                    Register::Register8(register) => {
                        let value = self
                            .machine
                            .registers()
                            .get_8(register, self.machine.memory());
                        format!("0x{:02x}", value)
                    }
                    Register::Register16(register) => {
                        let value = self.machine.registers().get_16(register);
                        format!("0x{:04x}", value.value())
                    }
                };
                let par = Paragraph::new(vec![Spans::from(vec![
                    Span::raw(format!("{}", register)),
                    Span::raw(": "),
                    Span::styled(value_string, Style::default().add_modifier(Modifier::BOLD)),
                ])]);

                f.render_widget(par, areas[row_index]);
            }
        }
    }

    fn draw_instructions(&self, f: &mut Frame<'_, CrosstermBackend<io::Stdout>>, area: Rect) {
        let block = Block::default()
            .title(Span::styled(
                "Instructions",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            // .(alignment)
            .borders(Borders::all())
            // .borders(flag)
            .border_type(BorderType::Rounded);
        let block_area = block.inner(area).inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });
        f.render_widget(block, area);

        {
            let value = self.machine.pc();
            let pc = Paragraph::new(Spans::from(vec![
                Span::raw("PC: "),
                Span::styled(
                    format!("0x{:04x}", value.value()),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]));
            f.render_widget(pc, block_area);
        }

        let mut instructions_area = block_area;
        instructions_area.y -= 1;
        instructions_area.height -= 1;
    }

    fn input(&mut self, event: event::KeyEvent) -> anyhow::Result<()> {
        match event.code {
            KeyCode::Char('q') => {
                self.quit_sender.send(())?;
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn start(machine: Machine) -> anyhow::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    enable_raw_mode()?;

    let (quit_sender, quit_receiver) = mpsc::channel::<()>();
    let mut ui = Ui::new(machine, quit_sender.clone());

    std::thread::spawn(move || -> Result<(), anyhow::Error> {
        loop {
            ui.tick(&mut terminal)?;
            if event::poll(Duration::from_millis(100))? {
                if let event::Event::Key(key_event) = event::read()? {
                    if key_event.code == event::KeyCode::Char('c')
                        && key_event
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        // signal by settting our AtomicBool to false
                        quit_sender.send(())?;
                    } else {
                        ui.input(key_event)?;
                    }
                }
            }
        }
    });

    quit_receiver.iter().next();

    // restore terminal
    let mut backend = CrosstermBackend::new(io::stdout());
    disable_raw_mode()?;
    execute!(&mut backend, LeaveAlternateScreen, DisableMouseCapture,)?;
    backend.show_cursor()?;

    Ok(())
}
