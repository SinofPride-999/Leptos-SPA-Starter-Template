use leptos::prelude::*;

#[component]
pub fn Button() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <button
                on:click=move |_| {
                    set_count.update(|v| *v += 1);
                }
            >
                "Click me: "
                {count}
            </button>

            <p>
                "Double count: "
                {move || count.get() * 2}
            </p>
        </div>
    }
}
