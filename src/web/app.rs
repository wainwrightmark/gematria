use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::{self, prelude::*};
use crate::web::prelude::*;
use chrono::format::format;
use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
    <div class="container">
    <InputBox />
    <OutputBox />
    </div>
    }
}

#[function_component(InputBox)]
fn input_box() -> Html {
    let onchange = Dispatch::new().apply_callback(|e: Event| {
        let input = e.target_unchecked_into::<HtmlInputElement>();

        ChangeInputMsg {
            input: input.value(),
        }
    });

    html! {
        <input type="text" placeholder="Your name?" {onchange}/>
        }
}

#[function_component(OutputBox)]
fn output_box() -> Html {
    let result = use_selector(|state: &WordState| state.result.clone());

    let text = if let Some(w) = result.as_ref() {
        w.text.clone()
    } else {
        "".to_string()
    };

    html!(<h3> {text}  </h3>)
}

// #[derive(PartialEq, Eq, Properties)]
// pub struct MoveButtonProperties {
//     pub cube: CubieCube,
//     pub name: String,
// }

// #[function_component(MoveButton)]
// fn move_button(properties: &MoveButtonProperties) -> Html {
//     let cube = properties.cube.clone();
//     let onclick: Option<Callback<MouseEvent>> =
//         Some(Dispatch::new().apply_callback(move |_| MoveMsg { cube: cube.clone() }));

//     html!(<button {onclick} class="size-2 col btn-small"> {properties.name.clone()}  </button>)
// }
