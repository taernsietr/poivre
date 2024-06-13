use leptos::*;
use crate::resources::users::User;

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
            <div>
                <h4>"Latest activity"</h4>
            </div>
            <div>
                <h4>"Notifications"</h4>
            </div>
        </div>
    }
}

/// Page detailing the profile of a single user. Allows for editing profile information.
#[component]
pub fn UserProfile() -> impl IntoView {

}

/// Page for managing preferences: adding, removing or editing likes and dislikes
#[component]
pub fn UserPreferences() -> impl IntoView {

}

/// Page for managing friends
#[component]
pub fn UserFriends() -> impl IntoView {

}

/// Page for starting new reports, printing complete reports or seeing report history
#[component]
pub fn UserReports() -> impl IntoView {

}
