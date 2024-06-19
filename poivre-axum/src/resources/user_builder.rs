use std::{
    error::Error,
    fmt
};
use leptos::logging;
use serde::{Serialize,Deserialize};
use chrono::{DateTime,NaiveDate,Utc,Local};
use regex::Regex;
use crate::shared::Image;

const DATE_FORMAT: &str = "%H:%M:%S";

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

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct UserBuilder {
    image: Image,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: NaiveDate,
}

impl UserBuilder {
    pub fn new(
        image: Image,
        username: String,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        date_of_birth: String 
    ) -> Result<UserBuilder, UserParseError> {
        let mut err = Vec::<UserParseError>::new();
        let timestamp = Local::now().format(DATE_FORMAT);
        let email_regex = Regex::new(r"[\w.+-]+@\w+\.\w{2,}").unwrap();
        let username_regex = Regex::new(ILLEGAL_USERNAME_CHARACTERS).unwrap();
        let password_regex = Regex::new(ILLEGAL_PASSWORD_CHARACTERS).unwrap();

        if password_regex.is_match(&password) { err.push(UserParseError::PasswordContainsInvalidCharacters); }
        else if password.chars().count() > PASSWORD_MAX_LENGTH { err.push(UserParseError::PasswordIsTooLong) }
        else if password.chars().count() < PASSWORD_MIN_LENGTH { err.push(UserParseError::PasswordIsTooShort) };

        if username_regex.is_match(&username) { err.push(UserParseError::UsernameContainsInvalidCharacters) }
        else if username.chars().count() > USERNAME_MAX_LENGTH { err.push(UserParseError::UsernameIsTooLong) }
        else if username.chars().count() < USERNAME_MIN_LENGTH { err.push(UserParseError::UsernameIsTooShort) };

        if !email_regex.is_match(&email) { err.push(UserParseError::InvalidEmail) }
        if first_name.chars().count() > FIRST_NAME_MAX_LENGTH { err.push(UserParseError::NameIsTooLong) }
        if last_name.chars().count() > LAST_NAME_MAX_LENGTH { err.push(UserParseError::LastNameIsTooLong) };

        let age: usize;
        let naive_date: NaiveDate; // f this
        match NaiveDate::parse_from_str(&date_of_birth, "%F") {
            Ok(inner) => {
                naive_date = inner;
                age = ((Utc::now().date_naive() - inner).num_days() / 365) as usize;
                if age < MINIMUM_AGE { err.push(UserParseError::DateOfBirthIsTooNear) }
                else if age > MAXIMUM_AGE { err.push(UserParseError::DateOfBirthIsTooFar) };
            },
            Err(_) => {
                err.push(UserParseError::InvalidDateOfBirth);
                naive_date = NaiveDate::default();
            }
        };

        match &err.len() {
            0 => (),
            1 => return Err(err[0].clone()),
            _ => {
                err.iter().for_each(|err| logging::error!("[{}] [poivre-axum] Error: {}", &timestamp, err));
                return Err(UserParseError::MultipleErrors);
            }
        };

        logging::log!("[{}] [poivre-axum] user parse ok, sending to db...", &timestamp);

        Ok (
            UserBuilder {
            image,
            username,
            email,
            password,
            first_name,
            last_name,
            date_of_birth: naive_date,
        })
    }
}
