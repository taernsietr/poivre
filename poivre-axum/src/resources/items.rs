use std::iter::Iterator;
use serde::{Serialize, Deserialize};
use super::category::Category;
use crate::shared::TableRow;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum Name {
    SingleName(String),
    MultipleName(Vec<String>)
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct Item {
    id: Option<String>,
    image: Option<String>,
    name: Name,
    category: Category,
    description: String,
    descriptors: Vec<String>,
    associated_cuisines: Vec<String>
}

impl TableRow for Item {
    fn headers (self) -> impl Iterator<Item = String> {
        vec!(
            "ID",
            "Image",
            "Name",
            "Category",
            "Description",
            "Descriptors",
            "Associated_Cuisines"
        ).into_iter().map(|x| x.to_string())
    }

    fn into_row(self) -> impl Iterator<Item = String> {
        vec!(
            self.id().unwrap_or_default(),
            self.image().unwrap_or_default(),
            self.name(),
            self.category(),
            self.description(),
            self.descriptors(),
            self.associated_cuisines()
        ).into_iter()
    }
}

impl Item {
    pub fn id(&self) -> Option<String> { self.id.clone() }
    pub fn image(&self) -> Option<String> { self.image.clone() }
    pub fn category(&self) -> String { self.category.to_string() }
    pub fn description(&self) -> String { self.description.clone() }
    pub fn descriptors(&self) -> String { self.descriptors.clone().join(", ") }
    pub fn associated_cuisines(&self) -> String { self.associated_cuisines.clone().join(", ") }
    /// Returns the name(s) for the item as a string. If there are multiple names, they are joined
    /// into a single string, separated by spaced commas.
    pub fn name(&self) -> String {
        match &self.name {
            Name::SingleName(name) => name.clone(),
            Name::MultipleName(name) => name.clone().join(", ")
        }
    }

    pub fn new(
        id: impl Into<String>,
        name: Name,
        image: impl Into<String>,
        category: Category,
        description: impl Into<String>,
        descriptors: Vec<impl Into<String>>,
        associated_cuisines: Vec<impl Into<String>>
    ) -> Item {
        Item {
            id: Some(id.into()),
            name,
            image: Some(image.into()),
            category,
            description: description.into(),
            descriptors: descriptors.into_iter().map(|descriptor| descriptor.into()).collect::<_>(),
            associated_cuisines: associated_cuisines.into_iter().map(|cuisine| cuisine.into()).collect::<_>()
        }
    }

    pub fn mock_item_list() -> Vec<Item> {
        vec!(
            Item::new(
                "0000000000",
                Name::SingleName("Apple".into()),
                "some/url",
                Category::Ingredient,
                "A sweet fruit, usually red.",
                vec!("fruit", "sweet", "aromatic"),
                vec!("")
            ),
            Item::new(
                "0000000001",
                Name::SingleName("Banana".into()),
                "some/url",
                Category::Ingredient,
                "A sweet fruit, yellow, with a thick peel.",
                vec!("fruit", "sweet", "aromatic"),
                vec!("")
            )
        )
    }
}
