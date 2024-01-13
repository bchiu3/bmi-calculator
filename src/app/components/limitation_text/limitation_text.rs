use stylist::{yew::styled_component, style, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("limitation_text.scss");

#[styled_component(LimitationText)]
pub fn limitation_text() -> Html {
    let container = style!("
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 35px;
    ").expect("Failed to parse style");

    let styles = Style::new(STYLE_FILE).expect("Failed to parse style");

    html! {
        <div class={classes!(container, "text-center", styles, "md:col-span-6")}>
            <h2>{"Limitations of BMI"}</h2>
            <p>{"Although BMI is often a practical indicator of 
            healthy weight, it is not suited for every person. 
            Specific groups should carefully consider their BMI 
            outcomes, and in certain cases, the measurement may not 
            be beneficial to use."}</p>
        </div>
    }
}