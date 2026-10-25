pub mod exercice1;
pub mod exercice2;

use exercice1::exo1;
use exercice2::exo2;

use gpui_kit::{AppContext, WindowOptions, application, assets, init, open_window};


fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx| {
            init(cx);

            open_window(
                WindowOptions::default(),
                cx,
                |_, cx| {
                    cx.new(|_| exo1 {
                        count: 0,
                    })
                },
            )
                .expect("Impossible d'ouvrir la fenêtre");
        });
}