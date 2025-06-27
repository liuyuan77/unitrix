/// A **type operator** that returns the minimum of `Self` and `Rhs`.
pub trait Min<Rhs = Self> {
    /// The type of the minimum of `Self` and `Rhs`
    type Output;
    /// Method returning the minimum
    fn min(self, rhs: Rhs) -> Self::Output;
}

/// A **type operator** that returns the maximum of `Self` and `Rhs`.
pub trait Max<Rhs = Self> {
    /// The type of the maximum of `Self` and `Rhs`
    type Output;
    /// Method returning the maximum
    fn max(self, rhs: Rhs) -> Self::Output;
}