use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::ops::Div;

pub trait CheckedDiv: Sized + Div<Output = Self> {
    fn checked_div(self, v: Self) -> Option<Self>;
    fn checked_mod(self, v: Self) -> Option<Self>;
}

impl CheckedDiv for Decimal {
    fn checked_div(self, v: Self) -> Option<Self> {
        if v == dec!(0.0) { None } else { Some(self / v) }
    }
    fn checked_mod(self, v: Self) -> Option<Self> {
        if v == dec!(0.0) { None } else { Some(self % v) }
    }
}

impl CheckedDiv for i128 {
    fn checked_div(self, v: Self) -> Option<Self> {
        if v == 0 { None } else { Some(self / v) }
    }
    fn checked_mod(self, v: Self) -> Option<Self> {
        if v == 0 { None } else { Some(self % v) }
    }
}
