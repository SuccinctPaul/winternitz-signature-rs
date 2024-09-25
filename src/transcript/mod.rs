use tiny_keccak::{Hasher, Keccak};

pub trait TranscriptHash {
    fn hash_to_u32(v: u32) -> u32;
}

pub struct Keccak256TranscriptHash;

impl TranscriptHash for Keccak256TranscriptHash {
    fn hash_to_u32(v: u32) -> u32 {
        let mut hasher = Keccak::v256();
        hasher.update(b"lamport signature blake3 hash");
        hasher.update(&v.to_be_bytes());

        let mut out = [0u8; 4];
        hasher.finalize(&mut out);

        u32::from_be_bytes(out)
    }
}

pub struct Blake3TranscriptHash;

impl TranscriptHash for Blake3TranscriptHash {
    fn hash_to_u32(v: u32) -> u32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"lamport signature blake3 hash");
        hasher.update(&v.to_be_bytes());
        let out = hasher.finalize();

        u32::from_be_bytes(out.as_bytes()[0..4].try_into().unwrap())
    }
}
