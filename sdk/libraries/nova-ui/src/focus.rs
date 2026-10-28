//! Deterministic keyboard focus traversal for small application forms.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusError {
    DuplicateWidget,
    CapacityReached,
}

pub const MAX_FOCUSABLE_WIDGETS: usize = 64;

pub struct FocusManager {
    widgets: [Option<WidgetId>; MAX_FOCUSABLE_WIDGETS],
    len: usize,
    focused: Option<usize>,
}

impl FocusManager {
    pub const fn new() -> Self {
        Self {
            widgets: [None; MAX_FOCUSABLE_WIDGETS],
            len: 0,
            focused: None,
        }
    }

    pub fn register(&mut self, id: WidgetId) -> Result<(), FocusError> {
        if self.widgets[..self.len].contains(&Some(id)) {
            return Err(FocusError::DuplicateWidget);
        }
        if self.len == MAX_FOCUSABLE_WIDGETS {
            return Err(FocusError::CapacityReached);
        }
        self.widgets[self.len] = Some(id);
        self.len += 1;
        if self.focused.is_none() {
            self.focused = Some(0);
        }
        Ok(())
    }

    pub fn remove(&mut self, id: WidgetId) -> bool {
        let Some(index) = self.widgets[..self.len]
            .iter()
            .position(|widget| *widget == Some(id))
        else {
            return false;
        };
        for current in index..self.len - 1 {
            self.widgets[current] = self.widgets[current + 1];
        }
        self.len -= 1;
        self.widgets[self.len] = None;
        self.focused = match self.focused {
            Some(_) if self.len == 0 => None,
            Some(focused) if focused > index => Some(focused - 1),
            Some(focused) if focused == index => Some(index.min(self.len - 1)),
            other => other,
        };
        true
    }

    pub fn focus(&mut self, id: WidgetId) -> bool {
        let Some(index) = self.widgets[..self.len]
            .iter()
            .position(|widget| *widget == Some(id))
        else {
            return false;
        };
        self.focused = Some(index);
        true
    }

    pub fn focus_next(&mut self) -> Option<WidgetId> {
        if self.len == 0 {
            self.focused = None;
            return None;
        }
        self.focused = Some(self.focused.map_or(0, |index| (index + 1) % self.len));
        self.focused_id()
    }

    pub fn focus_previous(&mut self) -> Option<WidgetId> {
        if self.len == 0 {
            self.focused = None;
            return None;
        }
        self.focused = Some(
            self.focused
                .map_or(self.len - 1, |index| (index + self.len - 1) % self.len),
        );
        self.focused_id()
    }

    pub fn focused_id(&self) -> Option<WidgetId> {
        self.focused
            .and_then(|index| self.widgets.get(index).copied().flatten())
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}
