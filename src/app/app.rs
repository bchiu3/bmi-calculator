use yew::prelude::*;

use super::components::*;

use stylist::{yew::styled_component, style, Style};

const STYLE_FILE: &str = include_str!("app.scss");

#[styled_component(App)]
pub fn app() -> Html {

    let style = Style::new(STYLE_FILE).expect("Failed to parse stylesheet");

    let container = style!(
        r#"
            display: flex;
            align-self: center;
            min-height: 100vh;
            max-width: min(100vw, 1440px);
            flex-direction: column;
        "#
    ).expect("Failed to parse stylesheet");

    html! {
        <main class={classes!(style, container)}>
            <BackgroundGradient></BackgroundGradient>
            <div class={classes!("header-container", "padding-containers")}>
                <Header></Header>
                <div class={classes!("grid-top", "items-center")}>
                    <IntroductionText></IntroductionText>
                    <Calculator></Calculator>
                </div>
            </div>
            <div class={classes!("grid", "padding-containers", "container-padding-bottom-gap")}>
                <img src="images/pattern-curved-line-left.svg" class={classes!("pattern-curved-line-left")} />
                <div class={classes!("pt-10", "grid-top")}>
                    <img src="images/image-man-eating.webp" class={classes!("image-man-eating")} />
                    <DescriptionText></DescriptionText>
                </div>
            </div>
            <div class={classes!("top-description-container")}>
                <div id="top-description" class={classes!("description-grid")}>
                    <TopDescription></TopDescription>
                </div>
            </div>
            <div class={classes!("grid", "padding-containers", "limitation-container", "gap-[32px]")}>
                <LimitationText></LimitationText>
                <LimitationCards></LimitationCards>
            </div>
        </main>
    }
}
