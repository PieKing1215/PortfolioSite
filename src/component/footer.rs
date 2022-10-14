use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        footer {
            p {
                "\u{00a9} David Mahany" br()
                "Created by me in Rust using the Perceus framework" br()
                a(href="https://github.com/PieKing1215/PortfolioSite") {
                    "Source repository on GitHub"
                }
            }
        }
    }
}
