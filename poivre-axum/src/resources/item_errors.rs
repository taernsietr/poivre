use thiserror::Error;
use serde::{Serialize,Deserialize};
use crate::resources::shared::parameters::*;

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
}
