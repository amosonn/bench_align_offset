use core::intrinsics;

pub unsafe fn align_offset(p: usize, stride: usize, a: usize) -> usize {
    /// Calculate multiplicative modular inverse of `x` modulo `m`.
    ///
    /// This implementation is tailored for align_offset and has following preconditions:
    ///
    /// * `m` is a power-of-two;
    /// * `x < m`; (if `x ≥ m`, pass in `x % m` instead)
    ///
    /// Implementation of this function shall not panic. Ever.
    #[inline]
    fn mod_inv(x: usize, m: usize) -> usize {
        /// Multiplicative modular inverse table modulo 2⁴ = 16.
        ///
        /// Note, that this table does not contain values where inverse does not exist (i.e., for
        /// `0⁻¹ mod 16`, `2⁻¹ mod 16`, etc.)
        const INV_TABLE_MOD_16: [u8; 8] = [1, 11, 13, 7, 9, 3, 5, 15];
        /// Modulo for which the `INV_TABLE_MOD_16` is intended.
        const INV_TABLE_MOD: usize = 16;
        /// INV_TABLE_MOD²
        const INV_TABLE_MOD_SQUARED: usize = INV_TABLE_MOD * INV_TABLE_MOD;

        let table_inverse = INV_TABLE_MOD_16[(x & (INV_TABLE_MOD - 1)) >> 1] as usize;
        if m <= INV_TABLE_MOD {
            table_inverse & (m - 1)
        } else {
            // We iterate "up" using the following formula:
            //
            // $$ xy ≡ 1 (mod 2ⁿ) → xy (2 - xy) ≡ 1 (mod 2²ⁿ) $$
            //
            // until 2²ⁿ ≥ m. Then we can reduce to our desired `m` by taking the result `mod m`.
            let mut inverse = table_inverse;
            let mut going_mod = INV_TABLE_MOD_SQUARED;
            loop {
                // y = y * (2 - xy) mod n
                //
                // Note, that we use wrapping operations here intentionally – the original formula
                // uses e.g., subtraction `mod n`. It is entirely fine to do them `mod
                // usize::max_value()` instead, because we take the result `mod n` at the end
                // anyway.
                inverse = inverse.wrapping_mul(2usize.wrapping_sub(x.wrapping_mul(inverse)))
                    & (going_mod - 1);
                if going_mod >= m {
                    return inverse & (m - 1);
                }
                going_mod = going_mod.wrapping_mul(going_mod);
            }
        }
    }

    let a_minus_one = a.wrapping_sub(1);
    let pmoda = p & a_minus_one;

    if pmoda == 0 {
        // Already aligned. Yay!
        return 0;
    }

    if stride <= 1 {
        return if stride == 0 {
            // If the pointer is not aligned, and the element is zero-sized, then no amount of
            // elements will ever align the pointer.
            !0
        } else {
            a.wrapping_sub(pmoda)
        };
    }

    let smoda = stride & a_minus_one;
    // a is power-of-two so cannot be 0. stride = 0 is handled above.
    let gcdpow = intrinsics::cttz_nonzero(stride).min(intrinsics::cttz_nonzero(a));
    let gcd = 1usize << gcdpow;

    if p & (gcd - 1) == 0 {
        // This branch solves for the following linear congruence equation:
        //
        // $$ p + so ≡ 0 mod a $$
        //
        // $p$ here is the pointer value, $s$ – stride of `T`, $o$ offset in `T`s, and $a$ – the
        // requested alignment.
        //
        // g = gcd(a, s)
        // o = (a - (p mod a))/g * ((s/g)⁻¹ mod a)
        //
        // The first term is “the relative alignment of p to a”, the second term is “how does
        // incrementing p by s bytes change the relative alignment of p”. Division by `g` is
        // necessary to make this equation well formed if $a$ and $s$ are not co-prime.
        //
        // Furthermore, the result produced by this solution is not “minimal”, so it is necessary
        // to take the result $o mod lcm(s, a)$. We can replace $lcm(s, a)$ with just a $a / g$.
        let j = a.wrapping_sub(pmoda) >> gcdpow;
        let k = smoda >> gcdpow;
        return (j.wrapping_mul(mod_inv(k, a))) & ((a >> gcdpow).wrapping_sub(1));
    }

    // Cannot be aligned at all.
    usize::max_value()
}
