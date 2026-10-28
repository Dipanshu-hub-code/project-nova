//! Fixed-size notification queue with bounded text and FIFO behavior.

pub const NOTIFICATION_CAPACITY: usize = 16;
pub const MAX_TITLE_BYTES: usize = 64;
pub const MAX_BODY_BYTES: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationError {
    TextTooLong,
    QueueFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Notification {
    pub app_id: u32,
    title: [u8; MAX_TITLE_BYTES],
    title_len: usize,
    body: [u8; MAX_BODY_BYTES],
    body_len: usize,
}

impl Notification {
    pub fn new(app_id: u32, title: &str, body: &str) -> Result<Self, NotificationError> {
        if title.len() > MAX_TITLE_BYTES || body.len() > MAX_BODY_BYTES {
            return Err(NotificationError::TextTooLong);
        }
        let mut notification = Self {
            app_id,
            title: [0; MAX_TITLE_BYTES],
            title_len: title.len(),
            body: [0; MAX_BODY_BYTES],
            body_len: body.len(),
        };
        notification.title[..title.len()].copy_from_slice(title.as_bytes());
        notification.body[..body.len()].copy_from_slice(body.as_bytes());
        Ok(notification)
    }

    pub fn title(&self) -> &str {
        core::str::from_utf8(&self.title[..self.title_len])
            .expect("notification title is stored from valid UTF-8")
    }

    pub fn body(&self) -> &str {
        core::str::from_utf8(&self.body[..self.body_len])
            .expect("notification body is stored from valid UTF-8")
    }

    pub const fn app_id(&self) -> u32 {
        self.app_id
    }
}

pub struct NotificationService {
    queue: [Option<Notification>; NOTIFICATION_CAPACITY],
    head: usize,
    len: usize,
}

impl NotificationService {
    pub const fn new() -> Self {
        Self {
            queue: [None; NOTIFICATION_CAPACITY],
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, notification: Notification) -> Result<(), NotificationError> {
        if self.len == NOTIFICATION_CAPACITY {
            return Err(NotificationError::QueueFull);
        }
        let tail = (self.head + self.len) % NOTIFICATION_CAPACITY;
        self.queue[tail] = Some(notification);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Notification> {
        if self.len == 0 {
            return None;
        }
        let notification = self.queue[self.head].take();
        self.head = (self.head + 1) % NOTIFICATION_CAPACITY;
        self.len -= 1;
        notification
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}
