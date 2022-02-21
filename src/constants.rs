// Errors
pub const NO_FILE_PROVIDED: i32 = 1;
pub const FILE_NOT_FOUND: i32 = 2;
pub const NOT_ELF_FILE: i32 = 3;

//TODO: Replace below with enums

// EI_CLASS constants, signifies 32 or 64 bit
pub const CLASS_32: u8 = 0x01;
pub const CLASS_64: u8 = 0x02;

// EI_DATA constants, signifies endianess
pub const DATA_LITTLE_ENDIAN: u8 = 0x01;
pub const DATA_BIG_ENDIAN: u8 = 0x02;

// EI_VERSION constant, signifies ELF version
pub const VERSION_DEFAULT: u8 = 0x01;

// EI_OSABI constants, signifies operating system ABI
pub const OSABI_SYSTEM_V: u8 = 0x00;
pub const OSABI_HP_UX: u8 = 0x01;
pub const OSABI_NETBSD: u8 = 0x02;
pub const OSABI_LINUX: u8 = 0x03;
pub const OSABI_HURD: u8 = 0x04;
pub const OSABI_SOLARIS: u8 = 0x06;
pub const OSABI_AIX: u8 = 0x07;
pub const OSABI_IRIX: u8 = 0x08;
pub const OSABI_FREEBSD: u8 = 0x09;
pub const OSABI_TRU64: u8 = 0x0A;
pub const OSABI_MODESTO: u8 = 0x0B;
pub const OSABI_OPENBSD: u8 = 0x0C;
pub const OSABI_OPENVMS: u8 = 0x0D;
pub const OSABI_NONSTOP: u8 = 0x0E;
pub const OSABI_AROS: u8 = 0x0F;
pub const OSABI_FENIX: u8 = 0x10;
pub const OSABI_CLOUDABI: u8 = 0x11;
pub const OSABI_OPENVOS: u8 = 0x12;

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

// E_MACHINE constants, ISA used
pub const E_MACHINE_NO_SPECIFIC: u16 = 0x00;
pub const E_MACHINE_ATTWE_32100: u16 = 0x01;
pub const E_MACHINE_SPARC: u16 = 0x02;
pub const E_MACHINE_X86: u16 = 0x03;
pub const E_MACHINE_M68K: u16 = 0x04;
pub const E_MACHINE_M88K: u16 = 0x05;
pub const E_MACHINE_INTEL_MCU: u16 = 0x06;
pub const E_MACHINE_INTEL_80860: u16 = 0x07;
pub const E_MACHINE_MIPS: u16 = 0x08;
pub const E_MACHINE_IBM_370: u16 = 0x09;
pub const E_MACHINE_MIPS_RS3000_LE: u16 = 0x0A;
pub const E_MACHINE_RESERVED0: u16 = 0x0B;
pub const E_MACHINE_RESERVED1:u16 = 0x0C;
pub const E_MACHINE_RESERVED2: u16 = 0x0D;
pub const E_MACHINE_HP_PA_RISC: u16 = 0x0E;
pub const E_MACHINE_RESERVED3: u16 = 0x0F;
pub const E_MACHINE_RESERVED4: u16 = 0x10;
pub const E_MACHINE_RESERVED5: u16 = 0x11;
pub const E_MACHINE_RESERVED6: u16 = 0x12;
pub const E_MACHINE_INTEL_80960: u16 = 0x13;
pub const E_MACHINE_POWERPC: u16 = 0x14;
pub const E_MACHINE_POWERPC_64: u16 = 0x15;
pub const E_MACHINE_S390: u16 = 0x16;
pub const E_MACHINE_IBM_SPU: u16 = 0x17;
pub const E_MACHINE_RESERVED7: u16 = 0x18;
pub const E_MACHINE_RESERVED8: u16 = 0x19;
pub const E_MACHINE_RESERVED9: u16 = 0x1A;
pub const E_MACHINE_RESERVED10: u16 = 0x1B;
pub const E_MACHINE_RESERVED11: u16 = 0x1C;
pub const E_MACHINE_RESERVED12: u16 = 0x1D;
pub const E_MACHINE_RESERVED13: u16 = 0x1E;
pub const E_MACHINE_RESERVED14: u16 = 0x1F;
pub const E_MACHINE_RESERVED15: u16 = 0x20;
pub const E_MACHINE_RESERVED16: u16 = 0x21;
pub const E_MACHINE_RESERVED17: u16 = 0x22;
pub const E_MACHINE_RESERVED18: u16 = 0x23;
pub const E_MACHINE_NEC_V800: u16 = 0x24;
pub const E_MACHINE_FUJITSU_FR20: u16 = 0x25;
pub const E_MACHINE_TRW_RH_32: u16 = 0x26;
pub const E_MACHINE_MOTOROLA_RCE: u16 = 0x27;
pub const E_MACHINE_ARM: u16 = 0x28;
pub const E_MACHINE_DIGITAL_ALPHA: u16 = 0x29;
pub const E_MACHINE_SUPERH: u16 = 0x2A;
pub const E_MACHINE_SPARC_V9: u16 = 0x2B;
pub const E_MACHINE_SIEMENS_TRICORE: u16 = 0x2C;
pub const E_MACHINE_ARGONAUT_RISC_CORE: u16 = 0x2D;
pub const E_MACHINE_HITACHI_H8300: u16 = 0x2E;
pub const E_MACHINE_HITACHI_H8300H: u16 = 0x2F;
pub const E_MACHINE_HITACHI_H8S: u16 = 0x30;
pub const E_MACHINE_HITACHI_H8500: u16 = 0x31;
pub const E_MACHINE_IA64: u16 = 0x32;
pub const E_MACHINE_STANFORD_MIPS_X: u16 = 0x33;
pub const E_MACHINE_MOTOROLA_COLDFIRE: u16 = 0x34;
pub const E_MACHINE_MOTOROLA_M68HC12: u16 = 0x35;
pub const E_MACHINE_FUJITSU_MMA: u16 = 0x36;
pub const E_MACHINE_SIEMENS_PCP: u16 = 0x37;
pub const E_MACHINE_SONY_NCPU: u16 = 0x38;
pub const E_MACHINE_DENSO_NDR1: u16 = 0x39;
pub const E_MACHINE_MOTOROLA_STAR_CORE: u16 = 0x3A;
pub const E_MACHINE_TOYOTA_ME16: u16 = 0x3B;
pub const E_MACHINE_STM_ST100: u16 = 0x3C;
pub const E_MACHINE_ALC_TINYJ: u16 = 0x3D;
pub const E_MACHINE_AMD_X86_64: u16 = 0x3E;
pub const E_MACHINE_TMS320C6000: u16 = 0x8C;
pub const E_MACHINE_MCST_ELBRUS_E2K: u16 = 0xAF;
pub const E_MACHINE_ARM_64: u16 = 0xB7;
pub const E_MACHINE_RISC_V: u16 = 0xF3;
pub const E_MACHINE_BPF: u16 = 0xF7;
pub const E_MACHINE_WDC_65C816: u16 = 0x101;
