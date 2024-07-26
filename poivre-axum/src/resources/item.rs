use std::iter::Iterator;
use leptos::logging;
use serde::{Serialize, Deserialize};
use chrono::Local;
use surrealdb::sql::Thing;
use crate::resources::{
  category::Category,
  item_errors::ItemParseError,
  shared::{
    ItemName,
    Image,
    Displayable,
    parameters::*
  }
};

/// An item from the database; as such, it must have an ID
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct Item {
  id: Option<Thing>,
  image: Image,
  name: ItemName,
  category: Category,
  description: String,
  descriptors: Vec<String>,
  associated_cuisines: Vec<String>
}

// TODO: determine if Vec<String> getters should return the actual vec or a joined string
impl Item {
  pub fn category(&self) -> String { self.category.to_string() }
  pub fn description(&self) -> String { self.description.clone() }
  pub fn descriptors(&self) -> String { self.descriptors.clone().join(", ") }
  pub fn associated_cuisines(&self) -> String { self.associated_cuisines.clone().join(", ") }
  pub fn new(
    name: impl Into<ItemName>,
    image: Image,
    category: Category,
    description: String,
    descriptors: Vec<String>,
    associated_cuisines: Vec<String>
  ) -> Result<Item, ItemParseError> {
    let timestamp = Local::now().format(DATE_FORMAT);
    logging::log!("[{}] [poivre-axum] item parse ok, sending to db...", &timestamp);

    Ok (
      Item {
        id: None,
        name: name.into(),
        image,
        category,
        description,
        descriptors,
        associated_cuisines
      }
    )
  }

  pub fn update(&mut self) -> Self {
    Item::new(
      Some(self.id()),
      name,
      image,
      category,
      description,
      descriptors,
      associated_cuisines
    )
  }
}

impl Displayable for Item {
  fn id(&self) -> String {
    match &self.id {
      Some(id) => id.clone().id.to_string(),
      None => String::new()
    }
  }

  fn url(&self) -> String { format!("/items/{}", self.id()) }

  fn image(&self) -> String {
    match &self.image {
      Image::HasImage(url) => url.clone(),
      Image::NoImage => String::new()
    }
  }

  /// Returns the name(s) for the item as a string. If there are multiple names, they are joined
  /// into a single string, separated by spaced commas.
  fn display_name(&self) -> String {
    match &self.name {
      ItemName::Single(name) => name.clone(),
      ItemName::Multiple(name) => name.clone().join(", ")
    }
  }

  fn alt_text(&self) -> String { format!("image for the item {}", self.display_name()) }

  fn headers() -> impl Iterator<Item = String> {
    vec!(
      "ID",
      "Image",
      "Name",
      "Category",
      "Description",
      "Descriptors",
      "Associated Cuisines"
    ).into_iter().map(|x| x.to_string())
  }

  fn row_values(&self) -> impl Iterator<Item = String> {
    vec!(
      self.id(),
      self.image(),
      self.display_name(),
      self.category(),
      self.description(),
      self.descriptors(),
      self.associated_cuisines()
    ).into_iter()
  }

}

