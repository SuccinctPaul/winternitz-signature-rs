// be bits.
pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes.iter().flat_map(bytes_to_be_bits).collect()
}

pub fn bytes_to_be_bits(byte: &u8) -> Vec<bool> {
    let mut bits = vec![];
    for i in 0..8 {
        let bit = (byte >> i) & 1 == 1;
        bits.push(bit)
    }
    bits.reverse();
    bits
}
fn bytes_to_le_bits(byte: u8) -> Vec<bool> {
    let mut bits = vec![];
    for i in 0..8 {
        let bit = (byte >> i) & 1 == 1;
        bits.push(bit)
    }
    bits
}

fn u32_to_bits(value: u32) -> Vec<bool> {
    bytes_to_bits(&value.to_le_bytes())
}

fn u64_to_bits(value: u64) -> Vec<bool> {
    bytes_to_bits(&value.to_le_bytes())
}

pub fn checksum(byte: u32, bit_lens: usize) -> Vec<bool> {
    let mut bits = vec![];

    for i in 0..bit_lens {
        let bit = (byte >> i) & 1 == 1;
        bits.push(bit)
    }
    bits.reverse();
    bits
}

#[cfg(test)]
mod test {
    use crate::utils::checksum;

    #[test]
    fn test_checksum() {
        let m: u8 = 0b01001100;
        let v = 8;
        // num of 0 bits
        let c = 5;
        let target_len = ((v as f32).log2().ceil() as usize) + 1;
        let c_bit = checksum(c, target_len);
        // 0101
        assert_eq!(vec![false, true, false, true], c_bit);
    }
}
