//! Bounded in-memory account metadata; deliberately stores no credentials.

pub const MAX_ACCOUNTS: usize = 32;
pub const MAX_USERNAME_LEN: usize = 32;
pub const MAX_HOME_LEN: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Locked,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountError {
    Full,
    InvalidUsername,
    DuplicateUsername,
    UserIdExhausted,
    AccountNotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    uid: UserId,
    username: [u8; MAX_USERNAME_LEN],
    username_len: usize,
    home: [u8; MAX_HOME_LEN],
    home_len: usize,
    status: AccountStatus,
}

impl Account {
    pub const fn uid(&self) -> UserId {
        self.uid
    }

    pub fn username(&self) -> &str {
        core::str::from_utf8(&self.username[..self.username_len])
            .expect("validated account username")
    }

    pub fn home_directory(&self) -> &str {
        core::str::from_utf8(&self.home[..self.home_len])
            .expect("account home is generated from validated username")
    }

    pub const fn status(&self) -> AccountStatus {
        self.status
    }
}

pub struct AccountDatabase {
    accounts: [Option<Account>; MAX_ACCOUNTS],
    count: usize,
    next_uid: u32,
}

impl AccountDatabase {
    pub const fn new() -> Self {
        Self {
            accounts: [None; MAX_ACCOUNTS],
            count: 0,
            next_uid: 1000,
        }
    }

    pub fn create_account(&mut self, username: &str) -> Result<UserId, AccountError> {
        if !valid_username(username) {
            return Err(AccountError::InvalidUsername);
        }
        if self.find_by_username(username).is_some() {
            return Err(AccountError::DuplicateUsername);
        }
        let slot = self
            .accounts
            .iter()
            .position(Option::is_none)
            .ok_or(AccountError::Full)?;
        let next_uid = self
            .next_uid
            .checked_add(1)
            .ok_or(AccountError::UserIdExhausted)?;
        let mut account = Account {
            uid: UserId(self.next_uid),
            username: [0; MAX_USERNAME_LEN],
            username_len: username.len(),
            home: [0; MAX_HOME_LEN],
            home_len: 6 + username.len(),
            status: AccountStatus::Active,
        };
        account.username[..username.len()].copy_from_slice(username.as_bytes());
        account.home[..6].copy_from_slice(b"/home/");
        account.home[6..account.home_len].copy_from_slice(username.as_bytes());
        self.accounts[slot] = Some(account);
        self.next_uid = next_uid;
        self.count += 1;
        Ok(account.uid)
    }

    pub fn find_by_uid(&self, uid: UserId) -> Option<&Account> {
        self.accounts
            .iter()
            .flatten()
            .find(|account| account.uid == uid)
    }

    pub fn find_by_username(&self, username: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .flatten()
            .find(|account| account.username() == username)
    }

    pub fn set_status(&mut self, uid: UserId, status: AccountStatus) -> Result<(), AccountError> {
        let account = self
            .accounts
            .iter_mut()
            .flatten()
            .find(|account| account.uid == uid)
            .ok_or(AccountError::AccountNotFound)?;
        account.status = status;
        Ok(())
    }

    pub const fn len(&self) -> usize {
        self.count
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl Default for AccountDatabase {
    fn default() -> Self {
        Self::new()
    }
}

fn valid_username(username: &str) -> bool {
    !username.is_empty()
        && username.len() <= MAX_USERNAME_LEN
        && username.as_bytes()[0].is_ascii_lowercase()
        && username.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
        })
}
