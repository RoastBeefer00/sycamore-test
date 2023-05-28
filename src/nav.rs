use sycamore::prelude::*;

#[component]
pub fn Nav<'a, G: Html> (cx: Scope) -> View<G> {

    view! { cx,
        div(class="bg-slate-900 p-4 text-center border-b-2 border-b-indigo-700") {
            h1(class="text-indigo-700 lg:text-7xl text-4xl shadow") {
                "WE NEED TO COOK"
            }
        }
    }
}
