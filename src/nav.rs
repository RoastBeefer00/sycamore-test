use sycamore::prelude::*;

#[component]
pub fn Nav<'a, G: Html> (cx: Scope) -> View<G> {

    view! { cx,
        div(class="bg-purple-900 p-2 text-center") {
            h1(class="text-white text-9xl") {
                "WE NEED TO COOK"
            }
        }
    }
}
