pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes
        .iter()
        .flat_map(|byte| {
            let mut bits = vec![];
            for i in 0..8 {
                let bit = (byte >> i) & 1 == 1;
                bits.push(bit)
            }
            bits
        })
        .collect()
}

fn u32_to_bits(value: u32) -> Vec<bool> {
    bytes_to_bits(&value.to_le_bytes())
}

fn u64_to_bits(value: u64) -> Vec<bool> {
    bytes_to_bits(&value.to_le_bytes())
}
