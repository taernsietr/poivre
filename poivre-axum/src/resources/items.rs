use std::iter::Iterator;
use serde::{Serialize, Deserialize};
use super::category::Category;
use crate::shared::{
    ItemName,
    Image,
    PoivreCard,
    PoivreTableRow
};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct ItemBuilder {
    image: Image,
    name: ItemName,
    category: Category,
    description: String,
    descriptors: Vec<String>,
    associated_cuisines: Vec<String>
}

impl ItemBuilder {
    pub fn new(
        name: ItemName,
        image: Image,
        category: Category,
        description: impl Into<String>,
        descriptors: Vec<impl Into<String>>,
        associated_cuisines: Vec<impl Into<String>>
    ) -> Self {
        ItemBuilder {
            name,
            image,
            category,
            description: description.into(),
            descriptors: descriptors.into_iter().map(|descriptor| descriptor.into()).collect::<_>(),
            associated_cuisines: associated_cuisines.into_iter().map(|cuisine| cuisine.into()).collect::<_>()
        }
    }
}

/// An item from the database; as such, it must have an ID
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct Item {
    id: String,
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
            "Associated_Cuisines"
        ).into_iter().map(|x| x.to_string())
    }

    fn into_row(&self) -> impl Iterator<Item = String> {
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
    fn url(&self) -> String { format!("/items/{}", self.id()) }
    fn img(&self) -> String { self.image() }
    fn alt_text(&self) -> String { format!("image for the item {}, {}", self.id(), self.name()) }
    fn card_name(&self) -> String { self.name() }
}

// TODO: determine if Vec<String> getters should return the actual vec or a joined string
impl Item {
    pub fn id(&self) -> String { self.id.clone() }
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

    pub fn mock_item_list() -> Vec<Item> {
        vec!(
            Item {
                id: "0000000000".to_string(),
                name: ItemName::SingleName("Apple".to_string()),
                image: Image::NoImage,
                category: Category::Ingredient,
                description: "A sweet fruit, usually red.".to_string(),
                descriptors: vec!("fruit".to_string(), "sweet".to_string(), "aromatic".to_string()),
                associated_cuisines: vec!("".to_string())
            },
            Item {
                id: "0000000001".to_string(),
                name: ItemName::SingleName("Banana".to_string()),
                image: Image::HasImage("some/url".to_string()),
                category: Category::Ingredient,
                description: "A sweet fruit, yellow, with a thick peel.".to_string(),
                descriptors: vec!("fruit".to_string(), "sweet".to_string(), "aromatic".to_string()),
                associated_cuisines: vec!("".to_string())
            },
            Item {
                id: "0000000002".to_string(),
                name: ItemName::SingleName("Pizza".into()),
                image: Image::HasImage("some/url".to_string()),
                category: Category::Dish,
                description: "One of the most popular dishes worldwide with roots in Italy, pizzas are flat disks of dough (though ocasionally found in other shapes), usually covered with tomato sauce and melted cheese, and may have any number of toppings added to it.".to_string(),
                descriptors: vec!("bread".to_string(), "gluten".to_string(), "savory".to_string(), "baked".to_string()),
                associated_cuisines: vec!("italian".to_string(), "fast-food".to_string())
            }
        )
    }
}
