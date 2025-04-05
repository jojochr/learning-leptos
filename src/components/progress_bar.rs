use leptos::prelude::*;

#[component]
pub fn ProgressBar(
    progress: ReadSignal<i32>,
    factor: u8
) -> impl IntoView {
    view! {
        <progress max=50 value=move || progress.get() * factor as i32 />
    }
}