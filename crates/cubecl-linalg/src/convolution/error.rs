use std::fmt::Debug;

use cubecl_core::tune::AutotuneError;

use crate::matmul::kernels::{MatmulAvailabilityError, MatmulLaunchError};

pub enum ConvLaunchError {
    Matmul(MatmulLaunchError),
    Groups(usize),
    CubeCountTooLarge,
    Unknown,
}

impl Debug for ConvLaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvLaunchError::Matmul(err) => {
                write!(f, "{err:?}")
            }
            ConvLaunchError::Groups(groups) => {
                writeln!(
                    f,
                    "Unable to launch matmul because groups must be one, is actually {groups}",
                )
            }
            ConvLaunchError::CubeCountTooLarge => {
                write!(f, "The cube count is larger than the max supported.")
            }
            ConvLaunchError::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<MatmulLaunchError> for ConvLaunchError {
    fn from(value: MatmulLaunchError) -> Self {
        Self::Matmul(value)
    }
}

impl From<MatmulAvailabilityError> for ConvLaunchError {
    fn from(value: MatmulAvailabilityError) -> Self {
        Self::Matmul(MatmulLaunchError::Unavailable(value))
    }
}

#[allow(clippy::from_over_into)]
impl Into<AutotuneError> for ConvLaunchError {
    fn into(self) -> AutotuneError {
        AutotuneError::Unknown(format!("{self:?}"))
    }
}
