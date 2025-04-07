use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::ops::Div;

/// This trait houses various methods for making instructions
/// more generic instead of declaring a separate function for each
/// stack. In a way I'm doing that here, but in a more Rusty way.
///
/// Trig functions named safe rather than checked to not overlap
/// with Decimal library's checked function names.
pub trait NumericTrait: Sized + Div<Output = Self> {
    fn checked_div(self, v: Self) -> Option<Self>;
    fn checked_mod(self, v: Self) -> Option<Self>;
    fn increment(self) -> Self;
    fn decrement(self) -> Self;
    fn safe_sin(self) -> Option<Self>;
    fn safe_cos(self) -> Option<Self>;
    fn safe_tan(self) -> Option<Self>;
    fn inverse(self) -> Option<Self>;
    fn safe_exp(self) -> Option<Self>;
    fn absolute(self) -> Self;
    fn safe_log10(self) -> Option<Self>;
    fn safe_sqrt(self) -> Option<Self>;
    fn sign_reverse(self) -> Self;
    fn square(self) -> Self;
}

impl NumericTrait for Decimal {
    fn checked_div(self, v: Self) -> Option<Self> {
        if v == dec!(0.0) { None } else { Some(self / v) }
    }
    fn checked_mod(self, v: Self) -> Option<Self> {
        if v == dec!(0.0) { None } else { Some(self % v) }
    }
    fn increment(self) -> Self {
        self + dec!(1.0)
    }
    fn decrement(self) -> Self {
        self - dec!(1.0)
    }
    fn safe_sin(self) -> Option<Self> {
        self.checked_sin()
    }
    fn safe_cos(self) -> Option<Self> {
        self.checked_cos()
    }
    fn safe_tan(self) -> Option<Self> {
        self.checked_tan()
    }
    fn inverse(self) -> Option<Self> {
        dec!(1.0).checked_div(self)
    }
    fn safe_exp(self) -> Option<Self> {
        self.checked_exp()
    }
    fn absolute(self) -> Self {
        self.abs()
    }
    fn safe_log10(self) -> Option<Self> {
        self.absolute().checked_log10()
    }
    fn safe_sqrt(self) -> Option<Self> {
        self.absolute().sqrt()
    }
    fn sign_reverse(self) -> Self {
        self * dec!(-1)
    }
    fn square(self) -> Self {
        self * self
    }
}

impl NumericTrait for i128 {
    fn checked_div(self, v: Self) -> Option<Self> {
        if v == 0 { None } else { Some(self / v) }
    }
    fn checked_mod(self, v: Self) -> Option<Self> {
        if v == 0 { None } else { Some(self % v) }
    }
    fn increment(self) -> Self {
        self + 1
    }
    fn decrement(self) -> Self {
        self - 1
    }
    /// Casts the i128 to a Decimal and takes the checked_sin
    /// of the value. Casts the calculated value back to an i128.
    fn safe_sin(self) -> Option<Self> {
        Decimal::from_i128(self)?.checked_sin()?.to_i128()
    }
    fn safe_cos(self) -> Option<Self> {
        Decimal::from_i128(self)?.checked_cos()?.to_i128()
    }
    fn safe_tan(self) -> Option<Self> {
        Decimal::from_i128(self)?.checked_tan()?.to_i128()
    }
    fn inverse(self) -> Option<Self> {
        if self == 0 { None } else { Some(1 / self) }
    }
    fn safe_exp(self) -> Option<Self> {
        Decimal::from_i128(self)?.checked_exp()?.to_i128()
    }
    fn absolute(self) -> Self {
        self.abs()
    }
    fn safe_log10(self) -> Option<Self> {
        Decimal::from_i128(self)?
            .absolute()
            .checked_log10()?
            .to_i128()
    }
    fn safe_sqrt(self) -> Option<Self> {
        Decimal::from_i128(self)?.absolute().sqrt()?.to_i128()
    }
    fn sign_reverse(self) -> Self {
        -1 * self
    }
    fn square(self) -> Self {
        self * self
    }
}

/// A trait for types to implement logical functions that work
/// for push types.
pub trait LogicalTrait {
    fn logical_and(self, v: Self) -> Self;
    fn logical_or(self, v: Self) -> Self;
    fn logical_not(self) -> Self;
    fn logical_xor(self, v: Self) -> Self;
}

impl LogicalTrait for bool {
    fn logical_and(self, v: Self) -> Self {
        self && v
    }
    fn logical_or(self, v: Self) -> Self {
        self || v
    }
    fn logical_not(self) -> Self {
        !self
    }
    fn logical_xor(self, v: Self) -> Self {
        match (self, v) {
            (true, true) | (false, false) => false,
            _ => true,
        }
    }
}

/// A trait for uniform conversions between types.
pub trait CastingTrait: Sized {
    fn from_bool(v: bool) -> Option<Self>;
    fn from_int(v: i128) -> Option<Self>;
    fn from_float(v: Decimal) -> Option<Self>;
}

impl CastingTrait for i128 {
    fn from_bool(v: bool) -> Option<Self> {
        Some(if v { 1 } else { 0 })
    }
    fn from_int(v: i128) -> Option<Self> {
        Some(v)
    }
    fn from_float(v: Decimal) -> Option<Self> {
        v.to_i128()
    }
}

impl CastingTrait for Decimal {
    fn from_bool(v: bool) -> Option<Self> {
        Some(if v { dec!(1.0) } else { dec!(0.0) })
    }
    fn from_int(v: i128) -> Option<Self> {
        Decimal::from_i128(v)
    }
    fn from_float(v: Decimal) -> Option<Self> {
        Some(v)
    }
}

impl CastingTrait for bool {
    fn from_bool(v: bool) -> Option<Self> {
        Some(v)
    }
    fn from_int(v: i128) -> Option<Self> {
        Some(if v != 0 { true } else { false })
    }
    fn from_float(v: Decimal) -> Option<Self> {
        Some(if v != dec!(0.0) { true } else { false })
    }
}
