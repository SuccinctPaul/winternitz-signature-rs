use crate::transcript::TranscriptHash;
use crate::utils::bytes_to_bits;
use rand::Rng;
use std::marker::PhantomData;

mod one_bit_lamport;

struct SecretKey {
    x0: Vec<u32>,
    x1: Vec<u32>,
}

struct PublicKey {
    y0: Vec<u32>,
    y1: Vec<u32>,
}

// basic Lamport signature system S_L=(G, S, V )
pub struct BasicLamportSignature<T: TranscriptHash> {
    bit_len: usize,
    phantom: PhantomData<T>,
}

impl<T: TranscriptHash> BasicLamportSignature<T> {
    // Simply choose two random values x0 and x1 in X and set
    // sk := (x0, x1)
    // pk := (H(x0), H(x1))
    pub fn gen<R: Rng>(rng: &mut R, bit_num: usize) -> (SecretKey, PublicKey) {
        // sk := (x0, x1)
        let x0 = (0..bit_num)
            .into_iter()
            .map(|_| rng.next_u32())
            .collect::<Vec<_>>();
        let x1 = (0..bit_num)
            .into_iter()
            .map(|_| rng.next_u32())
            .collect::<Vec<_>>();

        // pk := (H(x0), H(x1))
        let y0 = x0.iter().map(|v| T::hash_to_u32(*v)).collect::<Vec<_>>();
        let y1 = x1.iter().map(|v| T::hash_to_u32(*v)).collect::<Vec<_>>();

        (SecretKey { x0, x1 }, PublicKey { y0, y1 })
    }

    // sign a v-bit message

    pub fn sign(sk: &SecretKey, msg: &[u8]) -> Vec<u32> {
        assert!(
            sk.x0.len() >= msg.len() * 8,
            "sk bit len should be greater than msg ones"
        );
        let msg_bits = bytes_to_bits(msg);

        // S(sk, m) := x_m.
        msg_bits
            .into_iter()
            .enumerate()
            .map(|(i, mi)| if mi { sk.x1[i] } else { sk.x0[i] })
            .collect()
    }

    // To verify a signature σ on m simply check that f(σ) = y_m. We call this system S1bit.
    fn verify(pk: &PublicKey, msg: &[u8], sig: Vec<u32>) -> bool {
        assert!(
            pk.y0.len() >= msg.len() * 8,
            "pk bit len should be greater than msg ones"
        );
        assert_eq!(
            msg.len() * 8,
            sig.len(),
            "msg bit len should be equal with sig ones"
        );

        let msg_bits = bytes_to_bits(msg);
        assert_eq!(msg_bits.len(), sig.len());

        for ((i, mi), f_tau) in msg_bits.iter().enumerate().zip(sig) {
            let y_m = if *mi { pk.y1[i] } else { pk.y0[i] };

            if T::hash_to_u32(f_tau) != y_m {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transcript::Blake3TranscriptHash;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_basiclamport_signature() {
        let mut prng = ChaCha20Rng::seed_from_u64(7987979837453);
        let msg = b"hello, here is the test for basic Lamport signature";
        let msg_len = msg.len() * 8;

        let (sk, pk) = BasicLamportSignature::<Blake3TranscriptHash>::gen(&mut prng, msg_len);
        let sig = BasicLamportSignature::<Blake3TranscriptHash>::sign(&sk, msg);
        assert!(
            BasicLamportSignature::<Blake3TranscriptHash>::verify(&pk, msg, sig),
            "OneBitLamportSignature verify failed when m=1"
        );
    }
}
