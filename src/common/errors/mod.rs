#![allow(non_camel_case_types)]
#![allow(clippy::enum_variant_names)]

use core::fmt;

pub type Result<T> = std::result::Result<T, CError>;

/// All application error kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CError {
    // Generic
    GenericBadRequest,
    GenericInternalServer,
    GenericRequestTimedOut,
    GenericUnauthorized,
    GenericPermission,
    GenericUnknownAPIPath,
    InvalidDatabaseClient,
}

impl CError {
    pub const fn code(self) -> Option<&'static str> {
        Some(match self {
            // Generic
            CError::GenericBadRequest => "400000",
            CError::GenericUnauthorized => "400001",
            CError::GenericPermission => "400003",
            CError::GenericUnknownAPIPath => "400004",
            CError::GenericInternalServer => "500000",
            CError::GenericRequestTimedOut => "500004",
            CError::InvalidDatabaseClient => "500005",
        })
    }

    /// Exact human-readable message text from your Go error declarations.
    pub const fn message(self) -> &'static str {
        match self {
            // Generic
            CError::GenericBadRequest => "bad request error",
            CError::GenericInternalServer => "internal server error",
            CError::GenericRequestTimedOut => "request timeout error",
            CError::GenericUnauthorized => "unauthorized request error",
            CError::GenericPermission => "invalid permission error",
            CError::GenericUnknownAPIPath => "unknown api path",
            CError::InvalidDatabaseClient => "invalid database client",
        }
    }
}

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for CError {}

pub fn code_for_option(err: Option<CError>) -> &'static str {
    match err {
        None => "OK",
        Some(e) => e.code().unwrap_or("UNKNOWN"),
    }
}

pub fn message_for_option(err: Option<CError>) -> &'static str {
    match err {
        None => "OK",
        Some(e) => e.message(),
    }
}
