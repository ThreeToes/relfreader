mod constants;
mod headers;

use std::{env, fs};
use std::fs::File;
use std::path::Path;
use crate::constants::*;
use crate::headers::{ElfHeader64, Header};

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Must pass in file argument");
        return Err(NO_FILE_PROVIDED)
    }

    args.iter().skip(1).for_each(|x| {
        let file = File::open(Path::new(x));
        println!("{}", x);
        match file {
            Ok(f) => {
                let h = ElfHeader64::new(&f);
                if h.is_elf() {
                    println!("Class: {}", class_lookup(h.arch_bits()));
                    println!("Endianess: {}", data_lookup(h.endianess()));
                    println!("OS: {}", os_lookup(h.os()));
                } else {
                    println!("That's not an ELF file");
                }
            },
            Err(e) => {
                println!("couldn't open file: {}", e);
            }
        }

    });
    Ok(())

}
