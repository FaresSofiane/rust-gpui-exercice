use gpui_kit::{div, Context, IntoElement, Render, Window, ParentElement, Styled};
use gpui_kit::component::button::Button;

pub enum Page {
    Home,
    Settings
}
pub struct exo3 {
    pub current_page: Page
}

impl exo3 {
    fn render_page(&mut self) -> impl IntoElement {
        match self.current_page {
            Page::Home => div()
                .child("Home"),
            Page::Settings => div()
                .child("Settings"),
        }
    }
}

impl Render for exo3 {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {


        div()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .child(Button::new("button_home")
                        .label("Accueil")
                        .on_click(cx.listener(move |x, x1, x2, x3| {
                            x.current_page = Page::Home;
                            x3.notify()
                        }))
                        .flex_1())
                    .child(Button::new("button_settings")
                        .label("Réglages")
                        .on_click(cx.listener(move |x, x1, x2, x3| {
                            x.current_page = Page::Settings;
                            x3.notify()
                        }))
                        .flex_1()
                    )
            
            )
            .child(self.render_page())

    }

}