use std::{ops::{SubAssign, Sub, Add, AddAssign, Neg, Mul}, cmp::Ordering};

use bigdecimal::{Zero, BigDecimal};
use diesel_async::AsyncConnection;
// Re-export so that macros work.
pub use serde;
pub use diesel;
use diesel::{
    sql_types::{
        Numeric,
        BigInt,
    },
    pg::Pg,
    AsExpression,
    FromSqlRow,
};
use serde::{Deserialize, Serialize};

pub mod error;
pub mod wrap;

pub mod ext;

pub mod id;
pub mod values;

use crate::error::{
    NumericU64Error,
    NumericU32Error,
};

wrap_type! {
    #[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
    PgU64<Pg>(Numeric > bigdecimal::BigDecimal > u64) |big| {
        use bigdecimal::{
            ToPrimitive,
            Signed,
        };
        let value = if let Some(value) = big.to_u64() {
            Ok(value)
        } else if big > u64::MAX.into() {
            Err(NumericU64Error::Overflow)
        } else if big.is_negative() {
            Err(NumericU64Error::Negative)
        } else if !big.is_integer() {
            Err(NumericU64Error::Decimal)
        } else {
            Err(NumericU64Error::Unknown)?
        }?;
        value
    } |b| {
        use bigdecimal::BigDecimal;
        &BigDecimal::from(b)
    }
}

// This is okay, since PgU64 exactly match the domain.
impl From<u64> for PgU64 {
    fn from(u: u64) -> Self {
        Self(u)
    }
}

wrap_type! {
    #[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
    PgU32<Pg>(BigInt > i64 > u32)
    |value| {
        if value > u32::MAX.into() {
            return Err(NumericU32Error::Overflow.into());
        } else if value.is_negative() {
            return Err(NumericU32Error::Negative.into());
        }
        value as u32
    }
    |b| {
        &(b as i64)
    }
}

// This is okay, since PgU32 exactly match the domain.
impl From<u32> for PgU32 {
    fn from(u: u32) -> Self {
        Self(u)
    }
}

#[derive(Debug, Default, Copy, Clone, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Numeric)]
pub struct SignedU64 {
    pub total: u64,
    pub is_negative: bool,
}

impl SubAssign for SignedU64 {
    fn sub_assign(&mut self, other: Self) {
        *self += -other;
    }
}

impl Sub for SignedU64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Add for SignedU64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut cloned = self.clone();
        cloned += other;
        cloned
    }
}

impl AddAssign for SignedU64 {
    fn add_assign(&mut self, other: Self) {
        if self.is_negative == other.is_negative {
            self.total += other.total;
        } else {
            if self.total >= other.total {
                self.total -= other.total;
            } else {
                self.total = other.total - self.total;
                self.is_negative = other.is_negative;
            }
        }
    }
}

impl Neg for SignedU64 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            total: self.total,
            is_negative: !self.is_negative,
        }
    }
}

impl Zero for SignedU64 {
    fn zero() -> Self {
        Self {
            total: 0,
            is_negative: false,
        }
    }

    fn is_zero(&self) -> bool {
        self.total == 0
    }
}

impl Ord for SignedU64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.total == 0 && other.total == 0 {
            return Ordering::Equal;
        }

        match (self.is_negative, other.is_negative) {
            (true, false) => {
                Ordering::Less
            },
            (true, true) => {
                other.total.cmp(&self.total)
            },
            (false, true) => {
                Ordering::Greater
            },
            (false, false) => {
                self.total.cmp(&other.total)
            },
        }
    }
}

impl PartialOrd for SignedU64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SignedU64 {
    fn eq(&self, other: &Self) -> bool {
        if self.total == 0 && other.total == 0 {
            return true;
        }

        self.is_negative == other.is_negative && self.total == other.total
    }
}

impl Eq for SignedU64 {}

impl Mul for SignedU64 {
    type Output = SignedU64;
    fn mul(self, other: Self) -> Self::Output {
        let is_negative = self.is_negative ^ other.is_negative;
        let total = self.total * other.total;
        Self {
            total,
            is_negative,
        }
    }
}

impl Mul<u64> for SignedU64 {
    type Output = SignedU64;
    fn mul(self, other: u64) -> Self::Output {
        let total = self.total * other;
        Self {
            total,
            is_negative: self.is_negative,
        }
    }
}

impl_sql_convert!(
    <Pg>
    Numeric > BigDecimal > SignedU64
    |big| {
        use bigdecimal::{
            ToPrimitive,
            Signed,
        };
        let value = if let Some(value) = big.abs().to_u64() {
            if big.is_negative() {
                Ok(SignedU64 {
                    is_negative: true,
                    total: value,
                })
            } else {
                Ok(SignedU64 {
                    is_negative: false,
                    total: value,
                })
            }
        } else if big > u64::MAX.into() {
            Err(NumericU64Error::Overflow)
        } else if !big.is_integer() {
            Err(NumericU64Error::Decimal)
        } else {
            Err(NumericU64Error::Unknown)?
        }?;
        value
    }
    |v| {
        if v.is_negative {
            BigDecimal::from(v.total).neg()
        } else {
            BigDecimal::from(v.total)
        }
    }
);

