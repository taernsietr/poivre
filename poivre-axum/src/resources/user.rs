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
    Displayable,
    parameters::*
  }
};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct User {
  id: Option<Thing>,
  image: Image,
  username: String,
  email: String,
  password: String,
  first_name: String,
  last_name: String,
  date_of_birth: String,
  friends: Vec<String>
}

impl User {
  pub fn username(&self) -> String { self.username.clone() }
  pub fn email(&self) -> String { self.email.clone() }
  pub fn first_name(&self) -> String { self.first_name.clone() }
  pub fn last_name(&self) -> String { self.last_name.clone() }
  pub fn date_of_birth(&self) -> String { self.date_of_birth.clone() }
  pub fn friends(&self) -> Vec<String> { self.friends.clone() }
  
  pub fn new(
    image: Image,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: String 
  ) -> Result<User, UserParseError> {
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
    let valid_date: NaiveDate; // f this

    match NaiveDate::parse_from_str(&date_of_birth, "%F") {
      Ok(inner) => {
        valid_date = inner;
        age = ((Utc::now().date_naive() - inner).num_days() / 365) as usize;
        if age < MINIMUM_AGE { err.push(UserParseError::DateOfBirthIsTooNear) }
        else if age > MAXIMUM_AGE { err.push(UserParseError::DateOfBirthIsTooFar) };
      },
      Err(_) => {
        err.push(UserParseError::InvalidDateOfBirth);
        valid_date = NaiveDate::default()
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

    let friends = vec!();

    logging::log!("[{}] [poivre-axum] user parse ok, sending to db...", &timestamp);

    Ok (
      User {
        id: None,
        image,
        username,
        email,
        password,
        first_name,
        last_name,
        date_of_birth: valid_date.to_string(),
        friends
      }
    )
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

  fn display_name(&self) -> String { format!("{} {}", self.first_name(), self.last_name()) }
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
      self.first_name(),
      self.last_name(),
      self.date_of_birth(),
    ).into_iter()
  }
}

