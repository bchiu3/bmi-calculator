use stylist::{yew::styled_component, Style, style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("limitation_cards.scss");

struct Card<'a> {
    image: &'a str,
    header: &'a str,
    description: &'a str,
    grid_col_start: &'a str,
    grid_row_start: &'a str,
}

const CARDS: [Card; 5] = [
    Card {
        image: "/images/icon-gender.svg",
        header: "Gender",
        description: "The development and body fat composition of girls and boys vary with age. 
        Consequently, a child's age and gender are considered when evaluating their BMI.",
        grid_col_start: "8",
        grid_row_start: "1"
    },
    Card {
        image: "/images/icon-age.svg",
        header: "Age",
        description: "In aging individuals, increased body fat and muscle loss may cause BMI to underestimate body fat content.",
        grid_col_start: "5",
        grid_row_start: "2"
    },
    Card {
        image: "/images/icon-muscle.svg",
        header: "Muscle",
        description: "BMI may misclassify muscular individuals as overweight or obese, as it doesn't differentiate muscle from fat.",
        grid_col_start: "9",
        grid_row_start: "2"
    },
    Card {
        image: "/images/icon-pregnancy.svg",
        header: "Pregnancy",
        description: "Expectant mothers experience weight gain due to their growing baby. Maintaining a healthy pre-pregnancy BMI is advisable to minimise health risks for both mother and child.",
        grid_col_start: "3",
        grid_row_start: "3"
    },
    Card {
        image: "/images/icon-race.svg",
        header: "Race",
        description: "Certain health concerns may affect individuals of some Black and Asian origins at lower BMIs than others. 
        To learn more, it is advised to discuss this with your GP or practice nurse.",
        grid_col_start: "7",
        grid_row_start: "3"
    }
];

fn card_style(card: &Card) -> String {
    format!(
        "grid-column-start: {}; grid-row-start: {}",
        card.grid_col_start, card.grid_row_start
    )
}

#[styled_component(LimitationCards)]
pub fn limitation_cards() -> Html {
    let style_container = Style::new(STYLE_FILE).expect("Failed to load style");

    let limitation_cards = style!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            gap: 1rem;
            grid-column-end: span 4;
            padding: 32px;
            gap: 32px;
            border-radius: 16px;
            background: var(--Pure-White, #FFF);
            box-shadow: 16px 32px 56px 0px rgba(143, 174, 207, 0.25);

            @media only screen
            and (max-width: 768px) {
                grid-column-start: auto !important; 
                grid-column-end: auto !important;
                grid-row-start: auto !important;
            }
        "#
    )
    .expect("Failed to load style");

    html! {
        {CARDS.iter().map(|card| {
            html! {
                <div class={classes!(limitation_cards.clone(), style_container.clone())} style={card_style(card)}>
                    <div class={classes!("head")}>
                        <img src={card.image} alt={card.header} />
                        <h3>{card.header}</h3>
                    </div>
                    <p>{card.description}</p>
                </div>
            }
        }).collect::<Html>()}
    }
}
