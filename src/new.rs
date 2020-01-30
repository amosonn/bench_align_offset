use core::{intrinsics, mem};

pub unsafe fn align_offset<T: Sized>(p: *const T, a: usize) -> usize {
    /// Calculate multiplicative modular inverse of `x` modulo `m = 2^mpow`.
    ///
    /// This implementation is tailored for align_offset and has following preconditions:
    ///
    /// * The requested modulu `m` is a power-of-two, so `mpow` can be an argument;
    /// * `x < m`; (if `x ≥ m`, pass in `x % m` instead)
    ///
    /// It also leaves reducing the result modulu `m` to the caller, so the result may be larger
    /// than `m`.
    ///
    /// Implementation of this function shall not panic. Ever.
    #[inline]
    fn mod_pow_2_inv(x: usize, mpow: usize) -> usize {
        /// Multiplicative modular inverse table modulo 2⁴ = 16.
        ///
        /// Note, that this table does not contain values where inverse does not exist (i.e., for
        /// `0⁻¹ mod 16`, `2⁻¹ mod 16`, etc.)
        const INV_TABLE_MOD_16: [u8; 8] = [1, 11, 13, 7, 9, 3, 5, 15];
        /// Modulo for which the `INV_TABLE_MOD_16` is intended.
        const INV_TABLE_MOD: usize = 16;
        /// $s$ such that INV_TABLE_MOD = $2^s$.
        const INV_TABLE_MOD_POW: usize = 4;
        /// $s$ such that INV_TABLE_MOD = $2^(s/2)$.
        const INV_TABLE_MOD_POW_TIMES_2: usize = 8;

        let table_inverse = INV_TABLE_MOD_16[(x & (INV_TABLE_MOD - 1)) >> 1] as usize;

        if mpow <= INV_TABLE_MOD_POW {
            table_inverse
        } else {
            // We iterate "up" using the following formula:
            //
            // $$ xy ≡ 1 (mod 2ⁿ) → xy (2 - xy) ≡ 1 (mod 2²ⁿ) $$
            //
            // until 2²ⁿ ≥ m. Then we can reduce to our desired `m` by taking the result `mod m`.
            //
            // Running $k$ iterations starting with a solution valid mod $2^s$ will get us a
            // solution valid mod $2^((2^k) * s)$, so we need to calculate for which $k$,
            // $2^k * s > log2(m)$.
            let mut inverse = table_inverse;
            let mut going_modpow = INV_TABLE_MOD_POW_TIMES_2;
            loop {
                // y = y * (2 - xy)
                //
                // Note, that we use wrapping operations here intentionally – the original formula
                // uses e.g., subtraction `mod n`. It is entirely fine to do them `mod
                // usize::max_value()` instead, because we take the result `mod n` at the end
                // anyway.
                inverse = inverse.wrapping_mul(2usize.wrapping_sub(x.wrapping_mul(inverse)));
                if going_modpow >= mpow {
                    return inverse;
                }
                going_modpow <<= 1;
            }
        }
    }

    let stride = mem::size_of::<T>();
    let a_minus_one = a.wrapping_sub(1);
    let pmoda = p as usize & a_minus_one;

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
    let apow = intrinsics::cttz_nonzero(a);
    let gcdpow = intrinsics::cttz_nonzero(stride).min(apow);
    let gcd = 1usize << gcdpow;

    if p as usize & (gcd - 1) == 0 {
        // This branch solves for the following linear congruence equation:
        //
        // $$ p + so ≡ 0 mod a $$
        //
        // $p$ here is the pointer value, $s$ – stride of `T`, $o$ offset in `T`s, and $a$ – the
        // requested alignment.
        //
        // With $g = gcd(a, s)$$, and the above asserting that $p$ is also divisible by $g$, we can
        // denote $a' = a/g$, $s' = s/g$, $p' = p/g$, then this becomes equivalent to:
        //
        // $$ p' + s'o ≡ 0 mod a' $$
        // $$ o = (a' - (p' mod a')) * ((s')⁻¹ mod a')
        //
        // The first term is “the relative alignment of $p$ to $a$” (divided by the $g$), the second
        // term is “how does incrementing $p$ by $s$ bytes change the relative alignment of $p$” (again
        // divided by $g$).
        // Division by $g$ is necessary to make the inverse well formed if $a$ and $s$ are not
        // co-prime.
        //
        // Furthermore, the result produced by this solution is not “minimal”, so it is necessary
        // to take the result $o mod lcm(s, a)$. We can replace $lcm(s, a)$ with just a $a'$.
        let a2 = a >> gcdpow;
        let s2 = smoda >> gcdpow;
        let minusp2 = a2.wrapping_sub(pmoda >> gcdpow);
        // mod_pow_2_inv returns a result which may be out of $a'$-s range, but it's fine to
        // multiply modulu usize::max_value() here, and then take modulu $a'$ afterwards.
        return minusp2.wrapping_mul(mod_pow_2_inv(s2, apow - gcdpow)) & (a2 - 1);
    }

    // Cannot be aligned at all.
    usize::max_value()
}
