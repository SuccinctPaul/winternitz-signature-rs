mod one_bit_lamport;

pub struct BasicLamportSignature {
    bit_len: usize,
}

impl BasicLamportSignature {
    // Simply choose two random values x0 and x1 in X and set
    // sk := (x0, x1)
    // pk := (H(x0), H(x1))
    pub fn setup() {
        // sk := (x0, x1)

        // pk := (H(x0), H(x1))
    }

    //  S1bit.
    // message m ∈ {0, 1}
    // Write pk = (y0, y1). To sign a one bit message m ∈ {0, 1} output the signature S(sk, m) := xm.
    // Concretely, the signature on the message ‘0’ is x0 and the signature on the message ‘1’ is x1.
    pub fn sign() {
        // S(sk, m) := x_m.
    }

    // To verify a signature σ on m simply check that f(σ) = y_m. We call this system S1bit.
    pub fn verify() {}
}
