use leptos::*;
use leptos_router::*;

use crate::{Article, ArticleParams};
use crate::api::AuthenticatedClient;
use crate::Item;

/// The articles list page.
#[component]
pub fn ArticleList(cx: Scope) -> impl IntoView {
    let params = use_query::<ArticleParams>(cx);

    // Page number, default to 1
    let page = move || params.with(|params| params.as_ref().map(|params| params.page).unwrap_or(1));
    // Page size, default to 20 fetch_items
    let size =
        move || params.with(|params| params.as_ref().map(|params| params.size).unwrap_or(20));

    let page_size = Signal::derive(cx, move || (page(), size()));

    // Out HTTP client
    let client = use_context::<ReadSignal<AuthenticatedClient>>(cx).unwrap();

    let res = create_local_resource(cx, page_size, move |(page, size)| async move {
        client().fetch_items(page, size).await
    });

    let fallback = move |cx, _: RwSignal<Errors>| {
        view! {cx, <p>"This is sad"</p>}
    };

    let suspense_fallback = move || {
        view! {cx, <p>"Suspense fallback"</p>}
    };

    let article_view = move || {
        res.read(cx).map(|data| {
            data.map(|b| {
                view! {cx,
                <For
                    each=move || b.content.clone()
                    key=|story| story.id
                    view=move |cx, article: Item| {
                        view! { cx,
                            <Article article/>
                        }
                    }
                />
                }
            })
        })
    };

    view! { cx,
        <ErrorBoundary fallback>
            <Transition fallback=suspense_fallback>
                 {article_view}
            </Transition>
        </ErrorBoundary>
    }
}

