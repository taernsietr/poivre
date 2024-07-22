use std::{
  error::Error,
  fmt
};
use serde::{Serialize,Deserialize};
use crate::resources::shared::parameters::*;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum UserParseError {
  InvalidEmail,
  UsernameContainsInvalidCharacters,
  PasswordContainsInvalidCharacters,
  InvalidDateOfBirth,
  DateOfBirthIsTooNear,
  DateOfBirthIsTooFar,
  NameIsTooLong,
  LastNameIsTooLong,
  UsernameIsTooLong,
  PasswordIsTooLong,
  UsernameIsTooShort,
  PasswordIsTooShort,
  MultipleErrors
}

impl Error for UserParseError {}

impl fmt::Display for UserParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UserParseError::InvalidEmail => write!(f, "Invalid email"),
      UserParseError::InvalidDateOfBirth => write!(f, "Invalid date of birth"),
      UserParseError::UsernameContainsInvalidCharacters => write!(f, "Username contains invalid characters"),
      UserParseError::UsernameIsTooLong => write!(f, "Username is too long"),
      UserParseError::NameIsTooLong => write!(f, "First name exceeds maximum length {FIRST_NAME_MAX_LENGTH}"),
      UserParseError::LastNameIsTooLong => write!(f, "Last name exceeds maximum length {LAST_NAME_MAX_LENGTH}"),
      UserParseError::UsernameIsTooShort => write!(f, "Username is too short"),
      UserParseError::PasswordContainsInvalidCharacters => write!(f, "Password contains invalid characters"),
      UserParseError::PasswordIsTooLong => write!(f, "Password is too long"),
      UserParseError::PasswordIsTooShort => write!(f, "Password is too short"),
      UserParseError::DateOfBirthIsTooNear => write!(f, "You must be at least 14 to create an account"),
      UserParseError::DateOfBirthIsTooFar => write!(f, "Are you really over 120 years old?"),
      UserParseError::MultipleErrors => write!(f, "Multiple errors found.")
    }
  }
}

