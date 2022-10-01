use components::counter::{self, Counter};
use components::my_component::{self, MyComponent};
use sycamore::prelude::*;

mod components;

fn main() {
    sycamore::render(|cx| {
        let state = create_signal(cx, 0);

        view! { cx,
            MyComponent(my_component::MyProps { name: String::from("John Doe"), email: "john.doe@doe.com".to_owned() })
            Counter(counter::MyProps { state: state })
        }
    });
}
