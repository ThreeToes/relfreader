use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::constants::*;
use crate::util;

pub trait Header {
    fn is_elf(&self) -> bool;
    fn arch_bits(&self) -> u8;
    fn endianess(&self) -> u8;
    fn elf_version(&self) -> u8;
    fn os(&self) -> u8;
    fn abi_version(&self) -> u8;
    fn e_type(&self) -> u16;
    fn e_machine(&self) -> u16;
    fn e_version(&self) -> u32;
    fn e_phoff(&self) -> usize;
    fn e_shoff(&self) -> usize;
    fn e_flags(&self) -> u32;
    fn e_ehsize(&self) -> u16;
    fn e_phentsize(&self) -> u16;
    fn e_phnum(&self) -> u16;
    fn e_shentsize(&self) -> u16;
    fn e_shnum(&self) -> u16;
    fn e_shstrndx(&self) -> u16;
}

pub fn new(path: &String) -> Result<Box<dyn Header>, String> {
    let p = Path::new(path);
    let f = File::open(p);
    match f {
        Ok(f) => {

            let header_bytes: Vec<u8> = f.bytes().take(0x40).map(|x|  {
                match x {
                    Ok(b) => b,
                    _ => 0,
                }
            }).collect();
            if !(
                header_bytes[0] == 0x7f &&
                    header_bytes[1] as char == 'E' &&
                    header_bytes[2] as char == 'L' &&
                    header_bytes[3] as char == 'F') {
                return Err(String::from("Not an elf file"))
            }
            match header_bytes[0x04] {
                CLASS_64 => Ok(Box::new(ElfHeader64::new(header_bytes))),
                CLASS_32 => Ok(Box::new(ElfHeader32::new(header_bytes))),
                _ => Err(format!("Couldn't recognise class byte"))
            }
        },
        Err(e) => Err(format!("{}",e)),
    }
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

    fn endianess(&self) -> u8 {
        self.e_ident[5]
    }

    fn elf_version(&self) -> u8 {
        self.e_ident[6]
    }

    fn os(&self) -> u8 {
        self.e_ident[7]
    }

    fn abi_version(&self) -> u8 {
        self.e_ident[8]
    }

    fn e_type(&self) -> u16 {
        self.e_type
    }

    fn e_machine(&self) -> u16 {
        self.e_machine
    }

    fn e_version(&self) -> u32 {
        self.e_version
    }

    fn e_phoff(&self) -> usize {
        self.e_phoff as usize
    }

    fn e_shoff(&self) -> usize {
        self.e_shoff as usize
    }

    fn e_flags(&self) -> u32 {
        self.e_flags
    }

    fn e_ehsize(&self) -> u16 {
        self.e_ehsize
    }

    fn e_phentsize(&self) -> u16 {
        self.e_phentsize
    }

    fn e_phnum(&self) -> u16 {
        self.e_phnum
    }

    fn e_shentsize(&self) -> u16 {
        self.e_shentsize
    }

    fn e_shnum(&self) -> u16 {
        self.e_shnum
    }

    fn e_shstrndx(&self) -> u16 {
        self.e_shstrndx
    }
}

