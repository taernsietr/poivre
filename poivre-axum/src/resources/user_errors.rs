use thiserror::Error;
use serde::{Serialize,Deserialize};
use crate::resources::shared::parameters::*;

#[derive(Error,Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum UserParseError {
  #[error("Invalid email")]
  InvalidEmail,
  #[error("Invalid image url")]
  InvalidImageUrl,
  #[error("Username contains invalid characters")]
  UsernameContainsInvalidCharacters,
  #[error("Password contains invalid characters")]
  PasswordContainsInvalidCharacters,
  #[error("Invalid date of birth")]
  InvalidDateOfBirth,
  #[error("You must be at least 14 to create an account")]
  DateOfBirthIsTooNear,
  #[error("Are you really over 120 years old?")]
  DateOfBirthIsTooFar,
  #[error("First name exceeds maximum length {}", FIRST_NAME_MAX_LENGTH)]
  FirstNameIsTooLong,
  #[error("Last name exceeds maximum length {}", LAST_NAME_MAX_LENGTH)]
  LastNameIsTooLong,
  #[error("Username is too long")]
  UsernameIsTooLong,
  #[error("Password is too long")]
  PasswordIsTooLong,
  #[error("Username is too short")]
  UsernameIsTooShort,
  #[error("Password is too short")]
  PasswordIsTooShort,
  #[error("Username cannot be empty")]
  EmptyUsername,
  #[error("Email cannot be empty")]
  EmptyEmail,
  #[error("Password cannot be empty")]
  EmptyPassword,
  #[error("First name cannot be empty")]
  EmptyFirstName,
  #[error("Last name cannot be empty")]
  EmptyLastName,
  #[error("Date of birth cannot be empty")]
  EmptyDateOfBirth,
  #[error("Could not validate user")]
  UnableToValidateUser
}

