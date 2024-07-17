use leptos::*;
use crate::db::setup::SURREALDB;
use crate::resources::user::User;

/// Server endpoint for returning all database users (debugging purposes only).
#[server(GetAllUsers)]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
  //Result::Ok(User::mock_user_list())
  match SURREALDB
  .select::<Vec<User>>("users")
  .await {
    Ok(users) => {
      dbg!(&users);
      Ok(users)
    },
    Err(e) => Err(ServerFnError::from(e))
  }
}

//pub fn mock_user_list() -> Vec<User> {
//    vec!(
//        User {
//            id: "00000000".to_string(),
//            image: Image::NoImage,
//            username: "john_doe".to_string(),
//            email: "johndoe@provider.com".to_string(),
//            password: "abcdefghi".to_string(),
//            first_name: "John".to_string(),
//            last_name: "Doe".to_string(),
//            date_of_birth: "09/11/2001".to_string(),
//            friends: Vec::<String>::new()
//        }
//    )
//}
//
