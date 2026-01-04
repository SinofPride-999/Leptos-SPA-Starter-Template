use leptos::prelude::*;

#[component]
pub fn Button() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="flex flex-col items-center gap-4">
            <button
                class="bg-gradient-to-r from-indigo-600 to-cyan-500 text-white font-bold py-3 px-6 rounded-lg
                       shadow-lg hover:shadow-xl hover:-translate-y-1 transition-all duration-300
                       hover:from-indigo-700 hover:to-cyan-600 active:scale-95
                       border border-indigo-500/30 hover:border-cyan-400/50
                       animate-fade-in"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
            >
                "Click me: " {count}
            </button>
        </div>
    }
}
