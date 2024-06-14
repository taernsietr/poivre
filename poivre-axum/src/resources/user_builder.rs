use std::{
    error::Error,
    fmt
};
use regex::Regex;
use serde::{Serialize,Deserialize};
use crate::shared::Image;

pub static ILLEGAL_USERNAME_CHARACTERS: &str = "º°ª§";
pub static ILLEGAL_PASSWORD_CHARACTERS: &str = "º°ª§";
pub static FIRST_NAME_MAX_LENGTH: usize = 64;
pub static LAST_NAME_MAX_LENGTH: usize = 64;
pub static USERNAME_MAX_LENGTH: usize = 32;
pub static USERNAME_MIN_LENGTH: usize = 8;
pub static PASSWORD_MAX_LENGTH: usize = 128;
pub static PASSWORD_MIN_LENGTH: usize = 8;
pub static MAXIMUM_AGE: usize = 120;
pub static MINIMUM_AGE: usize = 14;

#[derive(Debug)]
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
    PasswordIsTooShort
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
            UserParseError::PasswordIsTooLong => write!(f, "Username is too long"),
            UserParseError::PasswordIsTooShort => write!(f, "Username is too short"),
            UserParseError::DateOfBirthIsTooNear => write!(f, "You must be at least 14 to create an account"),
            UserParseError::DateOfBirthIsTooFar => write!(f, "Are you really over 120 years old?")
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct UserBuilder {
    image: Image,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: String,
}

impl UserBuilder {
    pub fn new(
        image: Image,
        username: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        date_of_birth: impl Into<String>
    ) -> Result<UserBuilder, UserParseError> {
        let username = username.into();
        let email = email.into();
        let password = password.into();
        let first_name = first_name.into();
        let last_name = last_name.into();
        let date_of_birth = chrono::NaiveDate::parse_from_str(&date_of_birth.into(), "%m/%d/%Y");

        let age: usize;
        let email_regex = Regex::new(r"[\w.+-]+@\w+\.\w{2,}").unwrap();
        let username_regex = Regex::new(ILLEGAL_USERNAME_CHARACTERS).unwrap();
        let password_regex = Regex::new(ILLEGAL_PASSWORD_CHARACTERS).unwrap();

        if password_regex.is_match(&password) {
            return Err(UserParseError::PasswordContainsInvalidCharacters) };
        if username_regex.is_match(&username) {
            return Err(UserParseError::UsernameContainsInvalidCharacters) };
        if !email_regex.is_match(&email) {
            return Err(UserParseError::InvalidEmail) };
        if username.len() > USERNAME_MAX_LENGTH {
            return Err(UserParseError::UsernameIsTooLong) };
        if username.len() < USERNAME_MIN_LENGTH {
            return Err(UserParseError::UsernameIsTooShort) };
        if password.len() > PASSWORD_MAX_LENGTH {
            return Err(UserParseError::PasswordIsTooLong) };
        if password.len() < PASSWORD_MIN_LENGTH {
            return Err(UserParseError::PasswordIsTooShort) };
        if first_name.len() > FIRST_NAME_MAX_LENGTH {
            return Err(UserParseError::NameIsTooLong) };
        if last_name.len() > LAST_NAME_MAX_LENGTH {
            return Err(UserParseError::LastNameIsTooLong) };

        match date_of_birth {
            Ok(inner) => age =
                ((chrono::Utc::now().date_naive() - inner).num_days() / 365) as usize,
            Err(_) => { return Err(UserParseError::InvalidDateOfBirth) }
        };
        if age < MINIMUM_AGE {
            return Err(UserParseError::DateOfBirthIsTooNear) };
        if age > MAXIMUM_AGE {
            return Err(UserParseError::DateOfBirthIsTooFar) };
        Ok (UserBuilder {
            image,
            username,
            email,
            password,
            first_name,
            last_name,
            date_of_birth: age.to_string(),
        })
    }
}
