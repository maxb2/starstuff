use pyo3::prelude::*;
use starstuff_types::angle::Angle;
use starstuff_types::catalog::osbsc::OSBSCStar;
use starstuff_types::coord::{
    Cartesian, Declination, Equitorial, Horizontal, Polar, RightAscension,
};

/**
Kinds of star coordinates.
> TODO: simplify this enum like crate::angle::Angle
*/
#[derive(Debug, Copy, Clone)]
pub enum StarCoordinates {
    Cartesian(Cartesian),
    Equitorial(Equitorial),
    Horizontal(Horizontal),
    Stereo(Polar),
}

impl From<StarCoordinates> for Cartesian {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Cartesian(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Cartesian!"),
        }
    }
}

impl From<StarCoordinates> for Equitorial {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Equitorial(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Equitorial!"),
        }
    }
}

impl From<StarCoordinates> for Horizontal {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Horizontal(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Horizontal!"),
        }
    }
}

impl From<StarCoordinates> for Polar {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Stereo(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Polar!"),
        }
    }
}

/**
Generic star structure.
> TODO: move this to a different crate and implement `from` traits for the catalog star structs in this crate.
 */
#[pyclass]
#[derive(Debug, Clone)]
pub struct Star {
    pub coordinates: StarCoordinates,
    #[pyo3(get, set)]
    pub name: Option<String>,
    #[pyo3(get, set)]
    pub id: usize,
}

impl TryFrom<OSBSCStar> for Star {
    type Error = ();

    fn try_from(ostar: OSBSCStar) -> Result<Self, Self::Error> {
        match (
            ostar.Hipparcos_id,
            ostar.right_ascension_rad,
            ostar.declination_rad,
        ) {
            (Some(id), Some(ra), Some(dec)) => Ok(Self {
                name: ostar.proper_name,
                id,
                coordinates: StarCoordinates::Equitorial(Equitorial {
                    right_ascension: RightAscension(Angle::Radian(ra)),
                    declination: Declination(Angle::Radian(dec)),
                }),
            }),
            _ => Err(()),
        }
    }
}

// A Star object written in Rust
#[pymethods]
impl Star {
    #[new]
    fn py_new(id: usize, ra: f64, dec: f64, name: Option<String>) -> PyResult<Self> {
        Ok(Self {
            id,
            name,
            coordinates: StarCoordinates::Equitorial(Equitorial {
                right_ascension: RightAscension(Angle::Radian(ra)),
                declination: Declination(Angle::Radian(dec)),
            }),
        })
    }

}
