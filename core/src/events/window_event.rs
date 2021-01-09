
use crate::state::mouse::MouseButton;

use keyboard_types::{Code, Key};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorIcon {
    Arrow,
    NResize,
    EResize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Test,
    WindowClose,
    WindowResize(f32, f32),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove(f32, f32),
    MouseScroll(f32, f32),
    MouseOver,
    MouseOut,
    CharInput(char),
    KeyDown(Code, Option<Key>),
    KeyUp(Code, Option<Key>),
    SetCursor(CursorIcon),
    MouseCaptureEvent,
    MouseCaptureOutEvent,
    Redraw,
    Restyle,
    Relayout,
}