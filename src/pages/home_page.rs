use leptos::prelude::*;

use crate::components::Button;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos SPA!"</h1>
        <p>"This is the Home page."</p>
        <Button/>
    }
}
