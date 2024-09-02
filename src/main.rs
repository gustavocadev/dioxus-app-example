#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
      Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
      Link { to: Route::Home {}, "Go to counter" }
      "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut todos: Signal<Vec<String>> =
        use_signal(|| vec!["Learn Dioxus".to_string(), "Build an app".to_string()]);
    let mut new_todo = use_signal(|| "".to_string());

    rsx! {
      main { class: "container mx-auto p-4",
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        h1 { class: "text-2xl font-bold", "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        div {
          "This is a simple example of a counter app using Dioxus."
          "It uses the `use_signal` hook to manage the state of the counter."
          "The counter is shared between the `Home` and `Blog` components."
        }
        h2 { class: "text-2xl font-bold", "Todos" }
        form {
          class: "flex gap-2",
          onsubmit: move |_| {
              todos.write().push(new_todo());
              new_todo.set("".to_string());
          },
          input {
            class: "border border-gray-300 p-2 rounded",
            r#type: "text",
            value: new_todo(),
            oninput: move |e| new_todo.set(e.value())
          }
          button {
            class: "p-2 bg-blue-500 text-white rounded",
            r#type: "submit",
            "Add Todo"
          }
        }
        ul { class: "list-disc",
          for i in todos() {
            li { "{i}" }
          }
        }
      }
    }
}
