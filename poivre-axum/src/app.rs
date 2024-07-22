use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
  utils::error_template::{AppError, ErrorTemplate},
  pages::{
    home::Home,
    admin::AdminDashboard,
    login::LoginForm,
    signup::SignupForm,
    user::UserHome,
    item::{
      ItemLanding,
      ItemDescription,
      AddItem,
      EditItem,
    },
    event::{
      CreateEvent,
      EventInvite,
      EventDetails,
    }
  },
  components::common::Navbar
};

/// Main app component. Contains the Router.
#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  let navbar_entries = vec!(
    ("Home", ""),
    ("Login", "login"),
    ("Sign Up", "signup"),
    ("Admin", "admin")
  );

  view! {
    <Stylesheet id="leptos" href="/pkg/poivre-axum.css"/>
    <Link rel="shortcut icon" type_="image/png" href="logo-32.png"/>
    <Router fallback=|| {
      let mut outside_errors = Errors::default();
      outside_errors.insert_with_default_key(AppError::NotFound);
      view! {
        <ErrorTemplate outside_errors/>
      }
      .into_view()
    }>
      <div class="h-screen max-w-[1920px] border-4 border-solid bg-orange-950 flex flex-col mx-auto mt-24">
        <Navbar entries=navbar_entries />
        <main class="flex flex-col border-4 border-solid bg-orange-300 m-4">
          <header class="place-self-center">
            <hgroup>
              <h1 class="text-lg font-medium">"Poivre" <sub><i>"['pwa.vʀ]"</i></sub></h1>
              <h2>"A platform for mapping culinary preferences and restrictions"</h2>
            </hgroup>
          </header>
          <div class="flex flex-col border-4 border-solid m-8 p-8">
            <Routes>
              <Route path="" view=Home />
              <Route path="admin" view=AdminDashboard />
              <Route path="signup" view=SignupForm />
              <Route path="login" view=LoginForm />
              <Route path="users/:id/" view=UserHome />
              <Route path="items" view=ItemLanding />
              <Route path="items/:id" view=ItemDescription />
              <Route path="items/:id/edit" view=EditItem />
              <Route path="add" view=AddItem />
            </Routes>
          </div>
        </main>
      </div>
    </Router>
  }
}

