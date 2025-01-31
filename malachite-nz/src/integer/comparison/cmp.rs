use crate::integer::Integer;
use std::cmp::Ordering;

impl PartialOrd for Integer {
    /// Compares two [`Integer`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    /// Compares two [`Integer`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(-123) < Integer::from(-122));
    /// assert!(Integer::from(-123) <= Integer::from(-122));
    /// assert!(Integer::from(-123) > Integer::from(-124));
    /// assert!(Integer::from(-123) >= Integer::from(-124));
    /// ```
    fn cmp(&self, other: &Integer) -> Ordering {
        if std::ptr::eq(self, other) {
            Ordering::Equal
        } else {
            match (self.sign, other.sign) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (true, true) => self.abs.cmp(&other.abs),
                (false, false) => other.abs.cmp(&self.abs),
            }
        }
    }
}
