use serde::{Serialize, Deserialize};
use crate::shared::{
    Image,
    PoivreCard,
    PoivreTableRow
};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct User {
    id: String,
    image: Image,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    friends: Vec<String>
}

impl PoivreTableRow for User {
    fn headers() -> impl Iterator<Item = String> {
        vec!(
            "ID",
            "Image",
            "Username",
            "email",
            "First Name",
            "Last Name",
            "Date of Birth"
        ).into_iter().map(|x| x.to_string())
    }

    fn into_row(&self) -> impl Iterator<Item = String> {
        vec!(
            self.id(),
            self.image(),
            self.username(),
            self.email(),
            self.first_name(),
            self.last_name(),
            self.date_of_birth(),
        ).into_iter()
    }
}

impl PoivreCard for User {
    fn url(&self) -> String { format!("/users/{}", self.id()) }
    fn img(&self) -> String { self.image() }
    fn alt_text(&self) -> String { format!("image for user {} {}", self.first_name(), self.last_name()) }
    fn card_name(&self) -> String { format!("{} {}", self.first_name(), self.last_name()) }
}

impl User {
    pub fn id(&self) -> String { self.id.clone() }
    pub fn username(&self) -> String { self.username.clone() }
    pub fn email(&self) -> String { self.email.clone() }
    pub fn first_name(&self) -> String { self.first_name.clone() }
    pub fn last_name(&self) -> String { self.last_name.clone() }
    pub fn date_of_birth(&self) -> String { self.date_of_birth.clone() }
    pub fn friends(&self) -> Vec<String> { self.friends.clone() }
    
    pub fn image(&self) -> String {
        match &self.image {
            Image::HasImage(url) => url.clone(),
            Image::NoImage => String::new()
        }
    }

    pub fn mock_user_list() -> Vec<User> {
        vec!(
            User {
                id: "00000000".to_string(),
                image: Image::NoImage,
                username: "john_doe".to_string(),
                email: "johndoe@provider.com".to_string(),
                password: "abcdefghi".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                date_of_birth: "09/11/2001".to_string(),
                friends: Vec::<String>::new()
            }
        )
    }
}

