use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Sub, SubAssign};

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by value. If the `u32` is greater than
/// the `Natural`, returns `None`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(123u32) - 123), "Some(0)");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - 0), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32) - 123), "Some(333)");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - 456), "None");
/// assert_eq!(format!("{:?}", Natural::from_str("1000000000000").unwrap() - 123),
///            "Some(999999999877)");
/// ```
impl Sub<u32> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: u32) -> Option<Natural> {
        if sub_assign_u32_helper(&mut self, other) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by reference. If the `u32` is greater
/// than the `Natural`, returns `None`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - 123), "Some(0)");
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - 0), "Some(123)");
/// assert_eq!(format!("{:?}", &Natural::from(456u32) - 123), "Some(333)");
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - 456), "None");
/// assert_eq!(format!("{:?}", &Natural::from_str("1000000000000").unwrap() - 123),
///            "Some(999999999877)");
/// ```
impl<'a> Sub<u32> for &'a Natural {
    type Output = Option<Natural>;

    fn sub(self, other: u32) -> Option<Natural> {
        if other == 0 {
            return Some(self.clone());
        }
        match *self {
            Small(small) if small >= other => {
                match small.checked_sub(other) {
                    Some(difference) => Some(Small(difference)),
                    None => unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_ui(&mut result, small.into());
                        gmp::mpz_sub_ui(&mut result, &result, other.into());
                        Some(Large(result))
                    },
                }
            }
            Small(_) => None,
            Large(ref large) => unsafe {
                // At this point self >= other
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut result);
                gmp::mpz_sub_ui(&mut result, large, other.into());
                let mut result = Large(result);
                result.demote_if_small();
                Some(result)
            },
        }
    }
}

/// Subtracts a `Natural` from a `u32`, taking the `Natural` by reference. If the `Natural` is
/// greater than the `u32`, returns `None`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(format!("{:?}", 123 - &Natural::from(123u32)), "Some(0)");
///     assert_eq!(format!("{:?}", 123 - &Natural::zero()), "Some(123)");
///     assert_eq!(format!("{:?}", 456 - &Natural::from(123u32)), "Some(333)");
///     assert_eq!(format!("{:?}", 123 - &Natural::from(456u32)), "None");
///     assert_eq!(format!("{:?}", 123 - &Natural::from_str("1000000000000").unwrap()), "None");
/// }
/// ```
impl<'a> Sub<&'a Natural> for u32 {
    type Output = Option<Natural>;

    fn sub(self, other: &'a Natural) -> Option<Natural> {
        other.to_u32().and_then(|x| self.checked_sub(x)).map(
            Natural::from,
        )
    }
}

/// Subtracts a `u32` from a `Natural` in place.
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
///
/// let mut x = Natural::from(15u32);
/// x -= 1;
/// x -= 2;
/// x -= 3;
/// x -= 4;
/// assert_eq!(x.to_string(), "5");
/// ```
impl SubAssign<u32> for Natural {
    fn sub_assign(&mut self, other: u32) {
        if sub_assign_u32_helper(self, other) {
            panic!(
                "Cannot subtract a u32 from a smaller Natural. self: {}, other: {}",
                *self,
                other
            );
        }
    }
}

pub(crate) fn sub_assign_u32_helper(x: &mut Natural, y: u32) -> bool {
    if y == 0 {
        return false;
    }
    match *x {
        Small(ref mut small) => {
            return match small.checked_sub(y) {
                Some(difference) => {
                    *small = difference;
                    false
                }
                None => true,
            };
        }
        Large(ref mut large) => unsafe {
            gmp::mpz_sub_ui(large, large, y.into());
        },
    }
    x.demote_if_small();
    false
}
