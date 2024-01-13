use std::{ops::Deref, rc::Rc};

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

    let curr_unit: Rc<dyn Units>;

    match props.unit.deref() {
        CalcType::Metric(units) => match props.unit_type.as_str() {
            "Height" => {
                curr_unit = Rc::new(units.height.clone());
            }
            "Weight" => {
                curr_unit = Rc::new(units.weight.clone());
            }
            _ => {
                curr_unit = Rc::new(units.height.clone());
                log!("Invalid unit type");
            }
        },
        CalcType::Imperial(units) => match props.unit_type.as_str() {
            "Height" => {
                curr_unit = Rc::new(units.height.clone());
            }
            "Weight" => {
                curr_unit = Rc::new(units.weight.clone());
            }
            _ => {
                curr_unit = Rc::new(units.height.clone());
                log!("Invalid unit type");
            }
        },
    }

    let blur_change = {
        let props = props.clone();
        let curr_unit = Rc::clone(&curr_unit);
        Callback::from(move |e: FocusEvent| {
            let target: Option<EventTarget> = e.target();
            if let Some(target) = target {
                let input = target.dyn_into::<HtmlInputElement>().unwrap();
                let value = input.value();
                let name = input.name();
                change_fields(
                    &props.unit_type,
                    props.unit.clone(),
                    Rc::clone(&curr_unit),
                    &value,
                    &name,
                );
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
            {curr_unit.get_fields().iter().enumerate().map(|field| {
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

fn change_fields(
    unit_type: &str,
    unit: UseStateHandle<CalcType>,
    curr_unit: Rc<dyn Units>,
    value: &str,
    name: &str,
) {
    match unit_type {
        "Height" => 
        match unit.deref() {
            CalcType::Metric(units) => {
                let curr_unit = curr_unit
                    .as_any()
                    .downcast_ref::<MetricHeightUnit>()
                    .unwrap();
                let mut height = curr_unit.clone();
                height.cm = value.parse().unwrap();
                let mut new_metric = units.clone();
                new_metric.height = height;
                unit.set(CalcType::Metric(new_metric));
            }
            CalcType::Imperial(units) => {
                let curr_unit = curr_unit
                    .as_any()
                    .downcast_ref::<ImperialHeightUnit>()
                    .unwrap();
                let mut height = curr_unit.clone();
                match name {
                    "ft" => {
                        height.ft = value.parse().unwrap_or(height.ft);
                    }
                    "in" => {
                        height.inches = value.parse().unwrap_or(height.inches);
                    }
                    _ => {
                        log!("Invalid unit type");
                    }
                }
                let mut new_imperial = units.clone();
                new_imperial.height = height;
                unit.set(CalcType::Imperial(new_imperial));
            }
        },
        "Weight" => 
        match unit.deref() {
            CalcType::Metric(units) => {
                let curr_unit = curr_unit
                    .as_any()
                    .downcast_ref::<MetricWeightUnit>()
                    .unwrap();
                let mut weight = curr_unit.clone();
                weight.kg = value.parse().unwrap();
                let mut new_metric = units.clone();
                new_metric.weight = weight;
                unit.set(CalcType::Metric(new_metric));
            }
            CalcType::Imperial(units) => {
                let curr_unit = curr_unit
                    .as_any()
                    .downcast_ref::<ImperialWeightUnit>()
                    .unwrap();
                let mut weight = curr_unit.clone();
                match name {
                    "st" => {
                        weight.st = value.parse().unwrap();
                    }
                    "lbs" => {
                        weight.lbs = value.parse().unwrap();
                    }
                    _ => {
                        log!("Invalid unit type");
                    }
                }
                let mut new_imperial = units.clone();
                new_imperial.weight = weight;
                unit.set(CalcType::Imperial(new_imperial));
            }
        },
        _ => {
            log!("Invalid unit type");
        }
    }
}
