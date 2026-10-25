use std::fmt::format;
use gpui_kit::{div, Context, IntoElement, Render, Window, ParentElement, Styled, FontWeight, rgb, relative};
use gpui_kit::assets::IconName;
use gpui_kit::component::{Icon};
use gpui_kit::prelude::FluentBuilder;

pub struct exo2 {
    pub games: Vec<String>
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
            .p_4()

    }
}