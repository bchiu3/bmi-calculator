use std::ops::Deref;

use gloo::console::log;
use stylist::{style, yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("calculator.scss");

use super::units::*;

use super::RadioInputs;
use super::UnitInputs;
use super::BmiCalculator;

fn is_metric(unit: &CalcType) -> bool {
    match unit {
        CalcType::Metric(_) => true,
        CalcType::Imperial(_) => false,
    }
}

#[styled_component(Calculator)]
pub fn calculator() -> Html {
    let calc_styles = Style::new(STYLE_FILE).expect("failed to mount stylesheet");

    let container = style! {
        r#"
            display: flex;
            gap: 32px;
            padding: 32px;
            flex-direction: column;
            justify-content: flex-start;
            background-color: white;
            border-radius: 16px;
            box-shadow: 16px 32px 56px 0px rgba(143, 174, 207, 0.25);
        "#
    }
    .expect("failed to mount stylesheet");

    let unit = use_state(|| CalcType::Metric(MetricUnit::new()));

    // use_effect_with(unit.clone(), |unit| {
    //     log!(serde_json::to_string(unit.deref()).unwrap());
    //     || ()
    // });

    let change_unit = {
        let unit = unit.clone();
        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                input.set_checked(true);
                match input.value().as_str() {
                    "metric" => {
                        unit.set(CalcType::Metric(MetricUnit::new()));
                    }
                    "imperial" => {
                        unit.set(CalcType::Imperial(ImperialUnit::new()));
                    }
                    a => log!(a),
                };
            }
        })
    };

    html! {
        <div class={classes!(calc_styles, container)}>
            <div class={"heading"}>{"Enter your details below"}</div>
            <div class={"flex flex-row gap-[24px] flex-wrap"}>
                <RadioInputs name={"unit_type"} value={"metric"} display_name={"Metric"} id={"metric-radio"} checked={is_metric(&*unit)} change_unit={change_unit.clone()}/>
                <RadioInputs name={"unit_type"} value={"imperial"} display_name={"Imperial"} id={"imperial-radio"} checked={!is_metric(&*unit)} change_unit={change_unit.clone()}/>
            </div>
            <div class={"grid grid-cols-2 gap-[24px]"}>
                <UnitInputs unit_type={"Height"} unit={unit.clone()}/>
                <UnitInputs unit_type={"Weight"} unit={unit.clone()}/>
            </div>
            <div class={""}>
                <BmiCalculator unit={unit.clone()}/>
            </div>
        </div>
    }
}
