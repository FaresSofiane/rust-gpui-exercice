pub mod exercice1;
pub mod exercice2;
pub mod exercice3;
pub mod exercice4;

use exercice1::exo1;
use exercice2::exo2;
use exercice3::exo3;

use gpui_kit::{AppContext, WindowOptions, application, assets, init, open_window};
use gpui_kit::component::TitleBar;
use crate::exercice3::Page;
use crate::exercice4::exo4;

fn main() {

    let games: Vec<String> = vec![
        String::from("Red Dead Redemption 2"),
        String::from("Cyberpunk 2077"),
        String::from("Minecraft"),
        String::from("GTA V"),
        String::from("Elden Ring"),
    ];

    application()
        .with_assets(assets::AllAssets)
        .run(|cx| {
            init(cx);

            open_window(
                TitleBar::window_options(),
                cx,
                |window, cx| {
                    cx.new(|cx| {
                        exo4 {}
                    })
                },
            )
                .expect("Impossible d'ouvrir la fenêtre");
        });
}