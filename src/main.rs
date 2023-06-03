use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use components::*;
use pages::articleList::*;
use pages::login::*;

use crate::api::{AuthenticatedClient, Tokens};

mod api;
mod components;
mod pages;

/// A list (more like a page) returned by the API
#[derive(Serialize, Deserialize, Clone)]
pub struct ApiList<T> {
    page: u32,
    page_size: u32,
    total_pages: u32,
    elements_number: u32,
    total_items: u32,
    content: Vec<T>,
}

/// A (lite) RSS item
#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    id: usize,
    title: String,
    url: String,
    content: String,
    channel_name: String,
}

/// Query parameters of the articles page
#[derive(Params, Debug, PartialEq, Eq)]
struct ArticleParams {
    page: u32,
    size: u32,
}

/// An Article Component
#[component]
pub fn Article(cx: Scope, article: Item) -> impl IntoView {
    view! { cx,
        <a href={article.url}><h3><b>"["{article.channel_name}"]"</b>" "{article.title}</h3></a>
        <p>{article.content}</p>
    }
}

/// Main component of the application, where the magic happens
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Build our tokens signal
    let tokens = create_rw_signal(cx, None::<Tokens>);

    // Build our client, note the tokens rw signal given in the constructor
    let (client, _) = create_signal(cx, AuthenticatedClient::new(tokens));

    // A derivative signal to indicate if the user is logged or not
    let logged = Signal::derive(cx, move || tokens().is_some());

    // Push the optional tokens in the context
    provide_context(cx, tokens);

    // Push the http client in the context
    provide_context(cx, client);

    // If a token is already present in the local storage of the browser, update the token signal
    if let Ok(stored_tokens) = LocalStorage::get("api_token") {
        tokens.update(|a| *a = Some(stored_tokens));
    }

    // Create a logout action
    let logout = create_action(cx, move |_| async move {
        if tokens().is_some() {
            tokens.update(|t| *t = None);
        }
    });

    // Logout callback
    let on_logout = move || {
        logout.dispatch(());
    };

    // Check the tokens state and update the local storage in consequence
    create_effect(cx, move |_| {
        log::debug!("Token state changed");
        match tokens.get() {
            Some(tokens) => {
                log::debug!("API is now authorized: save token in LocalStorage");
                LocalStorage::set("api_token", tokens).expect("LocalStorage::set");
            }
            None => {
                log::debug!("API is no longer authorized: delete token from LocalStorage");
                LocalStorage::delete("api_token");
            }
        }
    });

    view! { cx,
      <Router>
        <Navbar logged on_logout />
        <main>
          <Routes>
              <Route path="/login" view=move |cx| view! {cx,
                    <Login on_success= move |new_tokens: Tokens | {
                            log::debug!("This is your token! {:?}", tokens());
                            tokens.update(|t| *t = Some(new_tokens));
                        }
                    /> }
                />
              <Route path="/articles" view=|cx| view! {cx, <ArticleList /> }/>
          </Routes>
        </main>
      </Router>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| view! { cx,  <App/> })
}
