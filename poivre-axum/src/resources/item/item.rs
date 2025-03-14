use std::iter::Iterator;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use crate::resources::{
  category::Category,
  item_errors::ItemParseError,
  shared::{
    ItemName,
    Image,
    Displayable
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
  tags: Vec<String>,
  cuisines: Vec<String>
}

// TODO: determine if Vec<String> getters should return the actual vec or a joined string
impl Item {
  pub fn new() -> Item {
    Item {
      id: None,
      image: Image::default(),
      name: ItemName::default(),
      category: Category::default(),
      description: String::new(),
      tags: Vec::<String>::new(),
      cuisines: Vec::<String>::new()
    }
  }

  pub fn category(&self) -> String { self.category.to_string() }
  pub fn description(&self) -> String { self.description.clone() }
  pub fn tags(&self) -> String { self.tags.clone().join(", ") }
  pub fn cuisines(&self) -> String { self.cuisines.clone().join(", ") }

  /// Sets the display image for the Item. Any string will be set, while an empty input will set
  /// the image to NoImage. Invalid URLs are not checked.
  pub fn set_image(mut self, image: String) -> Self {
    if image.is_empty() { self.image = Image::NoImage }
    else { self.image = Image::HasImage(image); }
    self
  }

  pub fn set_name(mut self, name: String) -> Self {
    match name.chars().count() {
      0 => eprintln!("{}", ItemParseError::EmptyItemName),
      _ => { self.name = ItemName::from(name) }
    }
    self
  }
  
  pub fn set_category(mut self, category: String) -> Self {
    if category.is_empty() { eprintln!("{}", ItemParseError::EmptyItemCategory) }
    else {
      if let Ok(cat) = Category::from_str(&category) {
        self.category = cat
      }
    };
    self
  }

  pub fn set_description(mut self, description: String) -> Self {
    self.description = description;
    self
  }

  pub fn set_tags(mut self, tags: Vec<String>) -> Self {
    self.tags = tags;
    self
  }

  pub fn set_cuisines(mut self, cuisines: Vec<String>) -> Self {
    self.cuisines = cuisines;
    self
  }

  pub fn validate(self) -> Result<Self,ItemParseError> {
    if !self.display_name().is_empty() &&
    !self.description.is_empty() &&
    !self.tags.is_empty() &&
    !self.cuisines.is_empty() {
      Ok(self)
    }
    else {
      Err(ItemParseError::UnableToValidateItem)
    }
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

  fn alt_text(&self) -> String {
    format!("image for the item {}", self.display_name())
  }

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
      self.tags(),
      self.cuisines()
    ).into_iter()
  }

}

