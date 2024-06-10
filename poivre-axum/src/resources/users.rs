use std::{
    error::Error,
    fmt
};
use chrono::Datelike;
use regex::Regex;
use serde::{Serialize,Deserialize};

pub static ILLEGAL_USERNAME_CHARACTERS: &str = "";
pub static ILLEGAL_PASSWORD_CHARACTERS: &str = "";
pub static FIRST_NAME_MAX_LENGTH: usize = 64;
pub static LAST_NAME_MAX_LENGTH: usize = 64;
pub static USERNAME_MAX_LENGTH: usize = 32;
pub static USERNAME_MIN_LENGTH: usize = 8;
pub static PASSWORD_MAX_LENGTH: usize = 128;
pub static PASSWORD_MIN_LENGTH: usize = 8;
pub static MAXIMUM_AGE: usize = 120;
pub static MINIMUM_AGE: usize = 14;

#[derive(Debug,Serialize,Deserialize)]
pub struct User {
    id: Option<String>,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    birth_year: String,
    friends: Option<Vec<String>>
}

#[derive(Debug)]
pub enum UserParseError {
    InvalidEmail,
    UsernameContainsInvalidCharacters,
    PasswordContainsInvalidCharacters,
    InvalidBirthYear,
    BirthYearIsTooNew,
    BirthYearIsTooOld,
    NameIsTooLong,
    LastNameIsTooLong,
    UsernameIsTooLong,
    PasswordIsTooLong,
    UsernameIsTooShort,
    PasswordIsTooShort
}

impl fmt::Display for UserParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserParseError::InvalidEmail => write!(f, "Invalid email"),
            UserParseError::InvalidBirthYear => write!(f, "Invalid birth year"),
            UserParseError::UsernameContainsInvalidCharacters => write!(f, "Username contains invalid characters"),
            UserParseError::UsernameIsTooLong => write!(f, "Username is too long"),
            UserParseError::NameIsTooLong => write!(f, "First name exceeds maximum length {FIRST_NAME_MAX_LENGTH}"),
            UserParseError::LastNameIsTooLong => write!(f, "Last name exceeds maximum length {LAST_NAME_MAX_LENGTH}"),
            UserParseError::UsernameIsTooShort => write!(f, "Username is too short"),
            UserParseError::PasswordContainsInvalidCharacters => write!(f, "Password contains invalid characters"),
            UserParseError::PasswordIsTooLong => write!(f, "Username is too long"),
            UserParseError::PasswordIsTooShort => write!(f, "Username is too short"),
            UserParseError::BirthYearIsTooNew => write!(f, "You must be at least 14 to create an account"),
            UserParseError::BirthYearIsTooOld => write!(f, "Are you really over 120 years old?")
        }
    }
}

impl Error for UserParseError {}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        birth_year: String 
        ) -> Result<User, UserParseError> {
            let year = chrono::NaiveDateTime::parse_from_str(&birth_year, "%Y");
            let age: usize;
            let username_regex = Regex::new(ILLEGAL_USERNAME_CHARACTERS).unwrap();
            let email_regex = Regex::new(r"^[\w.+-]+@\w+\.\w{2,}$").unwrap();
            let password_regex = Regex::new(ILLEGAL_PASSWORD_CHARACTERS).unwrap();

            if password_regex.is_match(&password) {
                return Err(UserParseError::PasswordContainsInvalidCharacters) };
            if username_regex.is_match(&username) {
                return Err(UserParseError::UsernameContainsInvalidCharacters) };
            if email_regex.is_match(&email) {
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
            match year {
                Ok(inner) => age = inner.year() as usize,
                Err(_) => {
                    return Err(UserParseError::InvalidBirthYear) }
            };
            if age < MINIMUM_AGE {
                return Err(UserParseError::BirthYearIsTooNew) };
            if age > MAXIMUM_AGE {
                return Err(UserParseError::BirthYearIsTooOld) };
            Ok (User {
                id: None,
                username,
                email,
                password,
                first_name,
                last_name,
                birth_year: age.to_string(),
                friends: None
            })
    }
}
