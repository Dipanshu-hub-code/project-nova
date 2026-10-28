//! Pure access-policy context, not a kernel privilege boundary.

use crate::security::accounts::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrivilegeContext {
    uid: UserId,
    administrator: bool,
}

impl PrivilegeContext {
    pub const fn for_policy_user(uid: UserId) -> Self {
        Self {
            uid,
            administrator: false,
        }
    }

    pub const fn for_policy_administrator(uid: UserId) -> Self {
        Self {
            uid,
            administrator: true,
        }
    }

    pub const fn uid(self) -> UserId {
        self.uid
    }

    pub const fn is_administrator(self) -> bool {
        self.administrator
    }

    pub const fn can_access_user_files(self, owner: UserId) -> bool {
        self.administrator || self.uid.0 == owner.0
    }
}
