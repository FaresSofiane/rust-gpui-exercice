use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::rgb;
use gpui_kit::{Context, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div, relative};


pub struct exo1 {
    pub(crate) count: usize,
}

impl Render for exo1 {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .child(div()
                .child(
                    div()
                        .flex_col()
                        .gap_0()
                        .child(
                            div()
                                .flex()
                                .child(format!("Automatise"))
                                .text_2xl()
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(div()
                            .flex()
                            .child(format!("Prototype d'interface – exercice"))
                            .text_color(rgb(0x9c9b9a))
                        )
                )
                .child(div()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .justify_center()
                            .gap_3()
                            .child("Fichiers PDF ajoutés")
                            .child(div()
                                .child(format!("{}", self.count))
                                .text_3xl()
                                .font_weight(FontWeight::BOLD))

                            .bg(rgb(0xD4D4D8))
                            .p_4()
                            .rounded_2xl()
                            .mb_2()

                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(
                                Button::new("increment")
                                    .primary()
                                    .label("+ Ajouter un PDF")
                                    .on_click(cx.listener(|this, x1, x2, cx| {
                                        this.count += 1;
                                        cx.notify()
                                    }))
                                    .w(relative(0.85))
                                    .rounded_full()

                            )
                            .child(
                                Button::new("clear")
                                    .secondary()
                                    .label("Vider")
                                    .on_click(cx.listener(|this, x1, x2, cx| {
                                        this.count = 0;
                                        cx.notify()
                                    }))
                                    .w(relative(0.14))
                                    .rounded_full()
                            )
                    )
                    .w_full()
                    .child(
                        div()
                            .child(format!("{} PDF ajoutés à la selection", self.count))
                            .text_color(rgb(0x9c9b9a)))
                    .p_4()
                )
                .p_4()
                .border_1()
                .border_color(rgb(0x9c9b9a))
                .rounded_2xl()
            )

    }
}