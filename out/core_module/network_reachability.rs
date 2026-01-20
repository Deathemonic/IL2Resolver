#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetworkReachability {
    #[default]
    NotReachable = 0,
    ReachableViaCarrierDataNetwork = 1,
    ReachableViaLocalAreaNetwork = 2,
}
