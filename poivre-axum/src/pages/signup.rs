use leptos::*;
use leptos_meta::Title;
use leptos_router::{A, ActionForm};
use crate::{
    db::setup, resources::{
        user_builder::{UserBuilder, UserParseError},
        users::User
    }, shared::Image
};

/// User signup page
#[component]
pub fn SignupForm() -> impl IntoView {
    let signup = create_server_action::<SignUp>();

    view! {
        <Title text="Poivre - Sign Up"/>
        <ActionForm action=signup>
            <div class="flex flex-row">
                <label class="m-4 p-4 flex-auto">
                    "Username: "
                    <input 
                        type="text"
                        placeholder="username"
                        autocomplete="username"
                        name="username"
                        minlength="8"
                        maxlength="32"
                        autofocus
                    />
                </label>
                <label class="m-4 p-4 flex-auto">
                    "e-mail: "
                    <input
                        type="email"
                        placeholder="username@domain.com"
                        autocomplete="email"
                        name="email"
                        minlength="8"
                        maxlength="128"
                    />
                </label>
                <label class="m-4 p-4 flex-auto">
                    "Password: "
                    <input
                        type="password"
                        placeholder="password"
                        autocomplete="new-password"
                        name="password"
                    />
                </label>
            </div>
            <div class="flex flex-row">
                <label class="m-4 p-4 flex-auto">
                    "First name: "
                    <input
                        type="text"
                        placeholder="John"
                        autocomplete="given-name"
                        name="first_name"
                        maxlength="64"
                    />
                </label>
                <label class="m-4 p-4 flex-auto">
                    "Last name: "
                    <input
                        type="text"
                        placeholder="Doe"
                        autocomplete="family-name"
                        name="last_name"
                        maxlength="64"
                    />
                </label>
                <label class="m-4 p-4 flex-auto">
                    "Date of Birth: "
                    <input
                        type="date"
                        placeholder=""
                        autocomplete="bday"
                        name="date_of_birth"
                    />
                </label>
            </div>
            <input type="submit" value="Sign Up" />
            <div>"Have an account already? "<A href="/login">"Log in!"</A></div> 
        </ActionForm>
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
        let form_data = UserBuilder::new(
            Image::NoImage,
            username,
            email,
            password,
            first_name,
            last_name,
            date_of_birth 
        )?;

        let response: Result<Vec<UserBuilder>,surrealdb::Error> = setup::SURREALDB
            .create("users")
            .content(form_data)
            .await;
        
        Ok(())
}

