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

impl From<String> for ItemName {
  fn from(value: String) -> Self {
    //let trimmed = value.trim().to_string();
    let split = value
      .trim()
      .to_string()
      .split(" ,")
      .map(|str| str.trim().to_string())
      .collect::<Vec<String>>();

    match split.len() {
      1 => Self::Single(split.concat()),
      _ => Self::Multiple(split)
    }
  }
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
  pub static DEFAULT_ITEM_IMAGE_URL: &str = "default-item.png";
  pub static DEFAULT_USER_IMAGE_URL: &str = "default-user.png";
  pub static DEFAULT_CUISINE_IMAGE_URL: &str = "default-cuisine.png";

  pub static DATE_FORMAT: &str = "%H:%M:%S";

  // TODO: set the correct characters or refactor validation
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

