use thiserror::Error;

use diesel::result::Error as DieselError;

use treeerror::from;

pub trait DoleExtension {
    fn to_dole<L: std::error::Error>(self) -> DieselOr<L>;
}

impl DoleExtension for DieselError {
    fn to_dole<L: std::error::Error>(self) -> DieselOr<L> {
        DieselOr::Diesel(self)
    }
}

#[derive(Error, Debug)]
pub enum DieselOr<Logical> {
    // TODO Once negative impls are stabilized, explicitly prevent Logical from being DieselError
    // so that DieselOr<DieselError> is banned and From<DieselError> can only be DieselOr::Diesel.
    #[error("diesel {0}")]
    Diesel(DieselError),
    #[error("logical {0}")]
    Logical(#[from] Logical),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Error)]
pub enum NumericU64Error {
    #[error("numeric overflows")]
    Overflow,
    #[error("numeric underflows")]
    Negative,
    #[error("numeric is decimal")]
    Decimal,
    #[error("numeric is not a u64")]
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Error)]
pub enum NumericU32Error {
    #[error("numeric overflows")]
    Overflow,
    #[error("numeric underflows")]
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Error)]
pub enum LevelError {
    #[error("numeric overflows")]
    Overflow,
    #[error("numeric underflows")]
    Negative,
}
