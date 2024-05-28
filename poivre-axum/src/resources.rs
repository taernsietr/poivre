use std::fmt::{Display, Result, Formatter};
use serde::{Serialize,Deserialize};

#[derive(Clone,Serialize,Deserialize,PartialEq)]
pub enum Name {
    SingleName(String),
    MultipleName(Vec<String>)
}

#[derive(Clone,Serialize,Deserialize,PartialEq)]
pub enum Category {
    Ingredient,
    Dish,
    Beverage,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Category::Ingredient => write!(f, "Ingredient"),
            Category::Dish => write!(f, "Dish"),
            Category::Beverage => write!(f, "Beverage"),
        }
    }
}

#[derive(Clone,Serialize,Deserialize,PartialEq)]
pub struct Item {
    id: String,
    name: Name,
    image: String,
    category: Category,
    description: String,
    descriptors: Vec<String>,
}

impl Item {
    pub fn id(&self) -> String { self.id.clone() }
    pub fn image(&self) -> String { self.image.clone() }
    pub fn category(&self) -> String { self.category.to_string() }
    pub fn description(&self) -> String { self.description.clone() }
    pub fn descriptors(&self) -> String { self.descriptors.clone().join(", ") }
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
        descriptors: Vec<impl Into<String>>
    ) -> Item {
        Item {
            id: id.into(),
            name,
            image: image.into(),
            category,
            description: description.into(),
            descriptors: descriptors.into_iter().map(|tag| tag.into()).collect::<_>()
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
                vec!("fruit", "sweet", "aromatic")
            ),
            Item::new(
                "0000000001",
                Name::SingleName("Banana".into()),
                "some/url",
                Category::Ingredient,
                "A sweet fruit, yellow, with a thick peel.",
                vec!("fruit", "sweet", "aromatic")
            )
        )
    }
}
