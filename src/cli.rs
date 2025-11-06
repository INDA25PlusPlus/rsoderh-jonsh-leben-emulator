use std::{fs, io, path};

use anyhow::anyhow;
use clap::Parser;

use crate::{instruction::Address, machine::Machine, ui};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Load a program with the assembled machine code from the specified file. Specify '-' to read
    /// from stdin.
    #[arg(long)]
    binary: Option<path::PathBuf>,
}

pub fn start() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let mut machine = Machine::new();

    if let Some(binary_path) = args.binary {
        let mut file: Box<dyn io::Read> = if binary_path.to_str() == Some("-") {
            Box::new(io::stdin())
        } else {
            Box::new(
                fs::OpenOptions::new()
                    .read(true)
                    .open(binary_path)?,
            )
        };

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        
        if machine.memory_mut().write_slice(0, &buf).is_none() {
            return Err(anyhow!("Program doesn't fit in memory. Must be smaller than 256 Kib (65536 bytes)."));
            
        }
        
    }
    
    ui::start(machine)?;

    Ok(())
}
