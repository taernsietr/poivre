use leptos::*;
use crate::resources::user::User;

/// Sidebar for navigating user pages when logged in.
#[component]
pub fn UserSidebar() -> impl IntoView {
    view! {
        <section>
            <h3>"Menu"</h3>
            <button>"Profile"</button>
            <button>"Preferences"</button>
            <button>"Friends"</button>
            <button>"Reports"</button>
        </section>
    }
}

/// Landing page for logged-in users.
#[component]
pub fn UserHome() -> impl IntoView {
    let user: User;

    view! {
        <UserSidebar />
        <div>
            <h3>"Hello, {user.name()}!"</h3>
        </div>
    }
}

/// Page detailing the profile of a single user. Allows for editing profile information.
#[component]
pub fn UserProfile() -> impl IntoView {
    view! {
        <div>
            <UserLatestActivity />
            <UserNotifications />
        </div>
    }
}

#[component]
pub fn UserLatestActivity() -> impl IntoView {
    view! {
        <div>
            <h4>"Latest activity"</h4>
        </div>
    }
}

#[component]
pub fn UserNotifications() -> impl IntoView {
    view! {
        <div>
            <h4>"Notifications"</h4>
        </div>
    }
}
