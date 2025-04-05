use crate::components::progress_bar::ProgressBar;
use leptos::prelude::*;
/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count): (ReadSignal<i32>, WriteSignal<i32>) = signal(0);
    let doubled_count = move || count.get() * 2;

    view! {
            <button on:click=move |_| set_count.set(count.get() + increment)
                    style:color=move || format!("rgb({}, {}, 100)", count.get(), 100)
                    style:background-color="green"
            >

                "Click me: " {count}
            </button>
    <ProgressBar progress=count factor=2 />
        }
}
