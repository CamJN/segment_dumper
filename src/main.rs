use crate::Mach::{Binary, Fat};
use clap::Parser;
use goblin::mach::{Mach, MachO, MultiArch, SingleArch};
use std::fs;
use std::path::Path;
use std::ffi::os_str::OsStr;

/// Simple program to list segments and sections of mach-o executables
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    binaries: Vec<String>,
}

fn process_multi<'a>(multi: MultiArch<'a>) -> Vec<([u8; 16], [u8; 16])> {
    multi
        .into_iter()
        .filter_map(|res| res.ok())
        .filter_map(|s| match s {
            SingleArch::MachO(m) => Some(m),
            _ => {eprintln!("Passed archive not binary"); None}
        })
        .flat_map(|m| process_macho(m).into_iter()).collect()
}

fn process_macho<'a>(m: MachO<'a>) -> Vec<([u8; 16], [u8; 16])> {
    m.segments
        .sections()
        .flatten()
        .filter_map(|res| res.ok())
        .map(|s| (s.0.segname, s.0.sectname))
        .collect()
}

fn main() {
    let args = Args::parse();

    args.binaries
        .iter()
        .map(Path::new)
        .flat_map(fs::read)
        .flat_map(|bytes| match Mach::parse(&bytes) {
            Ok(Fat(m)) => process_multi(m),
            Ok(Binary(b)) => process_macho(b),
            Err(err) => { eprintln!("{}", err); Vec::new() },
        })
        .for_each(|t| unsafe { println!("{},{}",OsStr::from_encoded_bytes_unchecked(&t.0).display(),OsStr::from_encoded_bytes_unchecked(&t.1).display()) });
}
