use serde::{Serialize,Deserialize};

/// A type implementing this trait is able to convert its contents into a Vec of String, in order to
/// be iterated over and turned into a HTML table row.
pub trait PoivreTableRow {
  fn headers() -> impl Iterator<Item = String>;
  fn row_values(&self) -> impl Iterator<Item = String>;
}

/// A type implementing this trait fulfills the criteria for being displayed as a standard display
/// card
pub trait PoivreCard: Clone {
  fn url(&self) -> String;
  fn img(&self) -> String;
  fn alt_text(&self) -> String;
  fn card_name(&self) -> String;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum ItemName {
  SingleName(String),
  MultipleName(Vec<String>)
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum Image {
  NoImage,
  HasImage(String)
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum AttendeeStatus {
  Pending,
  Rejected,
  Accepted,
  Removed
}

pub mod parameters {
  pub static DATE_FORMAT: &str = "%H:%M:%S";

  // TODO: set the correct characters
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
}

