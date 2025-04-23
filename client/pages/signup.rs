use crate::db::user::SignUp;
use leptos::*;
use leptos_meta::Title;
use leptos_router::{ActionForm, A};

/// User signup page
#[component]
pub fn SignupForm() -> impl IntoView {
  let signup = create_server_action::<SignUp>();

  view! {
    <Title text="Poivre - Sign Up"/>
    <ActionForm action=signup>
      <div class="form-row">
        <label class="form-entry">
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
        <label class="form-entry">
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
        <label class="form-entry">
          "Password: "
          <input
            type="password"
            placeholder="password"
            autocomplete="new-password"
            name="password"
          />
        </label>
      </div>
      <div class="form-row">
        <label class="form-entry">
          "First name: "
          <input
            type="text"
            placeholder="John"
            autocomplete="given-name"
            name="first_name"
            maxlength="64"
          />
        </label>
        <label class="form-entry">
          "Last name: "
          <input
            type="text"
            placeholder="Doe"
            autocomplete="family-name"
            name="last_name"
            maxlength="64"
          />
        </label>
        <label class="form-entry">
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
