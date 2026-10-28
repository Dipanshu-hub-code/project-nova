//! Host-buildable UI model. Rendering requires a Nova window syscall that
//! the current kernel does not provide.

pub mod button;
pub mod focus;
pub mod label;
pub mod layout;
pub mod text_field;
pub mod theme;
pub mod window;

pub use nova_std::NovaError;

pub fn init() -> Result<(), NovaError> {
    Err(NovaError::NotSupported)
}

#[cfg(test)]
mod tests {
    use super::{
        button::{ActivationKey, Button, ButtonVariant},
        focus::{FocusError, FocusManager, WidgetId},
        label::Label,
        layout::{Insets, Size, StackLayout},
        text_field::{TextField, TextFieldError},
        theme::{CornerRadius, Spacing, ThemeMode},
        window::Window,
        NovaError,
    };

    #[test]
    fn widgets_validate_text_and_keep_metadata() {
        assert_eq!(Button::new(""), Err(NovaError::InvalidArgument));
        assert_eq!(Button::new("   "), Err(NovaError::InvalidArgument));
        assert_eq!(Button::new("Open").unwrap().label(), "Open");
        assert_eq!(Label::new("Welcome").unwrap().text(), "Welcome");
        assert_eq!(Window::new("Nova").unwrap().title(), "Nova");
        assert_eq!(Window::new(" ").err(), Some(NovaError::InvalidArgument));
    }

    #[test]
    fn window_creation_metadata_does_not_claim_display_support() {
        let window = Window::new("Preview window").unwrap();
        assert_eq!(window.show(), Err(NovaError::NotSupported));
        assert_eq!(super::init(), Err(NovaError::NotSupported));
    }

    #[test]
    fn themes_expose_consistent_light_and_dark_design_tokens() {
        let light = ThemeMode::Light.palette();
        let dark = ThemeMode::Dark.palette();
        assert_ne!(light.background, dark.background);
        assert_ne!(light.text, dark.text);
        assert_ne!(light.accent, dark.accent);
        assert_eq!(Spacing::SMALL, 8);
        assert_eq!(CornerRadius::MEDIUM, 10);
    }

    #[test]
    fn focus_manager_traverses_and_removes_widgets_deterministically() {
        let mut focus = FocusManager::new();
        let first = WidgetId(1);
        let second = WidgetId(2);
        focus.register(first).unwrap();
        focus.register(second).unwrap();
        assert_eq!(focus.focused_id(), Some(first));
        assert_eq!(focus.register(first), Err(FocusError::DuplicateWidget));
        assert_eq!(focus.focus_next(), Some(second));
        assert_eq!(focus.focus_next(), Some(first));
        assert_eq!(focus.focus_previous(), Some(second));
        assert!(focus.remove(second));
        assert_eq!(focus.focused_id(), Some(first));
        assert!(focus.remove(first));
        assert_eq!(focus.focused_id(), None);
        assert_eq!(focus.focus_next(), None);
    }

    #[test]
    fn text_field_edits_utf8_without_invalid_cursor_boundaries() {
        let mut field = TextField::with_value("Search files", "café", 32).unwrap();
        assert_eq!(field.accessible_label(), "Search files");
        field.move_left();
        assert_eq!(field.cursor_byte_offset(), 3);
        field.insert("x").unwrap();
        assert_eq!(field.value(), "cafxé");
        assert!(field.backspace());
        assert_eq!(field.value(), "café");
        field.move_home();
        assert!(field.delete_forward());
        assert_eq!(field.value(), "afé");
        field.move_end();
        assert_eq!(field.cursor_byte_offset(), field.value().len());
        assert_eq!(
            TextField::new("Search", 0).err(),
            Some(TextFieldError::InvalidMaximumLength)
        );
        assert_eq!(
            TextField::new(" ", 8).err(),
            Some(TextFieldError::MissingAccessibleLabel)
        );
        assert_eq!(
            TextField::with_value("Name", "too long", 3).err(),
            Some(TextFieldError::ValueTooLong)
        );
        assert_eq!(
            TextField::with_value("Name", "one\ntwo", 16).err(),
            Some(TextFieldError::InvalidValue)
        );
    }

    #[test]
    fn text_field_rejects_overflow_and_disabled_edits_without_mutation() {
        let mut field = TextField::with_value("Name", "abc", 4).unwrap();
        field.move_end();
        assert_eq!(field.insert("de"), Err(NovaError::InvalidArgument));
        assert_eq!(field.value(), "abc");
        field.set_enabled(false);
        assert_eq!(field.insert("x"), Err(NovaError::PermissionDenied));
        assert_eq!(field.clear(), Err(NovaError::PermissionDenied));
        assert_eq!(field.value(), "abc");
    }

    #[test]
    fn stack_layout_subtracts_padding_and_rejects_underflow() {
        let layout = StackLayout::vertical(8, Insets::all(10));
        assert_eq!(
            layout.content_size(Size {
                width: 100,
                height: 80
            }),
            Some(Size {
                width: 80,
                height: 60
            })
        );
        assert_eq!(
            layout.content_size(Size {
                width: 10,
                height: 80
            }),
            None
        );
    }

    #[test]
    fn button_activation_respects_enabled_and_keyboard_focus_state() {
        let mut button = Button::new("Save").unwrap();
        button.set_variant(ButtonVariant::Secondary);
        assert_eq!(button.variant(), ButtonVariant::Secondary);
        assert!(button.click());
        assert!(!button.activate_by_keyboard(ActivationKey::Enter));
        button.set_focused(true);
        assert!(button.is_focused());
        assert!(button.activate_by_keyboard(ActivationKey::Enter));
        assert!(button.activate_by_keyboard(ActivationKey::Space));
        assert!(!button.activate_by_keyboard(ActivationKey::Other));
        button.set_enabled(false);
        assert!(!button.is_focused());
        assert!(!button.click());
        assert!(!button.activate_by_keyboard(ActivationKey::Enter));
    }
}
