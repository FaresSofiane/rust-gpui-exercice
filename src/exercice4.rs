use gpui_kit::{div, Context, IntoElement, Render, Window, Styled, ParentElement, rgb};
use gpui_kit::component::TitleBar;

pub struct exo4 {
    
}

impl Render for exo4 {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                TitleBar::new()
                    .bg(rgb(0x27272A))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_color(rgb(0xFFFFFF))
                            .child("Automatise")
                    )
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child("Bienvenue sur Automatise !")
            )
    }
}