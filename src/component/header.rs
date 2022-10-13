use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header {
            h1 { "David Mahany \u{2013} Portfolio" }
            p {
                "Programmer & Game Developer" br() "(Site Under Construction)"
            }
        }
    }
}
