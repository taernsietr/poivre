use std::fmt::{Display, Result, Formatter};
use std::str::FromStr;
use serde::{Serialize,Deserialize};

pub struct ParseCategoryError;

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

impl FromStr for Category {
    type Err = ParseCategoryError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ingredient" => Ok(Category::Ingredient),
            "dish" => Ok(Category::Dish),
            "beverage" => Ok(Category::Beverage),
            _ => Err(ParseCategoryError)
        }
    }
}

