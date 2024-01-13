use yew::prelude::*;
use stylist::{yew::styled_component, style, Style};

pub struct Card<'a> {
    pub image: &'a str,
    pub header: &'a str,
    pub description: &'a str
}

const CARDS: [Card; 3] = [
    Card {
        image: "images/icon-eating.svg",
        header: "Healthy eating",
        description: "Healthy eating promotes weight control, 
        disease prevention, better digestion, immunity, mental clarity, 
        and mood."
    },
    Card {
        image: "images/icon-exercise.svg",
        header: "Regular exercise",
        description: "Exercise improves fitness, aids weight control, 
        elevates mood, and reduces disease risk, fostering wellness and 
        longevity."
    },
    Card {
        image: "images/icon-sleep.svg",
        header: "Adequate sleep",
        description: "Sleep enhances mental clarity, emotional stability, 
        and physical wellness, promoting overall restoration and 
        rejuvenation."
    }
];

const STYLE_FILE: &str = include_str!("top_description.scss");

#[styled_component(TopDescription)]
pub fn top_description() -> Html {
    let style = Style::new(STYLE_FILE).expect("Failed to parse style");

    let card_style = style!(
        r#"
            display: flex;
            flex-direction: column;
            gap: 24px;
            margin-top: 72px;
            margin-bottom: 72px;
        "#
    ).expect("Failed to parse style");

    html! {
        {
            CARDS.iter().map(|card| {
                html!{
                    <div class={classes!(card_style.clone(), style.clone())}>
                        <img src={card.image} alt={card.header} width="64px" height="64px" class={classes!("mb-2")}/>
                        <h1>{ card.header }</h1>
                        <p>{ card.description }</p>
                    </div>
                }
            }).collect::<Html>()
        }
    }
}