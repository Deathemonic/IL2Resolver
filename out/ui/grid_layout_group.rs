#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{LayoutGroup, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "GridLayoutGroup", namespace = "UnityEngine.UI", inherit = "LayoutGroup,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct GridLayoutGroup(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Corner {
    #[default]
    UpperLeft = 0,
    UpperRight = 1,
    LowerLeft = 2,
    LowerRight = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    #[default]
    Horizontal = 0,
    Vertical = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Constraint {
    #[default]
    Flexible = 0,
    FixedColumnCount = 1,
    FixedRowCount = 2,
}

#[unity_impl]
impl GridLayoutGroup {
    #[unity_method(name = "get_startCorner")]
    pub fn get_start_corner(&self) -> Corner {}

    #[unity_method(name = "set_startCorner")]
    pub fn set_start_corner(&self, value: Corner) {}

    #[unity_method(name = "get_startAxis")]
    pub fn get_start_axis(&self) -> Axis {}

    #[unity_method(name = "set_startAxis")]
    pub fn set_start_axis(&self, value: Axis) {}

    #[unity_method(name = "get_cellSize")]
    pub fn get_cell_size(&self) -> Vector2 {}

    #[unity_method(name = "set_cellSize")]
    pub fn set_cell_size(&self, value: Vector2) {}

    #[unity_method(name = "get_spacing")]
    pub fn get_spacing(&self) -> Vector2 {}

    #[unity_method(name = "set_spacing")]
    pub fn set_spacing(&self, value: Vector2) {}

    #[unity_method(name = "get_constraint")]
    pub fn get_constraint(&self) -> Constraint {}

    #[unity_method(name = "set_constraint")]
    pub fn set_constraint(&self, value: Constraint) {}

    #[unity_method(name = "get_constraintCount")]
    pub fn get_constraint_count(&self) -> i32 {}

    #[unity_method(name = "set_constraintCount")]
    pub fn set_constraint_count(&self, value: i32) {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

    #[unity_method(name = "SetLayoutHorizontal")]
    pub fn set_layout_horizontal(&self) {}

    #[unity_method(name = "SetLayoutVertical")]
    pub fn set_layout_vertical(&self) {}

}