impl From<u64> for SignedU64 {
    fn from(v: u64) -> Self {
        Self {
            total: v,
            is_negative: false,
        }
    }
}


pub trait PgC: AsyncConnection<Backend = Pg> + 'static {}
impl<C:  AsyncConnection<Backend = Pg> + 'static> PgC for C {}

#[cfg(test)]
mod test {
    mod signed_u64 {
        mod ord_and_eq {
            use crate::SignedU64;

            const fn test_data() -> [SignedU64; 6] {
                [SignedU64 {
                    total: 0,
                    is_negative: false,
                },
                SignedU64 {
                    total: 0,
                    is_negative: true,
                },
                SignedU64 {
                    total: 1,
                    is_negative: false,
                },
                SignedU64 {
                    total: 1,
                    is_negative: true,
                },
                SignedU64 {
                    total: 4,
                    is_negative: false,
                },
                SignedU64 {
                    total: 4,
                    is_negative: true,
                }]
            }

            macro_rules! test_single_sign {
                ($val:expr, $sign:tt) => {
                    let [pos_0, neg_0, pos_1, neg_1, pos_4, neg_4] = $val;

                    assert_eq!(pos_0 $sign pos_0,  0 $sign  0);
                    assert_eq!(pos_0 $sign neg_0,  0 $sign -0);
                    assert_eq!(pos_0 $sign pos_1,  0 $sign  1);
                    assert_eq!(pos_0 $sign neg_1,  0 $sign -1);
                    assert_eq!(pos_0 $sign pos_4,  0 $sign  4);
                    assert_eq!(pos_0 $sign neg_4,  0 $sign -4);

                    assert_eq!(neg_0 $sign pos_0, -0 $sign  0);
                    assert_eq!(neg_0 $sign neg_0, -0 $sign -0);
                    assert_eq!(neg_0 $sign pos_1, -0 $sign  1);
                    assert_eq!(neg_0 $sign neg_1, -0 $sign -1);
                    assert_eq!(neg_0 $sign pos_4, -0 $sign  4);
                    assert_eq!(neg_0 $sign neg_4, -0 $sign -4);

                    assert_eq!(pos_1 $sign pos_0,  1 $sign  0);
                    assert_eq!(pos_1 $sign neg_0,  1 $sign -0);
                    assert_eq!(pos_1 $sign pos_1,  1 $sign  1);
                    assert_eq!(pos_1 $sign neg_1,  1 $sign -1);
                    assert_eq!(pos_1 $sign pos_4,  1 $sign  4);
                    assert_eq!(pos_1 $sign neg_4,  1 $sign -4);

                    assert_eq!(neg_1 $sign pos_0, -1 $sign  0);
                    assert_eq!(neg_1 $sign neg_0, -1 $sign -0);
                    assert_eq!(neg_1 $sign pos_1, -1 $sign  1);
                    assert_eq!(neg_1 $sign neg_1, -1 $sign -1);
                    assert_eq!(neg_1 $sign pos_4, -1 $sign  4);
                    assert_eq!(neg_1 $sign neg_4, -1 $sign -4);

                    assert_eq!(pos_4 $sign pos_0,  4 $sign  0);
                    assert_eq!(pos_4 $sign neg_0,  4 $sign -0);
                    assert_eq!(pos_4 $sign pos_1,  4 $sign  1);
                    assert_eq!(pos_4 $sign neg_1,  4 $sign -1);
                    assert_eq!(pos_4 $sign pos_4,  4 $sign  4);
                    assert_eq!(pos_4 $sign neg_4,  4 $sign -4);

                    assert_eq!(neg_4 $sign pos_0, -4 $sign  0);
                    assert_eq!(neg_4 $sign neg_0, -4 $sign -0);
                    assert_eq!(neg_4 $sign pos_1, -4 $sign  1);
                    assert_eq!(neg_4 $sign neg_1, -4 $sign -1);
                    assert_eq!(neg_4 $sign pos_4, -4 $sign  4);
                    assert_eq!(neg_4 $sign neg_4, -4 $sign -4);
                };
            }

            #[test]
            fn eq() {
                test_single_sign!(test_data(), ==);
            }
            #[test]
            fn ne() {
                test_single_sign!(test_data(), !=);
            }
            #[test]
            fn gt() {
                test_single_sign!(test_data(), >);
            }
            #[test]
            fn lt() {
                test_single_sign!(test_data(), <);
            }
            #[test]
            fn ge() {
                test_single_sign!(test_data(), >=);
            }
            #[test]
            fn le() {
                test_single_sign!(test_data(), <=);
            }
        }
    }
}
