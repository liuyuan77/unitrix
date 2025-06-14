use core::ops::Not;
use crate::number::{B0, B1, Z0, P1, N1, NonZero, NonNegOne};

// ==================== 按位取反（!运算符） ====================

// ========== 基础类型实现 / Basic Type Implementations ==========
impl Not for Z0 { // Z0 (0) 的按位取反
    type Output = N1;
    fn not(self) -> Self::Output {
        N1
    }
}

impl Not for N1 { // N1 (-1) 的按位取反
    type Output = Z0;
    fn not(self) -> Self::Output {
        Z0
    }
}

impl Not for P1 { // P1 (+1) 的按位取反
    type Output = B0<N1>;
    fn not(self) -> Self::Output {
        B0::new()
    }
}

// ========== 递归类型实现 / Recursive Type Implementations ==========
impl<H: NonZero + NonNegOne + Not> Not for B0<H> { // B0<H> (...0) 的按位取反
    type Output = B1<H::Output>;
    fn not(self) -> Self::Output {
        B1::new()
    }
}

impl<H: NonZero+Not> Not for B1<H> { // B1<H> (...1) 的按位取反
    type Output = B0<H::Output>;
    fn not(self) -> Self::Output {
        B0::new()
    }
}

// ========== 特化实现 ==========
//与P1对应的B0<N1>需要特化
impl Not for B0<N1> { // B0<N1> (-2) 的按位取反
    type Output = P1;
    fn not(self) -> Self::Output {
        P1
    }
}

