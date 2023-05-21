use sycamore::prelude::*;

#[derive(Props)]
struct Counter<'a> {
    count: &'a ReadSignal<i32>,
}

#[component]
fn Counter<'a, G: Html>(cx: Scope<'a>, props: Counter<'a>) -> View<G> {
    view! { cx,
    div(class="container text-center bg-gray-400 my-6 mx-auto p-3 w-2/3 rounded") {
        p(class="text-9xl") {
            (props.count.get())
        }
    }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {

    let count = create_signal(cx, 0);
    let increment = |_| count.set(*count.get() + 1);

    view! { cx,
        div(class="container text-center bg-gray-800 my-6 mx-auto p-3 w-2/3 rounded") {
            h1(class="text-white text-2xl") { "Hello, World" }
        }

        Counter(count=count)
        div(class="my-6 mx-auto w-2/3") {
            button(on:click=increment, class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 w-full rounded") { "+" }
        }

    }
}


fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
