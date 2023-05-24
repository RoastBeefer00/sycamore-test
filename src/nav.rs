use sycamore::prelude::*;

#[component]
pub fn Nav<'a, G: Html> (cx: Scope) -> View<G> {

    view! { cx,
        div(class="bg-slate-800 p-4 text-center") {
            h1(class="text-white text-7xl shadow") {
                "WE NEED TO COOK"
            }
        }
    }
}
