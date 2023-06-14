use leptos::*;

#[component]
pub fn Pager (
    cx: Scope,
    current_page: u32,
    total_page: u32
) -> impl IntoView {

    view! {cx,
        <p>"You are on page " {move || current_page} " / " {move || total_page}</p>
    }
}
