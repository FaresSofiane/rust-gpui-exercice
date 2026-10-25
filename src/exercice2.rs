use gpui_kit::assets::IconName;
use gpui_kit::component::Icon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::{
    AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled, Window,
    div, rgb,
};

pub struct exo2 {
    pub games: Vec<String>,
    pub game_input: Entity<InputState>,
}

impl exo2 {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let game_input = cx.new(|cx| InputState::new(window, cx).placeholder("Nom du jeu..."));

        Self {
            games: Vec::new(),
            game_input,
        }
    }

    pub  fn game_element(game_name: String, index: usize, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_3()
            .child(
                div()
                    .child(format!("{}", index))
                    .text_color(rgb(0x71717A))
                    .text_sm(),
            )
            .child(div().child(format!("{}", game_name)).flex_1())
            .child(
                div().child(
                    Button::new(format!("delete_game_element-{}", index))
                        .icon(IconName::Trash)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.games.remove(index);
                            cx.notify()
                        })),
                ),
            )
            .p_4()
            .bg(rgb(0xF4F4F5))
            .my_2()
            .rounded_2xl()
            .justify_center()
            .items_center()
    }

}

fn title() -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .child(div().child(Icon::new(IconName::Gamepad2)))
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


impl Render for exo2 {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(
                div()
                    .child(header(self.games.len()))
                    .children(
                        self.games
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(index, x)| div().child(Self::game_element(x, index, cx))),
                    )
                    .p_4()
                    .border_1()
                    .border_color(rgb(0xE4E4E7))
                    .rounded_2xl(),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .justify_center()
                    .mt_3()
                    .child(div().child(Input::new(&self.game_input)).flex_1())
                    .child(
                        div().child(Button::new("add_game").primary().label("Ajouter").on_click(
                            cx.listener(|this, ce, window, cx| {
                                let game_to_add = this.game_input.read(cx).value().to_string();
                                this.games.push(String::from(game_to_add.clone()));
                                this.game_input.update(cx, |x3, x4| {
                                    x3.set_value("", window, x4);
                                });
                                cx.notify()
                            }),
                        )),
                    )
                    .child(
                        div().child(Button::new("clear").danger().label("Vider").on_click(
                            cx.listener(|this, ce, window, cx| {
                                this.games.clear();
                                cx.notify()
                            }),
                        )),
                    ),
            )
            .p_4()
    }
}
