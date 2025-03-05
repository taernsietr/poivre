use thiserror::Error;
use serde::{Serialize,Deserialize};

#[derive(Error,Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum ItemParseError {
  #[error("The supplied category is invalid")]
  InvalidCategory,
  #[error("Item name cannot be empty")]
  EmptyItemName,
  #[error("Item description cannot be empty")]
  EmptyItemDescription,
  #[error("Item category cannot be empty")]
  EmptyItemCategory,
  #[error("Could not validate item")]
  UnableToValidateItem,
}
