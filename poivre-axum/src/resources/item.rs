use std::iter::Iterator;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use super::category::Category;
use crate::resources::shared::{
  ItemName,
  Image,
  PoivreCard,
  PoivreTableRow
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

impl PoivreTableRow for Item {
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
      self.name(),
      self.category(),
      self.description(),
      self.descriptors(),
      self.associated_cuisines()
    ).into_iter()
  }
}

impl PoivreCard for Item {
  //fn url(&self) -> String { format!("/items/{}", self.id()) }
  fn url(&self) -> String { format!("/items/{}", "0000") }
  fn img(&self) -> String { self.image() }
  fn alt_text(&self) -> String { format!("image for the item {}", self.name()) }
  fn card_name(&self) -> String { self.name() }
}

// TODO: determine if Vec<String> getters should return the actual vec or a joined string
impl Item {
  pub fn id(&self) -> String {
    match &self.id {
      Some(id) => id.clone().to_string().replace("items:", ""),
      None => String::new()
    }
  }
  pub fn category(&self) -> String { self.category.to_string() }
  pub fn description(&self) -> String { self.description.clone() }
  pub fn descriptors(&self) -> String { self.descriptors.clone().join(", ") }
  pub fn associated_cuisines(&self) -> String { self.associated_cuisines.clone().join(", ") }

  // Returns the url for the image; if none is available, returns the url for a generic item
  // image
  // TODO: add generic image url
  pub fn image(&self) -> String {
    match &self.image {
      Image::HasImage(url) => url.clone(),
      Image::NoImage => String::new()
    }
  }

  /// Returns the name(s) for the item as a string. If there are multiple names, they are joined
  /// into a single string, separated by spaced commas.
  pub fn name(&self) -> String {
    match &self.name {
      ItemName::SingleName(name) => name.clone(),
      ItemName::MultipleName(name) => name.clone().join(", ")
    }
  }

}
