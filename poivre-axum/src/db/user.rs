use leptos::*;
use crate::db::setup;
use crate::resources::user::User;
use crate::resources::shared::Image;

/// Server endpoint for returning all database users (debugging purposes only).
#[server(GetAllUsers)]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
  match setup::SURREALDB
  .select::<Vec<User>>("users")
  .await {
    Ok(users) => {
      dbg!(&users);
      Ok(users)
    },
    Err(e) => Err(ServerFnError::from(e))
  }
}

/// New account handler function
#[server(SignUp)]
pub async fn sign_up(
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: String
) -> Result<(), ServerFnError> {
    // TODO: handle profile image - user may or may not supply one
    let form_data = User::new(
        Image::NoImage,
        username,
        email,
        password,
        first_name,
        last_name,
        date_of_birth,
    )?;

    let response: Result<Vec<User>,surrealdb::Error> = setup::SURREALDB
        .create("users")
        .content(form_data)
        .await;
    
    Ok(())
}

