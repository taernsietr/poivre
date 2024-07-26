use leptos::*;
use crate::db::setup::SURREALDB;
use crate::resources::user::User;
use crate::resources::shared::Image;

/// Server endpoint for returning all database users (debugging purposes only).
#[server(GetAllUsers)]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
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

/// New account handler function
#[server(SignUp)]
pub async fn sign_up(
  username: String,
  image: String,
  email: String,
  password: String,
  first_name: String,
  last_name: String,
  date_of_birth: String
) -> Result<(), ServerFnError> {
  match User::new()
    .set_image(image)
    .set_username(username)
    .set_email(email)
    .set_password(password)
    .set_name(first_name, last_name)
    .set_date_of_birth(date_of_birth)
    .validate()
  {
    Ok(user) => {
      SURREALDB
        .create::<Vec<User>>("users")
        .content(user)
        .await;
        Ok(())
    },
    Err(err) => {
      Err(err.into())
    }
  }

  //let response: Result<Vec<User>,surrealdb::Error> = setup::SURREALDB
  //SURREALDB
  //  .create("users")
  //  .content(form_data)
  //  .await;
  
  //Ok(())
}

