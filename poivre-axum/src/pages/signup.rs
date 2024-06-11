use leptos::*;
use leptos_meta::Title;
use leptos_router::{A, ActionForm};
use crate::db::setup;
use crate::resources::users::User;

/// User signup page
#[component]
pub fn SignupForm() -> impl IntoView {
    let signup = create_server_action::<SignUp>();
    let value = signup.value();
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <Title text="Poivre - Sign Up"/>
        <ActionForm action=signup>
            <label class="m-4 p-4 flex-auto">
                "Username: "
                <input 
                    type="text"
                    placeholder="username"
                    autocomplete="off"
                    name="username" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "e-mail: "
                <input
                    type="email"
                    placeholder="username@domain.com"
                    autocomplete="off"
                    name="email" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "Password: "
                <input
                    type="password"
                    placeholder="password"
                    autocomplete="off"
                    name="password" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "First name: "
                <input
                    type="text"
                    placeholder="John"
                    autocomplete="off"
                    name="first_name" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "Last name: "
                <input
                    type="text"
                    placeholder="Doe"
                    autocomplete="off"
                    name="last_name" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "Date of Birth: "
                <input
                    type="date"
                    placeholder=""
                    name="birth_year" />
            </label>
            <input
                type="submit"
                value="Sign up" />
            <div>"Have an account already? "<A href="/login">"Login in!"</A></div> 
        </ActionForm>
    }
}

/// New account handler function
#[server(SignUp, "/internal")]
pub async fn sign_up(
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    date_of_birth: String
    ) -> Result<(), ServerFnError> {
        // TODO: handle profile image - user may or may not supply one
        let signup = User::new(
            None,
            username,
            email,
            password,
            first_name,
            last_name,
            date_of_birth 
        )?;

        let response: Vec<User> = setup::SURREALDB
            .create("users")
            .content(signup)
            .await?;
        
        println!("User: {:?}", &response[0]);
        Ok(())
}

