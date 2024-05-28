use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    home::Home,
    admin::Admin,
    items::{ItemTable, ItemDescription},
    users::{Users, UserProfile},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/poivre-axum.css"/>
        <Title text="Poivre"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/admin" view=Admin/>
                    <Route path="/items" view=ItemTable/>
                    <Route path="/items/:id" view=ItemDescription/>
                    <Route path="/users" view=Users/>
                    <Route path="/users/:id" view=UserProfile/>
                </Routes>
            </main>
        </Router>
    }
}

