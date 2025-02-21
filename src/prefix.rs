use std::{
    fmt::Display,
    ops::{Div, Mul},
};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum UnitPfx {
    Tera,
    Giga,
    Mega,
    Kilo,
    #[default]
    None,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
}

impl UnitPfx {
    pub fn value(&self) -> f64 {
        match self {
            UnitPfx::Tera => 1e12,
            UnitPfx::Giga => 1e9,
            UnitPfx::Mega => 1e6,
            UnitPfx::Kilo => 1e3,
            UnitPfx::None => 1.0,
            UnitPfx::Milli => 1e-3,
            UnitPfx::Micro => 1e-6,
            UnitPfx::Nano => 1e-9,
            UnitPfx::Pico => 1e-12,
            UnitPfx::Femto => 1e-15,
        }
    }

    pub fn exp_value(&self) -> i32 {
        match self {
            UnitPfx::Tera => 12,
            UnitPfx::Giga => 9,
            UnitPfx::Mega => 6,
            UnitPfx::Kilo => 3,
            UnitPfx::None => 0,
            UnitPfx::Milli => -3,
            UnitPfx::Micro => -6,
            UnitPfx::Nano => -9,
            UnitPfx::Pico => -12,
            UnitPfx::Femto => -15,
        }
    }

    pub fn from_value(value: f64) -> Self {
        match value {
            v if v >= 1e12 => UnitPfx::Tera,
            v if (1e9..1e12).contains(&v) => UnitPfx::Giga,
            v if (1e6..1e9).contains(&v) => UnitPfx::Mega,
            v if (1e3..1e6).contains(&v) => UnitPfx::Kilo,
            v if (1.0..1e3).contains(&v) => UnitPfx::None,
            v if (1e-3..1.0).contains(&v) => UnitPfx::Milli,
            v if (1e-6..1e-3).contains(&v) => UnitPfx::Micro,
            v if (1e-9..1e-6).contains(&v) => UnitPfx::Nano,
            v if (1e-12..1e-9).contains(&v) => UnitPfx::Pico,
            _ => UnitPfx::Femto,
        }
    }

    pub fn from_exp(exp: i32) -> Self {
        match exp {
            v if (9..12).contains(&v) => UnitPfx::Giga,
            v if (6..9).contains(&v) => UnitPfx::Mega,
            v if (3..6).contains(&v) => UnitPfx::Kilo,
            v if (0..3).contains(&v) => UnitPfx::None,
            v if (-3..0).contains(&v) => UnitPfx::Milli,
            v if (-6..-3).contains(&v) => UnitPfx::Micro,
            v if (-9..-6).contains(&v) => UnitPfx::Nano,
            v if (-12..-9).contains(&v) => UnitPfx::Pico,
            _ => UnitPfx::Femto,
        }
    }

    pub fn get_conversion_factor(&self, rhs: &UnitPfx) -> f64 {
        self.value() / rhs.value()
    }

    pub fn min() -> Self {
        Self::Femto
    }

    pub fn max() -> Self {
        Self::Tera
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for UnitPfx {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_exp(self.exp_value() + rhs.exp_value())
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for UnitPfx {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_exp(self.exp_value() - rhs.exp_value())
    }
}
// Default UnitPfx is None
#[derive(Debug)]
pub enum UnitPfxError {}

impl std::str::FromStr for UnitPfx {
    type Err = UnitPfxError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("T") {
            Ok(Self::Tera)
        } else if s.contains("G") {
            Ok(Self::Giga)
        } else if s.contains("M") {
            Ok(Self::Mega)
        } else if s.contains("K") {
            Ok(Self::Kilo)
        } else if s.contains("m") {
            Ok(Self::Milli)
        } else if s.contains("u") {
            Ok(Self::Micro)
        } else if s.contains("n") {
            Ok(Self::Nano)
        } else if s.contains("p") {
            Ok(Self::Pico)
        } else if s.contains("f") {
            Ok(Self::Femto)
        } else {
            Ok(Self::None)
        }
    }
}

impl Display for UnitPfx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            UnitPfx::Tera => "T",
            UnitPfx::Giga => "G",
            UnitPfx::Mega => "M",
            UnitPfx::Kilo => "K",
            UnitPfx::None => "",
            UnitPfx::Milli => "m",
            UnitPfx::Micro => "µ",
            UnitPfx::Nano => "n",
            UnitPfx::Pico => "p",
            UnitPfx::Femto => "f",
        };
        write!(f, "{}", text)
    }
}
