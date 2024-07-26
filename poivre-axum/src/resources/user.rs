use std::{error::Error, fmt};
use leptos::logging;
use serde::{Serialize,Deserialize};
use chrono::{DateTime,NaiveDate,Utc,Local};
use regex::Regex;
use surrealdb::sql::Thing;
use crate::resources::{
  user_errors::UserParseError,
  shared::{
    Image,
    FullName,
    Displayable,
    parameters::*
  }
};

/// Represents an user account.
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct User {
  id: Option<Thing>,
  image: Image,
  username: String,
  email: String,
  password: String,
  name: FullName,
  date_of_birth: String,
  friends: Vec<String>
}

impl User {
  /// Constructs an empty User.
  pub fn new() -> User {
    User {
      id: None,
      image: Image::default(),
      username: String::new(),
      email: String::new(),
      password: String::new(),
      name: FullName::default(),
      date_of_birth: String::new(),
      friends: Vec::<String>::new()
    }
  }

  pub fn username(&self) -> String { self.username.clone() }
  pub fn email(&self) -> String { self.email.clone() }
  pub fn name(&self) -> FullName { self.name.clone() }
  pub fn date_of_birth(&self) -> String { self.date_of_birth.clone() }
  pub fn friends(&self) -> Vec<String> { self.friends.clone() }

  /// Sets the profile image for the User. Any string will be set, while an empty input will set
  /// the image to NoImage. Invalid URLs are not checked.
  pub fn set_image(mut self, image: String) -> Self {
    if image.is_empty() { self.image = Image::NoImage }
    else { self.image = Image::HasImage(image); }
    self
  }

  pub fn set_email(mut self, email: String) -> Self {
    if !Regex::new(EMAIL_REGEX).unwrap().is_match(&email) { eprintln!("{}", UserParseError::InvalidEmail) }
    else { self.email = email };
    self
  }

  pub fn set_username(mut self, username: String) -> Self {
    let invalid = Regex::new(ILLEGAL_USERNAME_CHARACTERS)
      .unwrap()
      .is_match(&username);
    match username.chars().count() {
      0 => eprintln!("{}", UserParseError::EmptyUsername),
      USERNAME_MAX_LENGTH..=usize::MAX => eprintln!("{}", UserParseError::UsernameIsTooLong),
      1..=USERNAME_MIN_LENGTH => eprintln!("{}", UserParseError::UsernameIsTooShort),
      _ if invalid => eprintln!("{}", UserParseError::UsernameContainsInvalidCharacters),
      _ => self.username = username
    }
    self
  }

  pub fn set_password(mut self, password: String) -> Self {
    let invalid = Regex::new(ILLEGAL_PASSWORD_CHARACTERS)
      .unwrap()
      .is_match(&password);
    match password.chars().count() {
      0 => eprintln!("{}", UserParseError::EmptyPassword),
      PASSWORD_MAX_LENGTH..=usize::MAX => eprintln!("{}", UserParseError::PasswordIsTooLong),
      1..=PASSWORD_MIN_LENGTH => eprintln!("{}", UserParseError::PasswordIsTooShort),
      _ if invalid => eprintln!("{}", UserParseError::PasswordContainsInvalidCharacters),
      _ => self.password = password 
    }
    self
  }

  pub fn set_name(mut self, first_name: String, last_name: String) -> Self {
    match first_name.chars().count() {
      0 => eprintln!("{}", UserParseError::EmptyFirstName),
      FIRST_NAME_MAX_LENGTH.. => { eprintln!("{}", UserParseError::FirstNameIsTooLong) },
      _ => match last_name.chars().count() {
        0 => eprintln!("{}", UserParseError::EmptyLastName),
        LAST_NAME_MAX_LENGTH.. => eprintln!("{}", UserParseError::LastNameIsTooLong),
        _ => { self.name = FullName { first_name, last_name } }
      }
    }
    self
  }

  pub fn set_date_of_birth(mut self, date_of_birth: String) -> Self {
    match NaiveDate::parse_from_str(&date_of_birth, "%F") {
      Ok(inner) => {
        match ((Utc::now().date_naive() - inner).num_days() / 365) as usize {
          ..MINIMUM_AGE => { eprintln!("{}", UserParseError::DateOfBirthIsTooNear) },
          MAXIMUM_AGE.. => { eprintln!("{}", UserParseError::DateOfBirthIsTooFar) },
          _ => self.date_of_birth = inner.to_string()
        }
      },
      Err(_) => { eprintln!("{}", UserParseError::InvalidDateOfBirth) }
    }
    self
  }

  pub fn validate(self) -> Result<Self,UserParseError> {
     if !self.username.is_empty() &&
     !self.email.is_empty() &&
     !self.password.is_empty() &&
     !self.name.is_empty() &&
     !self.date_of_birth.is_empty() {
       Ok(self)
     }
     else { Err(UserParseError::UnableToValidateUser) }
  }
}

impl Displayable for User {
  fn id(&self) -> String {
    match &self.id {
      Some(id) => id.clone().id.to_string(),
      None => String::new()
    }
  }

  fn url(&self) -> String { format!("/users/{}", self.id()) }

  fn image(&self) -> String {
    match &self.image {
      Image::HasImage(url) => url.clone(),
      Image::NoImage => String::new()
    }
  }

  fn display_name(&self) -> String { format!("{} {}", self.name.first_name, self.name.last_name) }
  fn alt_text(&self) -> String { format!("image for user {}", self.username()) }

  fn headers() -> impl Iterator<Item = String> {
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

  fn row_values(&self) -> impl Iterator<Item = String> {
    vec!(
      self.id(),
      self.image(),
      self.username(),
      self.email(),
      self.display_name(),
      self.date_of_birth(),
    ).into_iter()
  }
}

