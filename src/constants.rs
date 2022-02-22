//TODO: Replace below with enums

// EI_CLASS constants, signifies 32 or 64 bit
pub const CLASS_32: u8 = 0x01;
pub const CLASS_64: u8 = 0x02;

/// *class_lookup* returns a string for the class byte
pub fn class_lookup(class: u8) -> String {
    match class {
        CLASS_32 => String::from("32-bit"),
        CLASS_64 => String::from("64-bit"),
        _ => String::from("unknown class")
    }
}

// EI_DATA constants, signifies endianess
pub const DATA_LITTLE_ENDIAN: u8 = 0x01;
pub const DATA_BIG_ENDIAN: u8 = 0x02;

pub fn data_lookup(data: u8) -> String {
    match data {
        DATA_LITTLE_ENDIAN => String::from("Little endian"),
        DATA_BIG_ENDIAN => String::from("Big endian"),
        _ => String::from("unknown endianess"),
    }
}

// EI_VERSION constant, signifies ELF version
pub const VERSION_DEFAULT: u8 = 0x01;

pub fn os_lookup(os: u8) -> String {
    match os {
        0x00 =>  String::from("System V"),
        0x01 =>  String::from("HP-UX"),
        0x02 =>  String::from("NetBSD"),
        0x03 =>  String::from("Linux"),
        0x04 =>  String::from("GNU Hurd"),
        0x06 =>  String::from("Solaris"),
        0x07 =>  String::from("AIX"),
        0x08 =>  String::from("IRIX"),
        0x09 =>  String::from("FreeBSD"),
        0x0A =>  String::from("Tru64"),
        0x0B =>  String::from("Novell Modesto"),
        0x0C =>  String::from("OpenBSD"),
        0x0D =>  String::from("OpenVMS"),
        0x0E =>  String::from("NonStop Kernel"),
        0x0F =>  String::from("AROS"),
        0x10 =>  String::from("Fenix OS"),
        0x11 =>  String::from("CloudABI"),
        0x12 =>  String::from("Stratus Technologies OpenVOS"),
        _ => String::from("Unknown OS")
    }
}

/// *cpu_lookup* maps the e_machine byte to known labels
pub fn isa_lookup(isa: u16) -> String {
    match isa {
        0x00 =>  String::from("No specific instruction set"),
        0x01 =>  String::from("AT&T WE 32100"),
        0x02 =>  String::from("SPARC"),
        0x03 =>  String::from("x86"),
        0x04 =>  String::from("Motorola 68000 (M68k)"),
        0x05 =>  String::from("Motorola 88000 (M88k)"),
        0x06 =>  String::from("Intel MCU"),
        0x07 =>  String::from("Intel 80860"),
        0x08 =>  String::from("MIPS"),
        0x09 =>  String::from("IBM System/370"),
        0x0A =>  String::from("MIPS RS3000 Little-endian"),
        0x0B =>  String::from("Reserved for future use"),
        0x0C =>  String::from("Reserved for future use"),
        0x0D =>  String::from("Reserved for future use"),
        0x0E =>  String::from("Hewlett-Packard PA-RISC"),
        0x0F =>  String::from("Reserved for future use"),
        0x13 =>  String::from("Intel 80960"),
        0x14 =>  String::from("PowerPC"),
        0x15 =>  String::from("PowerPC (64-bit)"),
        0x16 =>  String::from("S390, including S390x"),
        0x17 =>  String::from("IBM SPU/SPC"),
        0x18 =>  String::from("Reserved for future use"),
        0x19 =>  String::from("Reserved for future use"),
        0x1A =>  String::from("Reserved for future use"),
        0x1B =>  String::from("Reserved for future use"),
        0x1C =>  String::from("Reserved for future use"),
        0x1D =>  String::from("Reserved for future use"),
        0x1E =>  String::from("Reserved for future use"),
        0x1F =>  String::from("Reserved for future use"),
        0x20 =>  String::from("Reserved for future use"),
        0x21 =>  String::from("Reserved for future use"),
        0x22 =>  String::from("Reserved for future use"),
        0x23 =>  String::from("Reserved for future use"),
        0x24 =>  String::from("NEC V800"),
        0x25 =>  String::from("Fujitsu FR20"),
        0x26 =>  String::from("TRW RH-32"),
        0x27 =>  String::from("Motorola RCE"),
        0x28 =>  String::from("ARM (up to ARMv7/Aarch32)"),
        0x29 =>  String::from("Digital Alpha"),
        0x2A =>  String::from("SuperH"),
        0x2B =>  String::from("SPARC Version 9"),
        0x2C =>  String::from("Siemens TriCore embedded processor"),
        0x2D =>  String::from("Argonaut RISC Core"),
        0x2E =>  String::from("Hitachi H8/300"),
        0x2F =>  String::from("Hitachi H8/300H"),
        0x30 =>  String::from("Hitachi H8S"),
        0x31 =>  String::from("Hitachi H8/500"),
        0x32 =>  String::from("IA-64"),
        0x33 =>  String::from("Stanford MIPS-X"),
        0x34 =>  String::from("Motorola ColdFire"),
        0x35 =>  String::from("Motorola M68HC12"),
        0x36 =>  String::from("Fujitsu MMA Multimedia Accelerator"),
        0x37 =>  String::from("Siemens PCP"),
        0x38 =>  String::from("Sony nCPU embedded RISC processor"),
        0x39 =>  String::from("Denso NDR1 microprocessor"),
        0x3A =>  String::from("Motorola Star*Core processor"),
        0x3B =>  String::from("Toyota ME16 processor"),
        0x3C =>  String::from("STMicroelectronics ST100 processor"),
        0x3D =>  String::from("Advanced Logic Corp. TinyJ embedded processor family"),
        0x3E =>  String::from("AMD x86-64"),
        0x8C =>  String::from("TMS320C6000 Family"),
        0xAF =>  String::from("MCST Elbrus e2k"),
        0xB7 =>  String::from("ARM 64-bits (ARMv8/Aarch64)"),
        0xF3 =>  String::from("RISC-V"),
        0xF7 =>  String::from("Berkeley Packet Filter"),
        0x101 =>  String::from("WDC 65C816"),
        _ => String::from("Unknown ISA")
    }
}

// E_TYPE constants, type of executable
pub const ET_NONE: u16 = 0x00;
pub const ET_REL: u16 = 0x01;
pub const ET_EXEC: u16 = 0x02;
pub const ET_DYN: u16 = 0x03;
pub const ET_CORE: u16 = 0x04;
pub const ET_LOOS: u16 = 0xFE00;
pub const ET_HIOS: u16 = 0xFEFF;
pub const ET_LOPROC: u16 = 0xFF00;
pub const ET_HIPROC: u16 = 0xFFFF;

pub fn type_lookup(t: u16) -> String {
    match t {
        ET_NONE =>  String::from("Unknown."),
        ET_REL =>  String::from("Relocatable file."),
        ET_EXEC =>  String::from("Executable file."),
        ET_DYN =>  String::from("Shared object."),
        ET_CORE =>  String::from("Core file."),
        ET_LOOS =>  String::from("Reserved inclusive range. Operating system specific."),
        ET_HIOS =>  String::from("Reserved inclusive range. Operating system specific."),
        ET_LOPROC =>  String::from("Reserved inclusive range. Processor specific."),
        ET_HIPROC =>  String::from("Reserved inclusive range. Processor specific."),
        _ => String::from("unknown type"),
    }
}