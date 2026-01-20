#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use super::base_event_data::BaseEventData;
use super::canvas_update::CanvasUpdate;
use super::end_edit_event::EndEditEvent;
use super::graphic::Graphic;
use super::on_change_event::OnChangeEvent;
use super::on_validate_input::OnValidateInput;
use super::pointer_event_data::PointerEventData;
use super::submit_event::SubmitEvent;
use super::text::Text;
use crate::core_module::{Color, TouchScreenKeyboard, TouchScreenKeyboardType};
use crate::imgui_module::Event;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Selectable, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "InputField", namespace = "UnityEngine.UI", inherit = "Selectable,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct InputField(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentType {
    #[default]
    Standard = 0,
    Autocorrected = 1,
    IntegerNumber = 2,
    DecimalNumber = 3,
    Alphanumeric = 4,
    Name = 5,
    EmailAddress = 6,
    Password = 7,
    Pin = 8,
    Custom = 9,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Standard = 0,
    AutoCorrect = 1,
    Password = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharacterValidation {
    #[default]
    None = 0,
    Integer = 1,
    Decimal = 2,
    Alphanumeric = 3,
    Name = 4,
    EmailAddress = 5,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineType {
    #[default]
    SingleLine = 0,
    MultiLineSubmit = 1,
    MultiLineNewline = 2,
}

#[unity_impl]
impl InputField {
    #[unity_method(name = "get_shouldHideMobileInput")]
    pub fn get_should_hide_mobile_input(&self) -> bool {}

    #[unity_method(name = "set_shouldHideMobileInput")]
    pub fn set_should_hide_mobile_input(&self, value: bool) {}

    #[unity_method(name = "get_shouldActivateOnSelect")]
    pub fn get_should_activate_on_select(&self) -> bool {}

    #[unity_method(name = "set_shouldActivateOnSelect")]
    pub fn set_should_activate_on_select(&self, value: bool) {}

    #[unity_method(name = "get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_text")]
    pub fn set_text(&self, value: &str) {}

    #[unity_method(name = "get_isFocused")]
    pub fn get_is_focused(&self) -> bool {}

    #[unity_method(name = "get_caretBlinkRate")]
    pub fn get_caret_blink_rate(&self) -> f32 {}

    #[unity_method(name = "set_caretBlinkRate")]
    pub fn set_caret_blink_rate(&self, value: f32) {}

    #[unity_method(name = "get_caretWidth")]
    pub fn get_caret_width(&self) -> i32 {}

    #[unity_method(name = "set_caretWidth")]
    pub fn set_caret_width(&self, value: i32) {}

    #[unity_method(name = "get_textComponent")]
    pub fn get_text_component(&self) -> Option<Text> {}

    #[unity_method(name = "set_textComponent")]
    pub fn set_text_component(&self, value: Option<Text>) {}

    #[unity_method(name = "get_placeholder")]
    pub fn get_placeholder(&self) -> Option<Graphic> {}

    #[unity_method(name = "set_placeholder")]
    pub fn set_placeholder(&self, value: Option<Graphic>) {}

    #[unity_method(name = "get_caretColor")]
    pub fn get_caret_color(&self) -> Color {}

    #[unity_method(name = "set_caretColor")]
    pub fn set_caret_color(&self, value: Color) {}

    #[unity_method(name = "get_customCaretColor")]
    pub fn get_custom_caret_color(&self) -> bool {}

    #[unity_method(name = "set_customCaretColor")]
    pub fn set_custom_caret_color(&self, value: bool) {}

    #[unity_method(name = "get_selectionColor")]
    pub fn get_selection_color(&self) -> Color {}

    #[unity_method(name = "set_selectionColor")]
    pub fn set_selection_color(&self, value: Color) {}

    #[unity_method(name = "get_onEndEdit")]
    pub fn get_on_end_edit(&self) -> Option<EndEditEvent> {}

    #[unity_method(name = "set_onEndEdit")]
    pub fn set_on_end_edit(&self, value: Option<EndEditEvent>) {}

    #[unity_method(name = "get_onSubmit")]
    pub fn get_on_submit(&self) -> Option<SubmitEvent> {}

    #[unity_method(name = "set_onSubmit")]
    pub fn set_on_submit(&self, value: Option<SubmitEvent>) {}

    #[unity_method(name = "get_onValueChange")]
    pub fn get_on_value_change(&self) -> Option<OnChangeEvent> {}

    #[unity_method(name = "set_onValueChange")]
    pub fn set_on_value_change(&self, value: Option<OnChangeEvent>) {}

    #[unity_method(name = "get_onValueChanged")]
    pub fn get_on_value_changed(&self) -> Option<OnChangeEvent> {}

    #[unity_method(name = "set_onValueChanged")]
    pub fn set_on_value_changed(&self, value: Option<OnChangeEvent>) {}

    #[unity_method(name = "get_onValidateInput")]
    pub fn get_on_validate_input(&self) -> Option<OnValidateInput> {}

    #[unity_method(name = "set_onValidateInput")]
    pub fn set_on_validate_input(&self, value: Option<OnValidateInput>) {}

    #[unity_method(name = "get_characterLimit")]
    pub fn get_character_limit(&self) -> i32 {}

    #[unity_method(name = "set_characterLimit")]
    pub fn set_character_limit(&self, value: i32) {}

    #[unity_method(name = "get_contentType")]
    pub fn get_content_type(&self) -> ContentType {}

    #[unity_method(name = "set_contentType")]
    pub fn set_content_type(&self, value: ContentType) {}

    #[unity_method(name = "get_lineType")]
    pub fn get_line_type(&self) -> LineType {}

    #[unity_method(name = "set_lineType")]
    pub fn set_line_type(&self, value: LineType) {}

    #[unity_method(name = "get_inputType")]
    pub fn get_input_type(&self) -> InputType {}

    #[unity_method(name = "set_inputType")]
    pub fn set_input_type(&self, value: InputType) {}

    #[unity_method(name = "get_touchScreenKeyboard")]
    pub fn get_touch_screen_keyboard(&self) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "get_keyboardType")]
    pub fn get_keyboard_type(&self) -> TouchScreenKeyboardType {}

    #[unity_method(name = "set_keyboardType")]
    pub fn set_keyboard_type(&self, value: TouchScreenKeyboardType) {}

    #[unity_method(name = "get_characterValidation")]
    pub fn get_character_validation(&self) -> CharacterValidation {}

    #[unity_method(name = "set_characterValidation")]
    pub fn set_character_validation(&self, value: CharacterValidation) {}

    #[unity_method(name = "get_readOnly")]
    pub fn get_read_only(&self) -> bool {}

    #[unity_method(name = "set_readOnly")]
    pub fn set_read_only(&self, value: bool) {}

    #[unity_method(name = "get_multiLine")]
    pub fn get_multi_line(&self) -> bool {}

    #[unity_method(name = "get_asteriskChar")]
    pub fn get_asterisk_char(&self) -> u16 {}

    #[unity_method(name = "set_asteriskChar")]
    pub fn set_asterisk_char(&self, value: u16) {}

    #[unity_method(name = "get_wasCanceled")]
    pub fn get_was_canceled(&self) -> bool {}

    #[unity_method(name = "get_caretSelectPosition")]
    pub fn get_caret_select_position(&self) -> i32 {}

    #[unity_method(name = "set_caretSelectPosition")]
    pub fn set_caret_select_position(&self, value: i32) {}

    #[unity_method(name = "get_caretPosition")]
    pub fn get_caret_position(&self) -> i32 {}

    #[unity_method(name = "set_caretPosition")]
    pub fn set_caret_position(&self, value: i32) {}

    #[unity_method(name = "get_selectionAnchorPosition")]
    pub fn get_selection_anchor_position(&self) -> i32 {}

    #[unity_method(name = "set_selectionAnchorPosition")]
    pub fn set_selection_anchor_position(&self, value: i32) {}

    #[unity_method(name = "get_selectionFocusPosition")]
    pub fn get_selection_focus_position(&self) -> i32 {}

    #[unity_method(name = "set_selectionFocusPosition")]
    pub fn set_selection_focus_position(&self, value: i32) {}

    #[unity_method(name = "get_minWidth")]
    pub fn get_min_width(&self) -> f32 {}

    #[unity_method(name = "get_preferredWidth")]
    pub fn get_preferred_width(&self) -> f32 {}

    #[unity_method(name = "get_flexibleWidth")]
    pub fn get_flexible_width(&self) -> f32 {}

    #[unity_method(name = "get_minHeight")]
    pub fn get_min_height(&self) -> f32 {}

    #[unity_method(name = "get_preferredHeight")]
    pub fn get_preferred_height(&self) -> f32 {}

    #[unity_method(name = "get_flexibleHeight")]
    pub fn get_flexible_height(&self) -> f32 {}

    #[unity_method(name = "get_layoutPriority")]
    pub fn get_layout_priority(&self) -> i32 {}

    #[unity_method(name = "SetTextWithoutNotify")]
    pub fn set_text_without_notify(&self, input: &str) {}

    #[unity_method(name = "MoveTextEnd")]
    pub fn move_text_end(&self, shift: bool) {}

    #[unity_method(name = "MoveTextStart")]
    pub fn move_text_start(&self, shift: bool) {}

    #[unity_method(name = "ScreenToLocal")]
    pub fn screen_to_local(&self, screen: Vector2) -> Vector2 {}

    #[unity_method(name = "OnBeginDrag")]
    pub fn on_begin_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnDrag")]
    pub fn on_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnEndDrag")]
    pub fn on_end_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerDown")]
    pub fn on_pointer_down(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "ProcessEvent")]
    pub fn process_event(&self, e: Option<Event>) {}

    #[unity_method(name = "OnUpdateSelected")]
    pub fn on_update_selected(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "ForceLabelUpdate")]
    pub fn force_label_update(&self) {}

    #[unity_method(name = "Rebuild")]
    pub fn rebuild(&self, update: CanvasUpdate) {}

    #[unity_method(name = "LayoutComplete")]
    pub fn layout_complete(&self) {}

    #[unity_method(name = "GraphicUpdateComplete")]
    pub fn graphic_update_complete(&self) {}

    #[unity_method(name = "ActivateInputField")]
    pub fn activate_input_field(&self) {}

    #[unity_method(name = "OnSelect")]
    pub fn on_select(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnPointerClick")]
    pub fn on_pointer_click(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "DeactivateInputField")]
    pub fn deactivate_input_field(&self) {}

    #[unity_method(name = "OnDeselect")]
    pub fn on_deselect(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnSubmit")]
    pub fn on_submit(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

}
