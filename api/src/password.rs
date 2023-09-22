use bcrypt::{BcryptError, DEFAULT_COST};
use serde::Deserialize;

use crate::from_unchecked::FromUnchecked;

pub struct Password(String);

#[derive(Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct HashedPassword(String);

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl From<HashedPassword> for String {
    fn from(value: HashedPassword) -> Self {
        value.0
    }
}

impl<'a> From<&'a HashedPassword> for &'a str {
    fn from(value: &'a HashedPassword) -> Self {
        &value.0
    }
}

impl FromUnchecked<String> for HashedPassword {
    fn from_unchecked(value: String) -> Self {
        Self(value)
    }
}

pub enum PasswordError {
    TooShort(usize),
    TooLong(usize),
    InvalidCharacters,
}

impl TryFrom<String> for Password {
    type Error = PasswordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.len();
        if len < 8 {
            return Err(Self::Error::TooShort(len));
        } else if len > 40 {
            return Err(Self::Error::TooLong(len));
        } else if !value.is_ascii() {
            return Err(Self::Error::InvalidCharacters);
        } else {
            return Ok(Password(value));
        }
    }
}

pub enum HashedPasswordError {
    BcryptError(BcryptError),
}

impl From<String> for HashedPassword {
    fn from(value: String) -> Self {
        HashedPassword(value)
    }
}

impl TryFrom<Password> for HashedPassword {
    type Error = HashedPasswordError;

    fn try_from(value: Password) -> Result<Self, Self::Error> {
        bcrypt::hash(value.0, DEFAULT_COST)
            .map(HashedPassword)
            .map_err(Self::Error::BcryptError)
    }
}
