use leptos::*;
use leptos_router::A;
use leptos_meta::Title;

/// User login page
#[component]
pub fn LoginForm() -> impl IntoView {
    view! {
        <Title text="Poivre - Login"/>
        <form method="POST">
            <label class="m-4 p-4 flex-auto">
                "Username: "
                <input
                    type="text"
                    placeholder="username"
                    name="user-login" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "Password: "
                <input
                    type="password"
                    placeholder="password"
                    name="user-password" />
            </label>
            <label class="m-4 p-4 flex-auto">
                "Remember Login?"
                <input
                    type="checkbox"
                    name="remember-login" />
            </label>
            <input type="submit" />
            <div>"Not registered? "<A href="/signup">"Sign up!"</A></div> 
        </form>
    }
}

