use leptos::prelude::*;

use crate::components::Button;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="glass-panel home-container animate-fade-in">
            <h1 class="home-title">"Welcome to Leptos SPA!"</h1>
            <p class="home-subtitle">
                "A modern, reactive web application built with Rust and Leptos framework.
                Experience lightning-fast performance with zero JavaScript overhead."
            </p>

            <div class="features-grid">
            <div class="feature-item">
                <div class="feature-icon">"⚡"</div>
                <h3 class="font-semibold text-lg mb-1">"Blazing Fast"</h3>
                <p class="feature-text">"Compiles to WebAssembly for native speed"</p>
            </div>
            <div class="feature-item">
                <div class="feature-icon">"🛡️"</div>
                <h3 class="font-semibold text-lg mb-1">"Memory Safe"</h3>
                <p class="feature-text">"Rust's ownership model ensures safety"</p>
            </div>
            <div class="feature-item">
                <div class="feature-icon">"🎨"</div>
                <h3 class="font-semibold text-lg mb-1">"Reactive"</h3>
                <p class="feature-text">"Fine-grained reactivity system"</p>
            </div>
            <div class="feature-item">
                <div class="feature-icon">"🚀"</div>
                <h3 class="font-semibold text-lg mb-1">"Production Ready"</h3>
                <p class="feature-text">"Type-safe and optimized for performance"</p>
            </div>
            </div>

            <div class="btn-container">
                <p class="feature-text">
                    "Try the interactive counter below:"
                </p>
                <Button/>
            </div>
        </div>
    }
}
