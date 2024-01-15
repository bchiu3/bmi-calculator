use std::{ops::Deref, rc::Rc, borrow::Borrow};

use gloo::console::log;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::app::components::calculator::units::*;

const STYLE_FILE: &str = include_str!("unit_inputs.scss");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unit_type: String,
    pub unit: UseStateHandle<CalcType>,
}

#[styled_component(UnitInputs)]
pub fn unit_inputs(props: &Props) -> Html {
    let container_style =
        Style::new(STYLE_FILE).expect("Could not load style for calculator inputs");

    let curr_unit: Box<dyn Units>;

    match props.unit.deref() {
        CalcType::Metric(units) => {
            curr_unit = Box::new(units.clone());
        }
        CalcType::Imperial(units) => {
            curr_unit = Box::new(units.clone());
        }
    }

    let curr_unit_type = match props.unit_type.as_str() {
        "Weight" => curr_unit.get_weight_fields(),
        "Height" => curr_unit.get_height_fields(),
        _ => panic!("Invalid unit type")
    };

    let blur_change = {
        let props = props.clone();
        let unit_change = props.unit.deref().clone();
        Callback::from(move |e: FocusEvent| {
            let target: Option<EventTarget> = e.target();
            if let Some(target) = target {
                let input = target.dyn_into::<HtmlInputElement>().unwrap();
                let value = input.value();
                let name = input.name();
                match props.unit.deref() {
                    CalcType::Metric(units) => {
                        if let CalcType::Metric(mut new_unit) = unit_change.clone() {
                            let value: f32 = value.parse().unwrap_or_else(|_| new_unit.get_value(&name));
                            new_unit.set_value(&name, value);
                            props.unit.set(CalcType::Metric(new_unit));
                        }
                    }
                    CalcType::Imperial(units) => {
                        if let CalcType::Imperial(mut new_unit) = unit_change.clone() {
                            let value: f32 = value.parse().unwrap_or_else(|_| new_unit.get_value(&name));
                            new_unit.set_value(&name, value);
                            props.unit.set(CalcType::Imperial(new_unit));
                        }
                    }
                }
            }
        })
    };

    // let log_stuff = {
    //     let props = props.clone();
    //     Callback::from(move |_| {
    //         log!(props.unit_type.clone());
    //         log!(serde_json::to_string(props.unit.deref()).unwrap());
    //     })
    // };

    html! {
        <>
            {curr_unit_type.iter().enumerate().map(|field| {
                    let index = field.0;
                    let field = field.1;
                    let mut curr_value = curr_unit.get_value(field).to_string();
                    if curr_value == "0" {
                        curr_value = "".to_string();
                    }
                    html! {
                        <div class={classes!("flex", "flex-col", "gap-[8px]", container_style.clone())}>
                            if index == 0 { 
                                <label for={props.unit_type.clone() + "-input"}>{props.unit_type.clone()}</label>
                            } else {
                                <div class={classes!("divider")}/>
                            }
                            <span class={classes!("input-container", "flex")}>
                                <input
                                class={classes!("input")} id={props.unit_type.clone() + "-input"}
                                type="text"
                                name={field.to_string()}
                                onblur={blur_change.clone()}
                                value={curr_value}
                                placeholder="0"
                                />
                                <span class={classes!("unit")}>{field}</span>
                            </span>
                        </div>
                    }
                })
            .collect::<Html>()}
        </>
    }
}
