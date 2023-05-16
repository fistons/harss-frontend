use leptos::*;

#[component]
pub fn CredentialsForm(cx: Scope, action: Action<(String, String), ()>) -> impl IntoView {
    let (password, set_password) = create_signal(cx, String::new());
    let (login, set_login) = create_signal(cx, String::new());

    let dispatch_action = move || action.dispatch((login(), password()));

    view! {cx,
        <form on:submit=move |ev| {ev.prevent_default(); dispatch_action()}>
            <p>"Please enter your credentials"</p>
            <input
              type = "text"
              required
              placeholder = "Username"
              on:keyup = move |ev: ev::KeyboardEvent| {
                let val = event_target_value(&ev);
                set_login.update(|v|*v = val);
              }
              // The `change` event fires when the browser fills the form automatically,
              on:change = move |ev| {
                let val = event_target_value(&ev);
                set_login.update(|v|*v = val);
              }
            />
            <input
              type = "password"
              required
              placeholder = "Password"
              on:keyup = move |ev: ev::KeyboardEvent| {
                let val = event_target_value(&ev);
                set_password.update(|p|*p = val);
              }
              // The `change` event fires when the browser fills the form automatically,
              on:change = move |ev| {
                let val = event_target_value(&ev);
                set_password.update(|p|*p = val);
              }
            />
            <button type="submit"
                on:click= move |_| dispatch_action()
            >"Login"</button>
        </form>
    }
}