impl ElfHeader64 {
    pub fn new(header_bytes: Vec<u8>) -> ElfHeader64 {
        let mut e_ident: [u8; 16] = [0;16];
        for i in 0..16 {
            e_ident[i] = header_bytes[i];
        }

        let e_type = util::bytes_to_u16([header_bytes[16], header_bytes[17]], e_ident[5]);
        let e_machine = util::bytes_to_u16([header_bytes[18], header_bytes[19]], e_ident[5]);
        let e_version = util::bytes_to_u32([header_bytes[20], header_bytes[21],
                                               header_bytes[22], header_bytes[23]], e_ident[5]);
        let e_entry = util::bytes_to_u64([header_bytes[24], header_bytes[25],
                                             header_bytes[26], header_bytes[27], header_bytes[28],
                                             header_bytes[29], header_bytes[30], header_bytes[31]],
                                         e_ident[5]);
        let e_phoff = util::bytes_to_u64([header_bytes[32], header_bytes[33],
                                             header_bytes[34], header_bytes[35], header_bytes[36],
                                             header_bytes[37], header_bytes[38], header_bytes[39]],
                                         e_ident[5]);
        let e_shoff = util::bytes_to_u64([header_bytes[40], header_bytes[41],
                                             header_bytes[42], header_bytes[43], header_bytes[44],
                                             header_bytes[45], header_bytes[46], header_bytes[47]],
                                         e_ident[5]);
        let e_flags = util::bytes_to_u32([header_bytes[48], header_bytes[49],
                                             header_bytes[50], header_bytes[51]], e_ident[5]);
        let e_ehsize = util::bytes_to_u16([header_bytes[52], header_bytes[53]], e_ident[5]);
        let e_phentsize = util::bytes_to_u16([header_bytes[54], header_bytes[55]], e_ident[5]);
        let e_phnum = util::bytes_to_u16([header_bytes[56], header_bytes[57]], e_ident[5]);
        let e_shentsize = util::bytes_to_u16([header_bytes[58], header_bytes[59]], e_ident[5]);
        let e_shnum = util::bytes_to_u16([header_bytes[60], header_bytes[61]], e_ident[5]);
        let e_shstrndx = util::bytes_to_u16([header_bytes[62], header_bytes[63]], e_ident[5]);

        let header = ElfHeader64{
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        };


        header
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

    fn endianess(&self) -> u8 {
        self.e_ident[5]
    }

    fn elf_version(&self) -> u8 {
        return self.e_ident[6]
    }

    fn os(&self) -> u8 {
        self.e_ident[7]
    }

    fn abi_version(&self) -> u8 {
        self.e_ident[8]
    }

    fn e_type(&self) -> u16 {
        self.e_type
    }

    fn e_machine(&self) -> u16 {
        self.e_machine
    }

    fn e_version(&self) -> u32 {
        self.e_version
    }

    fn e_phoff(&self) -> usize {
        self.e_phoff as usize
    }

    fn e_shoff(&self) -> usize {
        self.e_shoff as usize
    }

    fn e_flags(&self) -> u32 {
        self.e_flags
    }

    fn e_ehsize(&self) -> u16 {
        self.e_ehsize
    }

    fn e_phentsize(&self) -> u16 {
        self.e_phentsize
    }

    fn e_phnum(&self) -> u16 {
        self.e_phnum
    }

    fn e_shentsize(&self) -> u16 {
        self.e_shentsize
    }

    fn e_shnum(&self) -> u16 {
        self.e_shnum
    }

    fn e_shstrndx(&self) -> u16 {
        self.e_shstrndx
    }
}

impl ElfHeader32 {

    pub fn new(header_bytes: Vec<u8>) -> ElfHeader32 {
        let mut e_ident: [u8; 16] = [0;16];
        for i in 0..16 {
            e_ident[i] = header_bytes[i];
        }

        let e_type = util::bytes_to_u16([header_bytes[16], header_bytes[17]], e_ident[5]);
        let e_machine = util::bytes_to_u16([header_bytes[18], header_bytes[19]], e_ident[5]);
        let e_version = util::bytes_to_u32([header_bytes[20], header_bytes[21],
                                               header_bytes[22], header_bytes[23]], e_ident[5]);
        let e_entry = util::bytes_to_u32([header_bytes[24], header_bytes[25],
                                             header_bytes[26], header_bytes[27]], e_ident[5]);
        let e_phoff = util::bytes_to_u32([header_bytes[28], header_bytes[29],
                                             header_bytes[30], header_bytes[31]], e_ident[5]);
        let e_shoff = util::bytes_to_u32([header_bytes[32], header_bytes[33],
                                             header_bytes[34], header_bytes[35]],
                                         e_ident[5]);
        let e_flags = util::bytes_to_u32([header_bytes[36], header_bytes[37],
                                             header_bytes[38], header_bytes[39]], e_ident[5]);
        let e_ehsize = util::bytes_to_u16([header_bytes[40], header_bytes[41]], e_ident[5]);
        let e_phentsize = util::bytes_to_u16([header_bytes[42], header_bytes[43]], e_ident[5]);
        let e_phnum = util::bytes_to_u16([header_bytes[44], header_bytes[45]], e_ident[5]);
        let e_shentsize = util::bytes_to_u16([header_bytes[46], header_bytes[47]], e_ident[5]);
        let e_shnum = util::bytes_to_u16([header_bytes[48], header_bytes[49]], e_ident[5]);
        let e_shstrndx = util::bytes_to_u16([header_bytes[50], header_bytes[51]], e_ident[5]);

        let header = ElfHeader32{
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        };


        header
    }
}