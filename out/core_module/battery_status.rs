#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BatteryStatus {
    #[default]
    Unknown = 0,
    Charging = 1,
    Discharging = 2,
    NotCharging = 3,
    Full = 4,
}
