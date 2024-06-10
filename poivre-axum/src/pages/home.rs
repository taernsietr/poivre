use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Poivre - Home"/>
        <p>"Food is the central need, shared by all forms of life. For us, human beings, it is also a fundamental part of our cultures."</p>
    }
}

