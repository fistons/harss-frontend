use leptos::*;

use crate::api::{Tokens, login_attempt};
use crate::credentials::*;

#[component]
pub fn Login<F>(cx: Scope, on_success: F) -> impl IntoView
    where
        F: Fn(Tokens) + 'static + Clone,
{
    let (message, set_message) = create_signal(cx, "Welcome");

    let login_action: Action<(String, String), ()> =
        create_action(cx, move |(login, password): &(String, String)| {
            log!("Try to login with {login}");
            let email = login.to_string();
            let password = password.to_string();
            let on_success = on_success.clone();
            async move {
     
                let result = login_attempt(&email, &password).await;
                set_message("You are logged");
                on_success(result);
            }
        });

    view! {cx,
        <p>{message}</p>
        <CredentialsForm action=login_action/>
    }
}
