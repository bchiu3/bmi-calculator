use stylist::style;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let container = style!(
        r#"
            margin: 75px 0px;
        "#
    ).expect("failed to parse style");
    
    html! {
        <img class={container} src="images/logo.svg" alt="logo"/>
    }
}

