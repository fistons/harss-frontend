use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar<F>(cx: Scope, logged: Signal<bool>, on_logout: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    view! {cx,
        <nav>
            <Show
                when = logged
                fallback= |cx| view! {cx,
                    <A href="/login">"Login"</A>
                }>
                <p>"You are logged"</p>
                <ul>
                    <li><A href="/articles">"Articles"</A></li>
                    <li>
                        <A href="#" on:click={
                            let on_logout = on_logout.clone();
                            move |_| on_logout()
                          }>"Logout"
                        </A>
                    </li>
                </ul>
            </Show>
        </nav>
    }
}
