use crate::constants::{DATA_BIG_ENDIAN, DATA_LITTLE_ENDIAN};

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_bytes_to_u16_le() {
        let b: [u8; 2] = [0xFF, 0x88];
        assert_eq!(bytes_to_u16_le(b), 0x88FFu16);
    }

    #[test]
    fn test_bytes_to_u16_be() {
        let b: [u8; 2] = [0xFF, 0x88];
        assert_eq!(bytes_to_u16_be(b), 0xFF88u16);
    }
}

fn bytes_to_u16_le(bytes: [u8; 2]) -> u16 {
    let ret: u16 = bytes[1] as u16;
    (ret << 8) + bytes[0] as u16
}

fn bytes_to_u16_be(bytes: [u8; 2]) -> u16 {
    let ret: u16 = bytes[0] as u16;
    (ret << 8) + bytes[1] as u16
}

pub fn bytes_to_u16(bytes: [u8; 2], data_flag: u8) -> u16 {
    match data_flag {
        DATA_BIG_ENDIAN => bytes_to_u16_be(bytes),
        DATA_LITTLE_ENDIAN => bytes_to_u16_le(bytes),
        _ => 0,
    }
}

fn bytes_to_u32_le(bytes: [u8; 4]) -> u32 {
    let ret: u32 = bytes[3] as u32;
    (ret << 24) + ((bytes[2] as u32) << 16) + ((bytes[1] as u32) << 8) + bytes[0] as u32
}

fn bytes_to_u32_be(bytes: [u8; 4]) -> u32 {
    let ret: u32 = bytes[0] as u32;
    (ret << 24) + ((bytes[1] as u32) << 16) + ((bytes[2] as u32) << 8) + bytes[3] as u32
}


pub fn bytes_to_u32(bytes: [u8; 4], data_flag: u8) -> u32 {
    match data_flag {
        DATA_BIG_ENDIAN => bytes_to_u32_be(bytes),
        DATA_LITTLE_ENDIAN => bytes_to_u32_le(bytes),
        _ => 0,
    }
}

fn bytes_to_u64_le(bytes: [u8; 8]) -> u64 {
    let ret: u64 = bytes[7] as u64;
    (ret << 56) + ((bytes[6] as u64) << 48) + ((bytes[5] as u64) << 40) +
        ((bytes[4] as u64) << 32) + ((bytes[3] as u64) << 24) +
        ((bytes[2] as u64) << 16) + ((bytes[1] as u64) << 8) + bytes[0] as u64
}

fn bytes_to_u64_be(bytes: [u8; 8]) -> u64 {
    let ret: u64 = bytes[0] as u64;
    (ret << 56) + ((bytes[1] as u64) << 48) + ((bytes[2] as u64) << 40) +
        ((bytes[3] as u64) << 32) + ((bytes[4] as u64) << 24) + ((bytes[5] as u64) << 16) +
        ((bytes[6] as u64) << 8) + bytes[7] as u64
}


pub fn bytes_to_u64(bytes: [u8; 8], data_flag: u8) -> u64 {
    match data_flag {
        DATA_BIG_ENDIAN => bytes_to_u64_be(bytes),
        DATA_LITTLE_ENDIAN => bytes_to_u64_le(bytes),
        _ => 0,
    }
}
