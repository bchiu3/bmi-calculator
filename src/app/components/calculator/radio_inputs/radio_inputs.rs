use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("radio_inputs.scss");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub value: String,
    pub id: String,
    pub display_name: String,
    pub checked: bool,
    pub change_unit: Callback<MouseEvent>,
}

#[styled_component(RadioInputs)]
pub fn radio_inputs(props: &Props) -> Html {

    let container_style = Style::new(STYLE_FILE).expect("Could not load style for calculator inputs");

    let call = props.change_unit.clone();

    html! {
        <div class={classes!("grow", container_style)}>
            <input type="radio" class="hidden" id={props.id.clone()} name={props.name.clone()} value={props.value.clone()} 
            checked={props.checked.clone()}
            onclick={move |e| {call.emit(e);}}/>
            <label for={props.id.clone()}>
                <div class="circle"></div>
                {props.display_name.clone()}
            </label>
        </div>
    }
}