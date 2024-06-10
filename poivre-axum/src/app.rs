use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::{
    pages::{
        home::Home,
        admin::Admin,
        login::LoginForm,
        signup::SignupForm
    },
    components::{
        items::{ItemTable, ItemDescription},
        users::{UserTable, UserProfile},
        elements::Navbar
    }
};

/// Main app component. Contains the Router.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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
            <div class="mx-24 mt-8 bg-black-500">
                <Navbar />
                <main class="bg-orange-500">
                    <header>
                        <hgroup class="mx-auto my-4">
                            <h1 class="text-lg font-medium">"Poivre"</h1>
                            <sub><i>"['pwa.vR]"</i></sub>
                            <p>"A platform for mapping culinary preferences"</p>
                        </hgroup>
                    </header>
                    <div class="m-6 p-6 flex flex-col">
                        <Routes>
                            <Route path="/" view=Home />
                            <Route path="/admin" view=Admin />
                            <Route path="/signup" view=SignupForm />
                            <Route path="/login" view=LoginForm />
                            <Route path="/items" view=ItemTable />
                            <Route path="/items/:id" view=ItemDescription />
                            <Route path="/users" view=UserTable/>
                            <Route path="/users/:id" view=UserProfile />
                        </Routes>
                    </div>
                </main>
            </div>
        </Router>
    }
}

