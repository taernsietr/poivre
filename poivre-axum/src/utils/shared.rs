use serde::{Serialize,Deserialize};

/// A type implementing this trait is able to convert its contents into a Vec<String>, in order to
/// be iterated over and turned into a HTML table row.
pub trait PoivreTableRow {
    fn headers() -> impl Iterator<Item = String>;
    fn row_values(&self) -> impl Iterator<Item = String>;
}

pub trait PoivreCard: Clone {
    fn url(&self) -> String;
    fn img(&self) -> String;
    fn alt_text(&self) -> String;
    fn card_name(&self) -> String;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum ItemName {
    SingleName(String),
    MultipleName(Vec<String>)
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum Image {
    NoImage,
    HasImage(String)
}

