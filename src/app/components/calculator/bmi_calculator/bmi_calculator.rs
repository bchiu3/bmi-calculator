use std::ops::Deref;

use gloo::console::log;
use stylist::{yew::styled_component, Style, style};
use yew::prelude::*;

use crate::app::components::calculator::units::CalcType;

const STYLE_FILE: &str = include_str!("bmi_calculator.scss");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unit: UseStateHandle<CalcType>,
}

#[styled_component(BmiCalculator)]
pub fn bmi_calculator(props: &Props) -> Html {
    let style_sheet = Style::new(STYLE_FILE).expect("Could not load style for calculator inputs");

    let container_style = style!(
        r#"
            display: flex;
            padding: 32px;
            flex-direction: column;
            gap: 16px;
            align-self: stretch;
            border-radius: 132px 999px 999px 132px;
            background: linear-gradient(90deg, #345FF6 0%, #587DFF 100%);
        "#
    ).expect("failed to mount stylesheet");

    let container_answer = style!(
        r#"
            display: grid;
            padding: 32px;
            gap: 16px;
            grid-template-columns: 1fr 1fr;
            align-self: stretch;
            align-items: center;
            border-radius: 132px 999px 999px 132px;
            background: linear-gradient(90deg, #345FF6 0%, #587DFF 100%);
        "#
    ).expect("failed to mount stylesheet");

    let call = props.unit.clone();

    let bmi = use_state(|| -1.0);
    let bmi_text = use_state(|| String::from(""));
    let bmi_text_bold = use_state(|| String::from(""));


    let effect_unit = {
        let bmi = bmi.clone();
        move |call: &UseStateHandle<CalcType>| {
            match call.deref() {
                CalcType::Metric(unit) => {
                    let height = unit.height.cm;
                    let weight = unit.weight.kg;
                    if height == 0.0 {
                        bmi.set(-1.0);
                        return;
                    }
                    let bmi_val = weight / ((height / 100.0) * (height / 100.0));
                    bmi.set(bmi_val);
                }
                CalcType::Imperial(unit) => {
                    let height = unit.height.ft * 12.0 + unit.height.inches;
                    let weight = unit.weight.st * 14.0 + unit.weight.lbs;
                    if height == 0.0 {
                        bmi.set(-1.0);
                        return;
                    }
                    let bmi_val = weight * 703.0 / (height * height);
                    bmi.set(bmi_val);
                }
            }
        }
    };

    let effect_bmi = {
        let bmi_text = bmi_text.clone();
        let bmi_text_bold = bmi_text_bold.clone();
        let call = call.clone();
        move |bmi: &UseStateHandle<f32>| {
            if *bmi.deref() == -1.0 {
                return;
            }

            let ideal_weights = calc_ideal_weight(&call);
            let ideal_weights_text;
            match call.deref() {
                CalcType::Metric(_) => {
                    ideal_weights_text = format!("{:.1}kg - {:.1}kg", ideal_weights.0 .0, ideal_weights.1 .0);
                }
                CalcType::Imperial(_) => {
                    ideal_weights_text = format!("{:.1}st {:.1}lbs - {:.1}st {:.1}lbs", 
                    ideal_weights.0 .0, ideal_weights.0 .1, ideal_weights.1 .0, ideal_weights.1 .1);
                }
            }
            bmi_text_bold.set(ideal_weights_text);

            let bmi_new_text;
            if *bmi.deref() < 18.5 {
                bmi_new_text = format!("Your BMI suggests you’re underweight. Your ideal weight is between ");
            }
            else if *bmi.deref() >= 18.5 && *bmi.deref() < 24.9 {
                bmi_new_text = format!("Your BMI suggests you’re a healthy weight. Your ideal weight is between ");
            }
            else if *bmi.deref() >= 24.9 && *bmi.deref() < 29.9 {
                bmi_new_text = format!("Your BMI suggests you’re overweight. Your ideal weight is between ");
            }
            else {
                bmi_new_text = format!("Your BMI suggests you’re obese. Your ideal weight is between ");
            }
            bmi_text.set(bmi_new_text);
        }
    };

    use_effect_with(call.clone(), effect_unit);
    
    use_effect_with(bmi.clone(), effect_bmi);

    html! {
        if *bmi == -1.0 {
            <div class={classes!(style_sheet, container_style)}>
                <h3>{ "Welcome!" }</h3>
                <p>{"Enter your height and weight and you’ll see your BMI result here"}</p>
            </div>
        }  else {
            <div class={classes!(style_sheet, container_answer)}>
                <div class="head">
                    <h3 class="bmi-header">{ "Your BMI is" }</h3>
                    <p class="bmi"><b>{ format!("{:.1}", *bmi) }</b></p>
                </div>
                <p class="bmi-text">{bmi_text.deref()} <b> {bmi_text_bold.deref()} </b></p>
            </div>
        }
    }
}

pub fn calc_ideal_weight(call: &UseStateHandle<CalcType>) -> ((f32, f32), (f32, f32)) {
    let bmi_low_val = 18.5;
    let bmi_high_val = 24.9;
    match call.deref() {
        CalcType::Metric(unit) => {
            let height = unit.height.cm;
            // let weight = unit.weight.kg;
            let ideal_low_weight = bmi_low_val * ((height / 100.0) * (height / 100.0));
            let ideal_high_weight = bmi_high_val * ((height / 100.0) * (height / 100.0));
            
            ((ideal_low_weight, 0.0), (ideal_high_weight, 0.0))
        }
        CalcType::Imperial(unit) => {
            let height = unit.height.ft * 12.0 + unit.height.inches;
            // let weight = unit.weight.st * 14.0 + unit.weight.lbs;
            let ideal_low_weight = bmi_low_val * ((height * height)/ 703.0);
            let ideal_high_weight = bmi_high_val * ((height * height)/ 703.0);
            
            (((ideal_low_weight / 14.0).floor(), (ideal_low_weight % 14.0).floor()), 
            ((ideal_high_weight / 14.0).floor(), (ideal_high_weight % 14.0).floor()))
        }
    }
}