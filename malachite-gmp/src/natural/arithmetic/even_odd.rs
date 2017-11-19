use natural::Natural;

impl Natural {
    /// Determines whether a `Natural` is even.
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
    ///     assert_eq!(Natural::zero().is_even(), true);
    ///     assert_eq!(Natural::from(123u32).is_even(), false);
    ///     assert_eq!(Natural::from(128u32).is_even(), true);
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().is_even(), true);
    ///     assert_eq!(Natural::from_str("1000000000001").unwrap().is_even(), false);
    /// }
    /// ```
    pub fn is_even(&self) -> bool {
        self.to_u32_wrapping() & 1 == 0
    }

    /// Determines whether a `Natural` is odd.
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
    ///     assert_eq!(Natural::zero().is_odd(), false);
    ///     assert_eq!(Natural::from(123u32).is_odd(), true);
    ///     assert_eq!(Natural::from(128u32).is_odd(), false);
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().is_odd(), false);
    ///     assert_eq!(Natural::from_str("1000000000001").unwrap().is_odd(), true);
    /// }
    /// ```
    pub fn is_odd(&self) -> bool {
        self.to_u32_wrapping() & 1 != 0
    }
}
