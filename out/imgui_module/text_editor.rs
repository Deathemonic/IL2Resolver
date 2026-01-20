#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use super::event::Event;
use super::gui_content::GUIContent;
use super::gui_style::GUIStyle;
use crate::core_module::{Rect, TouchScreenKeyboard};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "TextEditor", namespace = "UnityEngine")]
pub struct TextEditor(pub *mut c_void);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DblClickSnapping {
    #[default]
    WORDS = 0,
    PARAGRAPHS = 1,
}

#[unity_impl]
impl TextEditor {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_content")]
    pub fn get_content(&self) -> Option<GUIContent> {}

    #[unity_method(name = "set_content")]
    pub fn set_content(&self, value: Option<GUIContent>) {}

    #[unity_method(name = "get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_text")]
    pub fn set_text(&self, value: &str) {}

    #[unity_method(name = "get_position")]
    pub fn get_position(&self) -> Rect {}

    #[unity_method(name = "set_position")]
    pub fn set_position(&self, value: Rect) {}

    #[unity_method(name = "get_cursorIndex")]
    pub fn get_cursor_index(&self) -> i32 {}

    #[unity_method(name = "set_cursorIndex")]
    pub fn set_cursor_index(&self, value: i32) {}

    #[unity_method(name = "get_selectIndex")]
    pub fn get_select_index(&self) -> i32 {}

    #[unity_method(name = "set_selectIndex")]
    pub fn set_select_index(&self, value: i32) {}

    #[unity_method(name = "get_doubleClickSnapping")]
    pub fn get_double_click_snapping(&self) -> DblClickSnapping {}

    #[unity_method(name = "set_doubleClickSnapping")]
    pub fn set_double_click_snapping(&self, value: DblClickSnapping) {}

    #[unity_method(name = "get_altCursorPosition")]
    pub fn get_alt_cursor_position(&self) -> i32 {}

    #[unity_method(name = "set_altCursorPosition")]
    pub fn set_alt_cursor_position(&self, value: i32) {}

    #[unity_method(name = "get_hasSelection")]
    pub fn get_has_selection(&self) -> bool {}

