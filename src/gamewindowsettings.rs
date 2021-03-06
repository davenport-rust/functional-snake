extern crate piston_window;
extern crate sdl2_window;
extern crate find_folder;

use piston_window::*;
use sdl2_window::*;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GameWindowSettings {
    pub window_height: u32,
    pub window_width: u32,
    pub block_size: u32,
}


pub fn new_game_window(
    gws: GameWindowSettings,
    game_name: String,
) -> Option<PistonWindow<Sdl2Window>> {
    let opengl = OpenGL::V3_2;
    let ws: Option<PistonWindow<Sdl2Window>> = WindowSettings::new(
        game_name,
        (gws.window_width.clone(),
         gws.window_height.clone()),
    )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .ok();

    //    ws.map(|mut ws| {
    //        ws.set_lazy(true);
    //        ws
    //    })
    ws
}

pub fn new_glyphs(window: &mut PistonWindow<Sdl2Window>) -> Option<Glyphs> {
    let assets_opt: Option<PathBuf> = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .ok();
    let font_opt = assets_opt.map(|asset| asset.join("FiraSans-Regular.ttf"));
    let factory = window.factory.clone();

    font_opt.and_then(|f| Glyphs::new(f, factory).ok())
}
