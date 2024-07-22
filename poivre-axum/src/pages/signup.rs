use leptos::*;
use leptos_meta::Title;
use leptos_router::{A, ActionForm};
use crate::{
    db::{
        setup,
        user::SignUp
    },
    resources::{
        user_errors::UserParseError,
        user::User
    }, resources::shared::Image
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

