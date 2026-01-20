#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSourceCurveType {
    #[default]
    CustomRolloff = 0,
    SpatialBlend = 1,
    ReverbZoneMix = 2,
    Spread = 3,
}
