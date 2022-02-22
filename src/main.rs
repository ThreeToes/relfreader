mod constants;
mod headers;
mod util;

use std::env;
use crate::constants::*;

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Must pass in file argument");
        return Err(0x1)
    }

    args.iter().skip(1).for_each(|x| {
        println!("{}", x);
        let res = headers::new(x);
        match res {
            Ok(h) => {
                if h.is_elf() {
                    println!("Class: {}", class_lookup(h.arch_bits()));
                    println!("Endianess: {}", data_lookup(h.endianess()));
                    println!("OS: {}", os_lookup(h.os()));
                    println!("Type: {}", type_lookup(h.e_type()));
                    println!("Machine: {}", isa_lookup(h.e_machine()));
                } else {
                    println!("That's not an ELF file");
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }

    });
    Ok(())

}
