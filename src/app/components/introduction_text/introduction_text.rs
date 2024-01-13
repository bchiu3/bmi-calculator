use yew::prelude::*;
use stylist::{yew::styled_component, Style, style};

const STYLE_FILE: &str = include_str!("introduction_text.scss");

#[styled_component(IntroductionText)]
pub fn introduction_text() -> Html {

    let style_file = Style::new(STYLE_FILE).expect("failed to mount stylesheet");

    let container = style!(
        r#"
            display: flex;
            gap: 35px;
            flex-direction: column;
            width: 85%;
        "#
    ).expect("failed to mount stylesheet");
    
    html! {
        <div class={classes!(style_file, container)}>
            <h1>{"Body Mass"}<br/>{"Index Calculator"}</h1>
            <p>{"Better understand your weight in relation to your height using our body mass index (BM) calculator. 
            While BMI is not the sole determinant of a healthy weight, 
            it offers a valuable starting point to evaluate your overall health and well-being."}</p>
        </div>
    }
}