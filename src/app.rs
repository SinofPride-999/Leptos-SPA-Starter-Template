use leptos::prelude::*;
use leptos_router::components::*;

use crate::routes::HomeRoutes;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <HomeRoutes/>
            </main>
        </Router>
    }
}
