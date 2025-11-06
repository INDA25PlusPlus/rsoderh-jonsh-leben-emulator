use std::{fs, io, path};

use anyhow::anyhow;
use clap::Parser;

use crate::{assembler, coding, machine::Machine, ui};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Load a program with the assembled machine code from the specified file. Specify '-' to read
    /// from stdin.
    #[arg(long)]
    binary: Option<path::PathBuf>,
    #[arg(long)]
    assembly: Option<path::PathBuf>,
}

pub fn start() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let mut machine = Machine::new();

    if let Some(path) = args.binary {
        let mut file: Box<dyn io::Read> = if path.to_str() == Some("-") {
            Box::new(io::stdin())
        } else {
            Box::new(
                fs::OpenOptions::new()
                    .read(true)
                    .open(path)?,
            )
        };

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        
        if machine.memory_mut().write_slice(0, &buf).is_none() {
            return Err(anyhow!("Program doesn't fit in memory. Must be smaller than 256 Kib (65536 bytes)."));
            
        }
    }
    
    if let Some(path) = args.assembly {
        let mut file: Box<dyn io::Read> = if path.to_str() == Some("-") {
            Box::new(io::stdin())
        } else {
            Box::new(
                fs::OpenOptions::new()
                    .read(true)
                    .open(path)?,
            )
        };

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        
        let (instructions, base_addr) = assembler::parse_assembly(&buf).map_err(|err| anyhow!("{}", err))?;
        
        let mut program = Vec::new();
        coding::encode_program(&mut program, &instructions)?;
        
        if machine.memory_mut().write_slice(base_addr, &program).is_none() {
            return Err(anyhow!("Program doesn't fit in memory. It is {} bytes large, but must be smaller than 256 Kib (65536 bytes).", program.len()));
            
        }
    }
    
    ui::start(machine)?;

    Ok(())
}
