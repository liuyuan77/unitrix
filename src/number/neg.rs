use core::ops::{Not, Neg};
use crate::number::{B0, B1, Z0, P1, N1, NonZero};

// ==================== 算术取负（-运算符） ====================
impl Neg for Z0 { // Z0 (0) 的取负
    type Output = Z0;
    fn neg(self) -> Self::Output {
        Z0
    }
}

impl Neg for N1 { // N1 (-1) 的取负,用P1替换B1<Z0>
    type Output = P1;
    fn neg(self) -> Self::Output {
        P1
    }
}

impl Neg for P1 {
    type Output = N1;
    fn neg(self) -> Self::Output {
        N1
    }
}

impl<H: NonZero + Neg> Neg for B0<H>{
    type Output = B0<H::Output>;
    fn neg(self) -> Self::Output {
        B0::new()
    }
}

impl<H: NonZero + Not> Neg for B1<H>{
    type Output = B0<H::Output>;
    fn neg(self) -> Self::Output {
        B0::new()
    }
}