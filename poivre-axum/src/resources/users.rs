use std::{
    error::Error,
    fmt
};
use regex::Regex;
use serde::{Serialize,Deserialize};
use crate::shared::TableRow;

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
pub struct User {
    id: Option<String>,
    image: Option<String>,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    friends: Option<Vec<String>>
}

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

impl Error for UserParseError {}

impl TableRow for User {
    fn headers (self) -> impl Iterator<Item = String> {
        vec!(
            "ID",
            "Image",
            "Username",
            "email",
            "First Name",
            "Last Name",
            "Date of Birth"
        ).into_iter().map(|x| x.to_string())
    }

    fn into_row(self) -> impl Iterator<Item = String> {
        vec!(
            self.id().unwrap_or_default(),
            self.image().unwrap_or_default(),
            self.username(),
            self.email(),
            self.first_name(),
            self.last_name(),
            self.date_of_birth(),
        ).into_iter()
    }
}

impl User {
    pub fn id(&self) -> Option<String> { self.id.clone() }
    pub fn image(&self) -> Option<String> { self.image.clone() }
    pub fn username(&self) -> String { self.username.clone() }
    pub fn email(&self) -> String { self.email.clone() }
    pub fn first_name(&self) -> String { self.first_name.clone() }
    pub fn last_name(&self) -> String { self.last_name.clone() }
    pub fn date_of_birth(&self) -> String { self.date_of_birth.clone() }
    pub fn friends(&self) -> Option<Vec<String>> { self.friends.clone() }

    pub fn new(
        image: Option<String>,
        username: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        date_of_birth: impl Into<String> + fmt::Debug 
        ) -> Result<User, UserParseError> {
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
            Ok (User {
                id: None,
                image,
                username,
                email,
                password,
                first_name,
                last_name,
                date_of_birth: age.to_string(),
                friends: None
            })
    }
    
    pub fn mock_user_list() -> Vec<User> {
        vec!(
            User::new(
                None,
                "john_doe",
                "johndoe@provider.com",
                "abcdefghi",
                "John",
                "Doe",
                "09/11/2001"
            ).unwrap()
        )
    }
}

