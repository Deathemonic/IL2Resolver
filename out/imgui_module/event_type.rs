#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventType {
    #[default]
    MouseDown = 0,
    MouseUp = 1,
    MouseMove = 2,
    MouseDrag = 3,
    KeyDown = 4,
    KeyUp = 5,
    ScrollWheel = 6,
    Repaint = 7,
    Layout = 8,
    DragUpdated = 9,
    DragPerform = 10,
    DragExited = 15,
    Ignore = 11,
    Used = 12,
    ValidateCommand = 13,
    ExecuteCommand = 14,
    ContextClick = 16,
    MouseEnterWindow = 20,
    MouseLeaveWindow = 21,
    TouchDown = 30,
    TouchUp = 31,
    TouchMove = 32,
    TouchEnter = 33,
    TouchLeave = 34,
    TouchStationary = 35,
}
