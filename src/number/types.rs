use core::marker::PhantomData;
use crate::sealed::Sealed;

// Special floating-point values
#[derive(Debug, PartialEq, Default)]
pub enum Special {
    #[default]
    Nan,        // Not a Number
    Infinity,   // Positive infinity
    NegInfinity,// Negative infinity
}

// Basic numeric type representations

/// Terminal representation for decimal 0
/// - Atomic constant in type system
/// - Cannot be used as generic parameter for B0/B1
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Z0;

/// Positive sign terminator / numeric 1 representation
/// - Standalone: value 1
/// - As generic parameter: 
///   - Current bit is 1
///   - Higher bits are 0
///   - Example: B1<P1> represents 011 (+3)
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct P1;

/// Negative sign terminator / numeric -1 representation
/// - Standalone: value -1
/// - As generic parameter:
///   - Current bit is 1
///   - Higher bits are 1
///   - Example: B0<N1> represents ...1110 (-2)
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct N1;

/// Binary digit 0 for two's complement
/// - H: Higher bits
/// - Example: B0<P1> represents 010 (+2)
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B0<H>(PhantomData<H>);

impl<H> Default for B0<H> {
    fn default() -> Self { B0(PhantomData) }
}

/// Binary digit 1 for two's complement
/// - H: Higher bits
/// - Example: B1<P1> represents 011 (+3)
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B1<H>(PhantomData<H>);

impl<H> Default for B1<H> {
    fn default() -> Self { B1(PhantomData) }
}

/// Floating-point number in type-level scientific notation (M × 2^E)
/// - Mantissa: Significand (includes sign)
/// - Exponent: In two's complement
/// - Supports NaN, +∞, -∞
#[derive(Clone, Copy, Debug)]
pub struct Float<Mantissa, Exponent>(PhantomData<(Mantissa, Exponent)>);

impl<Mantissa, Exponent> Default for Float<Mantissa, Exponent> {
    fn default() -> Self { Float(PhantomData) }
}

/// Bridge between library types and primitive numeric types
/// - Enables mixed operations between custom and primitive types
/// - Provides type-safe operator overloading
/// - Example: Var(3) + P1 → i32 + type-level 1
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Var<T>(pub T);

// Constructors
impl Z0 { #[inline] pub fn new() -> Self { Z0 } }
impl P1 { #[inline] pub fn new() -> Self { P1 } }
impl N1 { #[inline] pub fn new() -> Self { N1 } }

impl<H> B0<H> { 
    #[inline] pub fn new() -> Self { B0(PhantomData) } 
}

impl<H> B1<H> { 
    #[inline] pub fn new() -> Self { B1(PhantomData) } 
}

impl<Mantissa, Exponent> Float<Mantissa, Exponent> {
    #[inline] pub fn new() -> Self { Float(PhantomData) }
}

// Sealed trait implementations
impl Sealed for Special {}
impl Sealed for Z0 {}
impl Sealed for P1 {}
impl Sealed for N1 {}
impl<H> Sealed for B0<H> {}
impl<H> Sealed for B1<H> {}
impl<Mantissa, Exponent> Sealed for Float<Mantissa, Exponent> {}
impl Sealed for Var<i8> {}
impl Sealed for Var<i16> {}
impl Sealed for Var<i32> {}
impl Sealed for Var<i64> {}
impl Sealed for Var<i128> {}
impl Sealed for Var<isize> {}
impl Sealed for Var<f32> {}
impl Sealed for Var<f64> {}