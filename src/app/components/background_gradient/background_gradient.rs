use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component(BackgroundGradient)]
pub fn background_gradient() -> Html {
    let background = style!{
        r#"
        position:absolute;
        border-radius: 0px 0px 35px 35px;
        background: linear-gradient(-45deg, rgba(214,230,254,1) 0%, rgba(214,252,254,.5) 50%,  rgba(214,252,254,0) 100%);
        height: var(--gradient-height-mobile);
        width: min(100vw, 978px);
        z-index: -1;

        @media only screen 
        and (min-width : 1025px) {
            height: var(--gradient-height);
            margin-left: 24px;
        }
        "#
    }.expect("failed to mount stylesheet");

    html! {
        <div class={classes!(background)}>
        </div>
    }
}