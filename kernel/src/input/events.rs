//! Input event types shared by device drivers and the desktop.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Press(u8),
    Release(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEvent {
    Move { dx: i32, dy: i32 },
    ButtonPress(u8),
    ButtonRelease(u8),
    Scroll(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
}
