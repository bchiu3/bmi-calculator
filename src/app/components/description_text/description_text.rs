use stylist::{yew::styled_component, style, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("description_text.scss");


#[styled_component(DescriptionText)]
pub fn description_text() -> Html {
    let style_file = Style::new(STYLE_FILE).expect("failed to mount stylesheet");

    let container = style!(
        r#"
            display: flex;
            flex-direction: column;
            gap: 35px;
            width:85%;
        "#
    ).expect("failed to mount stylesheet");
    
    html! {
        <div class="flex justify-end items-center">
            <div class={classes!(style_file, container, "pt-40")}>
                <h2>{"What your BMI result means"}</h2>
                <p>{"A BMI range of 18.5 to 24.9 is considered a 'healthy weight.' 
                Maintaining a healthy weight may lower your chances of experiencing 
                health issues later on, such as obesity and type 2 diabetes. 
                Aim for a nutritious diet with reduced fat and sugar content, 
                incorporating ample fruits and vegetables. Additionally, strive for 
                regular physical activity, ideally about 30 minutes daily for five days a week."} </p>
            </div>
        </div>
    }
}