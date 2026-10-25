pub mod exercice1;
pub mod exercice2;

use exercice1::exo1;
use exercice2::exo2;

use gpui_kit::{AppContext, WindowOptions, application, assets, init, open_window};


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
                WindowOptions::default(),
                cx,
                |window, cx| {
                    cx.new(|cx| {
                        exo2::new(window, cx)
                    })
                },
            )
                .expect("Impossible d'ouvrir la fenêtre");
        });
}