    #[unity_method(name = "get_SelectedText")]
    pub fn get_selected_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "OnFocus")]
    pub fn on_focus(&self) {}

    #[unity_method(name = "OnLostFocus")]
    pub fn on_lost_focus(&self) {}

    #[unity_method(name = "HandleKeyEvent")]
    pub fn handle_key_event(&self, e: Option<Event>) -> bool {}

    #[unity_method(name = "DeleteLineBack")]
    pub fn delete_line_back(&self) -> bool {}

    #[unity_method(name = "DeleteWordBack")]
    pub fn delete_word_back(&self) -> bool {}

    #[unity_method(name = "DeleteWordForward")]
    pub fn delete_word_forward(&self) -> bool {}

    #[unity_method(name = "Delete")]
    pub fn delete(&self) -> bool {}

    #[unity_method(name = "CanPaste")]
    pub fn can_paste(&self) -> bool {}

    #[unity_method(name = "Backspace")]
    pub fn backspace(&self) -> bool {}

    #[unity_method(name = "SelectAll")]
    pub fn select_all(&self) {}

    #[unity_method(name = "SelectNone")]
    pub fn select_none(&self) {}

    #[unity_method(name = "DeleteSelection")]
    pub fn delete_selection(&self) -> bool {}

    #[unity_method(name = "ReplaceSelection")]
    pub fn replace_selection(&self, replace: &str) {}

    #[unity_method(name = "Insert")]
    pub fn insert(&self, c: u16) {}

    #[unity_method(name = "MoveSelectionToAltCursor")]
    pub fn move_selection_to_alt_cursor(&self) {}

    #[unity_method(name = "MoveRight")]
    pub fn move_right(&self) {}

    #[unity_method(name = "MoveLeft")]
    pub fn move_left(&self) {}

    #[unity_method(name = "MoveUp")]
    pub fn move_up(&self) {}

    #[unity_method(name = "MoveDown")]
    pub fn move_down(&self) {}

    #[unity_method(name = "MoveLineStart")]
    pub fn move_line_start(&self) {}

    #[unity_method(name = "MoveLineEnd")]
    pub fn move_line_end(&self) {}

    #[unity_method(name = "MoveGraphicalLineStart")]
    pub fn move_graphical_line_start(&self) {}

    #[unity_method(name = "MoveGraphicalLineEnd")]
    pub fn move_graphical_line_end(&self) {}

    #[unity_method(name = "MoveTextStart")]
    pub fn move_text_start(&self) {}

    #[unity_method(name = "MoveTextEnd")]
    pub fn move_text_end(&self) {}

    #[unity_method(name = "MoveParagraphForward")]
    pub fn move_paragraph_forward(&self) {}

    #[unity_method(name = "MoveParagraphBackward")]
    pub fn move_paragraph_backward(&self) {}

    #[unity_method(name = "MoveCursorToPosition")]
    pub fn move_cursor_to_position(&self, cursor_position: Vector2) {}

    #[unity_method(name = "MoveAltCursorToPosition")]
    pub fn move_alt_cursor_to_position(&self, cursor_position: Vector2) {}

    #[unity_method(name = "IsOverSelection")]
    pub fn is_over_selection(&self, cursor_position: Vector2) -> bool {}

    #[unity_method(name = "SelectToPosition")]
    pub fn select_to_position(&self, cursor_position: Vector2) {}

    #[unity_method(name = "SelectLeft")]
    pub fn select_left(&self) {}

    #[unity_method(name = "SelectRight")]
    pub fn select_right(&self) {}

    #[unity_method(name = "SelectUp")]
    pub fn select_up(&self) {}

    #[unity_method(name = "SelectDown")]
    pub fn select_down(&self) {}

    #[unity_method(name = "SelectTextEnd")]
    pub fn select_text_end(&self) {}

    #[unity_method(name = "SelectTextStart")]
    pub fn select_text_start(&self) {}

    #[unity_method(name = "MouseDragSelectsWholeWords")]
    pub fn mouse_drag_selects_whole_words(&self, on: bool) {}

    #[unity_method(name = "DblClickSnap")]
    pub fn dbl_click_snap(&self, snapping: DblClickSnapping) {}

    #[unity_method(name = "MoveWordRight")]
    pub fn move_word_right(&self) {}

    #[unity_method(name = "MoveToStartOfNextWord")]
    pub fn move_to_start_of_next_word(&self) {}

    #[unity_method(name = "MoveToEndOfPreviousWord")]
    pub fn move_to_end_of_previous_word(&self) {}

    #[unity_method(name = "SelectToStartOfNextWord")]
    pub fn select_to_start_of_next_word(&self) {}

    #[unity_method(name = "SelectToEndOfPreviousWord")]
    pub fn select_to_end_of_previous_word(&self) {}

    #[unity_method(name = "FindStartOfNextWord")]
    pub fn find_start_of_next_word(&self, p: i32) -> i32 {}

    #[unity_method(name = "MoveWordLeft")]
    pub fn move_word_left(&self) {}

    #[unity_method(name = "SelectWordRight")]
    pub fn select_word_right(&self) {}

    #[unity_method(name = "SelectWordLeft")]
    pub fn select_word_left(&self) {}

    #[unity_method(name = "ExpandSelectGraphicalLineStart")]
    pub fn expand_select_graphical_line_start(&self) {}

    #[unity_method(name = "ExpandSelectGraphicalLineEnd")]
    pub fn expand_select_graphical_line_end(&self) {}

    #[unity_method(name = "SelectGraphicalLineStart")]
    pub fn select_graphical_line_start(&self) {}

    #[unity_method(name = "SelectGraphicalLineEnd")]
    pub fn select_graphical_line_end(&self) {}

    #[unity_method(name = "SelectParagraphForward")]
    pub fn select_paragraph_forward(&self) {}

    #[unity_method(name = "SelectParagraphBackward")]
    pub fn select_paragraph_backward(&self) {}

    #[unity_method(name = "SelectCurrentWord")]
    pub fn select_current_word(&self) {}

    #[unity_method(name = "SelectCurrentParagraph")]
    pub fn select_current_paragraph(&self) {}

    #[unity_method(name = "UpdateScrollOffsetIfNeeded")]
    pub fn update_scroll_offset_if_needed(&self, evt: Option<Event>) {}

    #[unity_method(name = "DrawCursor")]
    pub fn draw_cursor(&self, new_text: &str) {}

    #[unity_method(name = "SaveBackup")]
    pub fn save_backup(&self) {}

    #[unity_method(name = "Undo")]
    pub fn undo(&self) {}

    #[unity_method(name = "Cut")]
    pub fn cut(&self) -> bool {}

    #[unity_method(name = "Copy")]
    pub fn copy(&self) {}

    #[unity_method(name = "Paste")]
    pub fn paste(&self) -> bool {}

    #[unity_method(name = "DetectFocusChange")]
    pub fn detect_focus_change(&self) {}

}
