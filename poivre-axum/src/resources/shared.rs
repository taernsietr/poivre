use std::fmt::format;

use serde::{Serialize,Deserialize};

/// A type implementing this trait it able to be rendered as a table row, a display card, or as a
/// displayable item in general, linking to its description page.
pub trait Displayable: Clone {
  /// Returns its id as stored in the database, without its table name prefixed
  fn id(&self) -> String;
  /// Returns the url to its description page
  fn url(&self) -> String;
  /// Returns the url to its image; if no image is available, redirects to the default image for
  /// its type
  fn image(&self) -> String;
  /// Returns its name, formatted according to its type
  fn display_name(&self) -> String;
  /// Returns the img element alt text, formatted according to its type
  fn alt_text(&self) -> String;
  /// Returns header values for a table displaying items of its type
  fn headers() -> impl Iterator<Item = String>;
  /// Returns its values as appropriate for displaying as a table row
  fn row_values(&self) -> impl Iterator<Item = String>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum ItemName {
  Single(String),
  Multiple(Vec<String>)
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct FullName {
  pub first_name: String,
  pub last_name: String
}

impl FullName {
  pub fn is_empty(&self) -> bool {
    self.first_name.is_empty() && self.last_name.is_empty()
  }
}

impl From<String> for ItemName {
  fn from(value: String) -> Self {
    let split = value
      .trim()
      .split(" ,")
      .map(|str| str.trim().to_string())
      .collect::<Vec<String>>();

    match split.len() {
      1 => Self::Single(split.concat()),
      _ => Self::Multiple(split)
    }
  }
}

impl Default for ItemName {
  fn default() -> Self {
    ItemName::Single(String::new())
  }
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub enum Image {
  #[default]
  NoImage,
  HasImage(String)
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub enum AttendeeStatus {
  #[default]
  Pending,
  Rejected,
  Accepted,
  Removed
}

pub mod parameters {
  pub const DEFAULT_ITEM_IMAGE_URL: &str = "default-item.png";
  pub const DEFAULT_USER_IMAGE_URL: &str = "default-user.png";
  pub const DEFAULT_CUISINE_IMAGE_URL: &str = "default-cuisine.png";

  pub const DATE_FORMAT: &str = "%H:%M:%S";

  // TODO: set the correct characters or refactor validation
  pub const EMAIL_REGEX: &str = r"[\w.+-]+@\w+\.\w{2,}";
  pub const ILLEGAL_USERNAME_CHARACTERS: &str = "º°ª§";
  pub const ILLEGAL_PASSWORD_CHARACTERS: &str = "º°ª§";
  pub const FIRST_NAME_MAX_LENGTH: usize = 64;
  pub const LAST_NAME_MAX_LENGTH: usize = 64;
  pub const USERNAME_MAX_LENGTH: usize = 32;
  pub const USERNAME_MIN_LENGTH: usize = 8;
  pub const PASSWORD_MAX_LENGTH: usize = 128;
  pub const PASSWORD_MIN_LENGTH: usize = 8;
  pub const MAXIMUM_AGE: usize = 120;
  pub const MINIMUM_AGE: usize = 14;
}

