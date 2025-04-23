use leptos::*;
use leptos_router::A;
use leptos_meta::Title;

/// User login page
#[component]
pub fn LoginForm() -> impl IntoView {
    view! {
        <Title text="Poivre - Login"/>
        <form method="POST">
            <label class="form-entry">
                "Username: "
                <input
                    type="text"
                    placeholder="username"
                    name="user-login" />
            </label>
            <label class="form-entry">
                "Password: "
                <input
                    type="password"
                    placeholder="password"
                    name="user-password" />
            </label>
            <label class="form-entry">
                "Remember Login?"
                <input
                    type="checkbox"
                    name="remember-login" />
            </label>
            <button type="submit">"Login"</button>
            <div>"Not registered? "<A href="/signup">"Sign up!"</A></div> 
        </form>
    }
}

