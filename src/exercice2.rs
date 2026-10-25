use gpui_kit::{div, Context, IntoElement, Render, Window, ParentElement, Styled, FontWeight, rgb, relative, AppContext, Entity};
use gpui_kit::assets::IconName;
use gpui_kit::component::{Icon};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};

pub struct exo2 {
    pub games: Vec<String>,
    pub game_input: Entity<InputState>
}

impl exo2 {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let game_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Nom du jeu...")
        });

        Self {
            games: Vec::new(),
            game_input
        }
    }

}

fn title() -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .child(div()
            .child(Icon::new(IconName::Gamepad2)))
        .child("Ma bibliothèque")
        .font_weight(FontWeight::SEMIBOLD)
}

fn game_len(number: usize) -> impl IntoElement {
    div()
        .flex()
        .child(format!("{} jeux", number))
        .p_1()
        .bg(rgb(0xf4f4f4))
        .items_center()
        .justify_center()
        .rounded_full()
}

fn header(number: usize) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .justify_between()
        .child(title())
        .child(game_len(number))

}

fn game_element(game_name: String, index: usize) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .child(
            div()
            .child(format!("{}", index))
                .w(relative(0.015))
                .text_color(rgb(0x71717A))
                .text_sm()
        )
        .child(div()
            .child(format!("{}", game_name))
            .flex_1())
        .p_4()
        .bg(rgb(0xF4F4F5))
        .my_2()
        .rounded_2xl()
        .justify_center()
        .items_center()



}

impl Render for exo2 {


    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        div()
            .child(
                div()
                    .child(header(self.games.len()))
                    .children(
                        self.games.iter().cloned()
                            .enumerate()
                            .map( |(index, x)| {div().child(game_element(x, index))})
                    )
                    .p_4()
                    .border_1()
                    .border_color(rgb(0xE4E4E7))
                    .rounded_2xl()
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .justify_center()
                    .mt_3()
                    .child(
                        div()
                            .child(Input::new(&self.game_input))
                            .flex_1()
                    )
                    .child(div()
                        .child(
                            Button::new("add_game")
                                .primary()
                                .label("Ajouter")
                                .on_click(cx.listener(|this, ce, window, cx| {
                                    let game_to_add = this.game_input.read(cx).value().to_string();
                                    this.games.push(String::from(game_to_add.clone()));
                                    this.game_input.update(cx, |x3, x4| {
                                        x3.set_value("", window, x4);
                                    });
                                    cx.notify()
                                }))
                        ))

                    .child(div()
                        .child(
                            Button::new("clear")
                                .danger()
                                .label("Vider")
                                .on_click(cx.listener(|this, ce, window, cx| {
                                   this.games.clear();
                                    cx.notify()
                                }))
                        ))
                
            )

            .p_4()

    }
}