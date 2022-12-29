use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::factorization::prime_sieve::{
    id_to_n, limbs_prime_sieve_size, limbs_prime_sieve_u64,
};
use crate::num::factorization::traits::Primes;
use crate::num::logic::traits::TrailingZeros;
use std::marker::PhantomData;

// This differs from the identically-named function in malachite-nz; this one returns None if there
// are no more false bits.
fn limbs_index_of_next_false_bit<T: PrimitiveUnsigned>(xs: &[T], start: u64) -> Option<u64> {
    let starting_index = usize::exact_from(start >> T::LOG_WIDTH);
    if starting_index >= xs.len() {
        return None;
    }
    if let Some(result) = xs[starting_index].index_of_next_false_bit(start & T::WIDTH_MASK) {
        if result != T::WIDTH {
            return Some((u64::wrapping_from(starting_index) << T::LOG_WIDTH) + result);
        }
    }
    if starting_index == xs.len() - 1 {
        return None;
    }
    let false_index = starting_index
        + 1
        + xs[starting_index + 1..]
            .iter()
            .take_while(|&&y| y == T::MAX)
            .count();
    if false_index == xs.len() {
        None
    } else {
        Some(
            (u64::exact_from(false_index) << T::LOG_WIDTH)
                + TrailingZeros::trailing_zeros(!xs[false_index]),
        )
    }
}

/// An iterator over that generates all primes less than a given value.
///
/// This `struct` is created by [`Primes::primes_less_than`] and
/// [`Primes::primes_less_than_or_equal_to`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct PrimesLessThanIterator<T: PrimitiveUnsigned> {
    i: u8,
    j: u64,
    sieve: Vec<u64>,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> PrimesLessThanIterator<T> {
    fn new(n: T) -> PrimesLessThanIterator<T> {
        let n: u64 = n.saturating_into();
        let mut sieve;
        if n < 5 {
            sieve = Vec::with_capacity(0);
        } else {
            sieve = vec![0; limbs_prime_sieve_size::<u64>(n)];
            limbs_prime_sieve_u64(&mut sieve, n);
        }
        PrimesLessThanIterator {
            i: 0,
            j: n,
            sieve,
            phantom: PhantomData,
        }
    }
}

impl<T: PrimitiveUnsigned> Iterator for PrimesLessThanIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.i {
            0 => {
                if self.j < 2 {
                    None
                } else {
                    self.i = 1;
                    Some(T::TWO)
                }
            }
            1 => {
                if self.j == 2 {
                    None
                } else {
                    self.i = 2;
                    self.j = 0;
                    Some(T::from(3u8))
                }
            }
            _ => {
                self.j = limbs_index_of_next_false_bit(&self.sieve, self.j)? + 1;
                Some(T::exact_from(id_to_n(self.j)))
            }
        }
    }
}

/// An iterator over that generates all primes.
///
/// This `struct` is created by [`Primes::primes`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct PrimesIterator<T: PrimitiveUnsigned> {
    limit: T,
    xs: PrimesLessThanIterator<T>,
}

impl<T: PrimitiveUnsigned> PrimesIterator<T> {
    fn new() -> PrimesIterator<T> {
        let limit = T::saturating_from(256u16);
        PrimesIterator {
            limit,
            xs: PrimesLessThanIterator::new(limit),
        }
    }
}

impl<T: PrimitiveUnsigned> Iterator for PrimesIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let p = self.xs.next();
            if p.is_some() {
                return p;
            } else if self.limit == T::MAX {
                return None;
            } else {
                self.limit.saturating_mul_assign(T::TWO);
                let j = self.xs.j;
                self.xs = T::primes_less_than_or_equal_to(&self.limit);
                self.xs.i = 3;
                self.xs.j = j;
            }
        }
    }
}

macro_rules! impl_primes {
    ($t:ident) => {
        impl Primes for $t {
            type I = PrimesIterator<$t>;
            type LI = PrimesLessThanIterator<$t>;

            /// Returns an iterator that generates all primes less than a given value.
            ///
            /// The iterator produced by `primes_less_than(n)` generates the same primes as the
            /// iterator produced by `primes().take_while(|&p| p < n)`, but the latter would be
            /// slower because it doesn't know in advance how large its prime sieve should be, and
            /// might have to create larger and larger prime sieves.
            ///
            /// # Worst-case complexity (amortized)
            /// $T(i) = O(\log \log i)$
            ///
            /// $M(i) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
            ///
            /// # Examples
            /// See [here](super::primes#primes_less_than).
            #[inline]
            fn primes_less_than(n: &$t) -> PrimesLessThanIterator<$t> {
                PrimesLessThanIterator::new(n.saturating_sub(1))
            }

            /// Returns an iterator that generates all primes less than or equal to a given value.
            ///
            /// The iterator produced by `primes_less_than_or_equal_to(n)` generates the same
            /// primes as the iterator produced by `primes().take_while(|&p| p <= n)`, but the
            /// latter would be slower because it doesn't know in advance how large its prime sieve
            /// should be, and might have to create larger and larger prime sieves.
            ///
            /// # Worst-case complexity (amortized)
            /// $T(i) = O(\log \log i)$
            ///
            /// $M(i) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
            ///
            /// # Examples
            /// See [here](super::primes#primes_less_than_or_equal_to).
            #[inline]
            fn primes_less_than_or_equal_to(&n: &$t) -> PrimesLessThanIterator<$t> {
                PrimesLessThanIterator::new(n)
            }

            /// Returns all primes that fit into the specified type.
            ///
            /// The iterator produced by `primes(n)` generates the same primes as the iterator
            /// produced by `primes_less_than_or_equal_to(T::MAX)`. If you really need to generate
            /// _every_ prime, and `T` is `u32` or smaller, then you should use the latter, as it
            /// will allocate all the needed memory at once. If `T` is `u64` or larger, or if you
            /// probably don't need every prime, then `primes()` will be faster as it won't
            /// allocate too much memory right away.
            ///
            /// # Worst-case complexity (amortized)
            /// $T(i) = O(\log \log i)$
            ///
            /// $M(i) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
            ///
            /// # Examples
            /// See [here](super::primes#primes).
            #[inline]
            fn primes() -> PrimesIterator<$t> {
                PrimesIterator::new()
            }
        }
    };
}
apply_to_unsigneds!(impl_primes);
