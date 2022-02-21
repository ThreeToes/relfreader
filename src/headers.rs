use std::fs::File;
use std::io::Read;
use crate::constants::*;

pub trait Header {
    fn is_elf(&self) -> bool;
    fn arch_bits(&self) -> u8;
}

pub struct ElfHeader64 {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl Header for ElfHeader64{
    fn is_elf(&self) -> bool {
        self.e_ident[0] == 0x7f &&
            self.e_ident[1] as char == 'E' &&
            self.e_ident[2] as char == 'L' &&
            self.e_ident[3] as char == 'F'
    }

    fn arch_bits(&self) -> u8 {
        self.e_ident[4]
    }
}

impl ElfHeader64 {
    pub fn new(f: &File) -> ElfHeader64 {
        let e_ident_bytes = f.bytes().take(16);
        let mut e_ident: [u8; 16] = [0;16];
        let mut cur: usize = 0;
        for x in e_ident_bytes {
            match x {
                Ok(x) => { e_ident[cur] = x; },
                Err(e) => {println!("Couldn't read bytes: {}", e);}
            }
            cur += 1;
        }
        ElfHeader64{
            e_ident,
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

pub struct ElfHeader32 {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl Header for ElfHeader32{
    fn is_elf(&self) -> bool {
        self.e_ident[0] == 0x7f &&
            self.e_ident[1] as char == 'E' &&
            self.e_ident[2] as char == 'L' &&
            self.e_ident[3] as char == 'F'
    }

    fn arch_bits(&self) -> u8 {
        self.e_ident[4]
    }
}