use rsoderh_nhg_leben_emulator::{self, machine::Machine, ui};

fn main() -> anyhow::Result<()> {
    let machine = Machine::new();
    ui::start(machine)
}
