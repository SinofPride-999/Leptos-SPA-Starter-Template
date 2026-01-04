use leptos::*;
use leptos_router::{components::*, path};

use crate::pages::HomePage;

#[component]
pub fn HomeRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| "Not found.">
            <Route path=path!("/") view=HomePage/>
        </Routes>
    }
}
