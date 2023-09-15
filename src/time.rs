use crate::deserialize::{Deserialize, FileVersion};
use once_3dm_macros::Deserialize;
use once_io::OStream;

#[derive(Default, Deserialize)]
pub struct Time {
    pub second: u32,
    pub minute: u32,
    pub hour: u32,
    pub month_day: u32,
    pub month: u32,
    pub year: u32,
    pub week_day: u32,
    pub year_day: u32,
}
