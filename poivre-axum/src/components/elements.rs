use leptos::*;
use leptos_router::A;

/// Main navbar element. Should appear in most, if not all pages.
/// TODO: If we are logged in, we should show profile links instead of the login link
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="float w-full border-4 border-solid justify-around mb-6 bg-orange-500">
            <div class="flex flex-row">
                <img
                    class="flex-initial p-2 m-2" 
                    src="logo-64.jpg"
                    alt="Poivre logo: a cartoonish pepper mill and salt shaker"
                />
                <A href="/">
                    <div class="flex-initial border-4 border-solid border-black-200
                        p-2 mx-2 my-6 bg-orange-700 hover:bg-white transition duration-300 ease-in-out">
                        Home
                    </div>
                </A>
                <A href="/login">
                    <div class="flex-initial border-4 border-solid border-black-200
                        p-2 mx-2 my-6 bg-orange-700 hover:bg-white transition duration-300 ease-in-out">
                        Login
                    </div>
                </A>
                <A href="/signup">
                    <div class="flex-initial border-4 border-solid border-black-200
                        p-2 mx-2 my-6 bg-orange-700 hover:bg-white transition duration-300 ease-in-out">
                        Sign Up
                    </div>
                </A>
                <A href="/admin">
                    <div class="flex-initial border-4 border-solid border-black-200
                        p-2 mx-2 my-6 bg-orange-700 hover:bg-white transition duration-300 ease-in-out">
                        Admin
                    </div>
                </A>
            </div>
        </nav>
    }
}

