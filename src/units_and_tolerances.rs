use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
};

pub enum LengthUnitSystem {
    None,
    Angstroms,
    Nanometers,
    Microns,
    Millimeters,
    Centimeters,
    Decimeters,
    Meters,
    Dekameters,
    Hectometers,
    Kilometers,
    Megameters,
    Gigameters,
    Microinches,
    Mils,
    Inches,
    Feet,
    Yards,
    Miles,
    PrinterPoints,
    PrinterPicas,
    NauticalMiles,
    AstronomicalUnits,
    LigthYears,
    Parsecs,
    CustomUnits,
    Unset,
}

impl Default for LengthUnitSystem {
    fn default() -> Self {
        LengthUnitSystem::Millimeters
    }
}

impl TryInto<LengthUnitSystem> for u32 {
    type Error = Error;

    fn try_into(self) -> Result<LengthUnitSystem, Self::Error> {
        use LengthUnitSystem::*;

        match self {
            0 => Ok(None),
            12 => Ok(Angstroms),
            13 => Ok(Nanometers),
            1 => Ok(Microns),
            2 => Ok(Millimeters),
            3 => Ok(Centimeters),
            14 => Ok(Decimeters),
            4 => Ok(Meters),
            15 => Ok(Dekameters),
            16 => Ok(Hectometers),
            5 => Ok(Kilometers),
            17 => Ok(Megameters),
            18 => Ok(Gigameters),
            6 => Ok(Microinches),
            7 => Ok(Mils),
            8 => Ok(Inches),
            9 => Ok(Feet),
            19 => Ok(Yards),
            10 => Ok(Miles),
            20 => Ok(PrinterPoints),
            21 => Ok(PrinterPicas),
            22 => Ok(NauticalMiles),
            23 => Ok(AstronomicalUnits),
            24 => Ok(LigthYears),
            25 => Ok(Parsecs),
            11 => Ok(CustomUnits),
            255 => Ok(Unset),
            _ => Err(Error::Simple(ErrorKind::InvalidLengthUnitSystem)),
        }
    }
}

impl Into<u32> for LengthUnitSystem {
    fn into(self) -> u32 {
        use LengthUnitSystem::*;

        match self {
            None => 0,
            Angstroms => 12,
            Nanometers => 13,
            Microns => 1,
            Millimeters => 2,
            Centimeters => 3,
            Decimeters => 14,
            Meters => 4,
            Dekameters => 15,
            Hectometers => 16,
            Kilometers => 5,
            Megameters => 17,
            Gigameters => 18,
            Microinches => 6,
            Mils => 7,
            Inches => 8,
            Feet => 9,
            Yards => 19,
            Miles => 10,
            PrinterPoints => 20,
            PrinterPicas => 21,
            NauticalMiles => 22,
            AstronomicalUnits => 23,
            LigthYears => 24,
            Parsecs => 25,
            CustomUnits => 11,
            Unset => 255,
        }
    }
}

pub enum DistanceDisplayMode {
    Decimal,
    Fractional,
    FeetInches,
}

impl Default for DistanceDisplayMode {
    fn default() -> Self {
        DistanceDisplayMode::Decimal
    }
}

impl TryInto<DistanceDisplayMode> for u32 {
    type Error = Error;

    fn try_into(self) -> Result<DistanceDisplayMode, Self::Error> {
        use DistanceDisplayMode::*;

        match self {
            0 => Ok(Decimal),
            1 => Ok(Fractional),
            2 => Ok(FeetInches),
            _ => Err(Error::Simple(ErrorKind::InvalidDistanceDisplayMode)),
        }
    }
}

impl Into<u32> for DistanceDisplayMode {
    fn into(self) -> u32 {
        use DistanceDisplayMode::*;

        match self {
            Decimal => 0,
            Fractional => 1,
            FeetInches => 2,
        }
    }
}

pub struct CustomUnitSystem {
    pub meters_per_unit: f64,
    pub name: String,
}

pub struct RegularUnitSystem {
    pub length_unit_system: LengthUnitSystem,
}

pub enum UnitSystem {
    Custom(CustomUnitSystem),
    Regular(RegularUnitSystem),
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::Regular(RegularUnitSystem {
            length_unit_system: LengthUnitSystem::Millimeters,
        })
    }
}

#[derive(Default)]
pub struct UnitsAndTolerances {
    pub unit_system: UnitSystem,
    pub absolute_tolerance: f64,
    pub angle_tolerance: f64,
    pub relative_tolerance: f64,
    pub distance_display_mode: DistanceDisplayMode,
    pub distance_display_precission: i32,
}

impl<V> Deserialize<V> for UnitsAndTolerances
where
    V: FileVersion,
    String: Deserialize<V>,
    ErrorStack: From<<String as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let mut ret = UnitsAndTolerances::default();
        let version = deserialize!(u32, V, ostream, "version");
        if 100 <= version && 200 > version {
            let mut meters_per_unit = 1.0f64;
            let mut custom_unit_name = String::default();

            let unit_system: LengthUnitSystem =
                match deserialize!(u32, V, ostream, "unit_system").try_into() {
                    Ok(ok) => ok,
                    Err(e) => {
                        let mut ret = ErrorStack::new(e);
                        ret.push_frame("unit_system", "LengthUnitSystem");
                        return Err(ret);
                    }
                };
            ret.absolute_tolerance = deserialize!(f64, V, ostream, "absolute_tolerance");
            ret.angle_tolerance = deserialize!(f64, V, ostream, "angle_tolerance");
            ret.relative_tolerance = deserialize!(f64, V, ostream, "relative_tolerance");
            if 101 <= version {
                ret.distance_display_mode =
                    match deserialize!(u32, V, ostream, "distance_display_mode").try_into() {
                        Ok(ok) => ok,
                        Err(e) => {
                            let mut ret = ErrorStack::new(e);
                            ret.push_frame("dispance_display_mode", "DistanceDisplayMode");
                            return Err(ret);
                        }
                    };
                ret.distance_display_precission =
                    deserialize!(i32, V, ostream, "distance_display_precission");
                if 0 > ret.distance_display_precission || 20 < ret.distance_display_precission {
                    ret.distance_display_precission = 3;
                }
                if 102 <= version {
                    meters_per_unit = deserialize!(f64, V, ostream, "meters_per_unit");
                    custom_unit_name = deserialize!(String, V, ostream, "custom_unit_name");
                }
            }
            ret.unit_system = match unit_system {
                LengthUnitSystem::CustomUnits => UnitSystem::Custom(CustomUnitSystem {
                    meters_per_unit,
                    name: custom_unit_name,
                }),
                u => UnitSystem::Regular(RegularUnitSystem {
                    length_unit_system: u,
                }),
            };
        }
        Ok(ret)
    }
}
