//! Reference: <<A Graduate Course in Applied Cryptography>> 14.1 Basic Lamport signatures

use rand::Rng;

struct SecretKey {
    x0: u32,
    x1: u32,
}

struct PublicKey {
    y0: u32,
    y1: u32,
}

// system S_{1bit}
struct OneBitLamportSignature;

impl OneBitLamportSignature {
    // Simply choose two random values x0 and x1 in X and set
    // sk := (x0, x1)
    // pk := (H(x0), H(x1))
    fn setup<R: Rng>(rng: &mut R) -> (SecretKey, PublicKey) {
        // sk := (x0, x1)
        let x0 = rng.next_u32();
        let x1 = rng.next_u32();

        // pk := (H(x0), H(x1))
        let y0 = blake3_hash(x0);
        let y1 = blake3_hash(x1);

        (SecretKey { x0, x1 }, PublicKey { y0, y1 })
    }

    // message m ∈ {0, 1}
    // Write pk = (y0, y1). To sign a one bit message m ∈ {0, 1} output the signature S(sk, m) := xm.
    // Concretely, the signature on the message ‘0’ is x0 and the signature on the message ‘1’ is x1.
    fn sign(sk: &SecretKey, m: bool) -> u32 {
        // S(sk, m) := x_m.
        if m {
            sk.x1
        } else {
            sk.x0
        }
    }

    // To verify a signature σ on m simply check that f(σ) = y_m. We call this system S1bit.
    fn verify(pk: &PublicKey, m: bool, sig: u32) -> bool {
        let f_tau = blake3_hash(sig);

        let y_m = if m { pk.y1 } else { pk.y0 };

        f_tau == y_m
    }
}

fn blake3_hash(input: u32) -> u32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"one bit lamport");
    hasher.update(&input.to_be_bytes());
    let out = hasher.finalize();

    u32::from_be_bytes(out.as_bytes()[0..4].try_into().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_one_bit_lamport_signature() {
        let mut prng = ChaCha20Rng::seed_from_u64(2);

        let (sk, pk) = OneBitLamportSignature::setup(&mut prng);
        // m = 1
        let m = true;
        let sig = OneBitLamportSignature::sign(&sk, m);
        assert!(
            OneBitLamportSignature::verify(&pk, m, sig),
            "OneBitLamportSignature verify failed when m=1"
        );

        // m = 0
        let m = false;
        let sig = OneBitLamportSignature::sign(&sk, m);
        assert!(
            OneBitLamportSignature::verify(&pk, m, sig),
            "OneBitLamportSignature verify failed when m=0"
        );
    }
}
