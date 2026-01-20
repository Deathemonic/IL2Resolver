#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeSortingCriteria {
    #[default]
    None = 0,
    Importance = 1,
    Size = 2,
    ImportanceThenSize = 3,
}
