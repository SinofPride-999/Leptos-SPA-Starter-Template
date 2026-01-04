use leptos::prelude::*;

#[component]
pub fn Button() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="flex flex-col items-center gap-4">
            <button
                class="btn-primary"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
            >
                "Click me: " {count}
            </button>
        </div>
    }
}
