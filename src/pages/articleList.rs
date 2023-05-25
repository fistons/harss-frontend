use leptos::*;
use leptos_router::*;

use crate::ArticleParams;
use crate::api::{Tokens, AuthenticatedClient};

/// The articles list page.
#[component]
pub fn ArticleList(cx: Scope) -> impl IntoView {
    let params = use_query::<ArticleParams>(cx);
    
    // Page number, default to 1
    let page = move || params.with(|params| params.as_ref().map(|params| params.page).unwrap_or(1));
    // Page size, default to 20 fetch_items
    let size = move || params.with(|params| params.as_ref().map(|params| params.size).unwrap_or(20));

    let page_size = Signal::derive(cx, move || (page(), size()));

    // Our optional `tokens`, used in the `<Show>` tag, and implicitly in our HTTP client 
    let tokens = use_context::<RwSignal<Option<Tokens>>>(cx).unwrap();
    // Out HTTP client
    let client = use_context::<ReadSignal<AuthenticatedClient>>(cx).unwrap();

    let res = create_local_resource(cx, page_size, move |(page, size)| async move {
        client().fetch_items(page, size).await
    });

    view! { cx,
            <Show when=move || tokens().is_some() fallback=|cx| view! {cx, <p>"non"</p>}>
            <Transition fallback=move || view! { cx, <p>"Loading..."</p> } >
            {move || {
                res.read(cx)
                    .map(|b| view! { cx,

            <ErrorBoundary fallback=|cx, error| view! {cx, <div>"Nope"</div>}>


                <p>"Page " {b.page} "/" {b.total_pages}</p>

                <span>
                    {move || if b.page > 1 {
                        view! {
                            cx,
                            <a class="page-link"
                                href=move || format!("/articles?page={}&size={}", page() - 1, size())
                                attr:aria_label="Previous Page">
                                "< prev"
                            </a>
                        }.into_any()
                    } else {
                        view! {
                            cx,
                            <span class="page-link disabled" aria-hidden="true">
                                "< prev"
                            </span>
                        }.into_any()
                    }}
                </span>
                "   ---    "
                <span>
                    {move || if b.page < b.total_pages {
                        view! {
                            cx,
                            <a class="page-link"
                                href=move || format!("/articles?page={}&size={}", page() + 1, size())
                                attr:aria_label="Next Page">
                                "next >"
                            </a>
                        }.into_any()
                    } else {
                        view! {
                            cx,
                            <span class="page-link disabled" aria-hidden="true">
                                "next >"
                            </span>
                        }.into_any()
                    }}
                </span>

                <For
                    each=move || b.content.clone()
                    key=|story| story.id
                    view=move |cx, article: Item| {
                        view! { cx,
                            <Article article/>
                        }
                    }
                 />
                </ErrorBoundary>
                })
            }}
            </Transition>
            </Show>
    }
}

