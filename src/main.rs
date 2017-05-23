extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate sdl2_window;

use piston_window::*;
use rand::*;
use sdl2_window::Sdl2Window;

pub static WINDOW_HEIGHT: usize = 480;
pub static WINDOW_WIDTH: usize = 640;
pub static BLOCK_SIZE: usize = 10; // NOTE: WINDOW_HEIGHT and WINDOW_WIDTH should be divisible by this

pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];


fn main() {
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Hello Piston!", (640, 480))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);

    let ref font = assets.join("FiraSans-Regular.ttf");
    println!("{:?}", font);
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();
    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(
            &e, |c, g| {
                let transform = c.transform.trans(10.0, 100.0);

                clear(GREEN, g);
                text::Text::new_color(RED, 32)
                    .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g);
            }
        );
    }
}


// fn main() {
//     let opengl = OpenGL::V3_2;
//
//     let mut window: PistonWindow = WindowSettings::new("FunctionalSnake", [500, 700])
//         .opengl(opengl)
//         .exit_on_esc(true)
//         .build()
//         .unwrap();
//
//     // let mut glyphs = Glyphs::new(font, window.factory.clone()).unwrap();
//
//     while let Some(e) = window.next() {
//         match e {
//             Input::Release(Button::Keyboard(key)) => {
//                 match key {
//                     // Key::W => game.player.moving(0, -1),
//                     // Key::S => {
//                     //     game.player.moving(0, 1);
//                     // }
//                     // Key::A => {
//                     //     game.player.moving(-1, 0);
//                     // }
//                     // Key::D => {
//                     //     game.player.moving(1, 0);
//                     // }
//                     // Key::M => {
//                     //     game.player.throw();
//                     // }
//                     _ => {}
//                 }
//             }
//
//             Input::Update(args) => {
//                 // if game.scene == 2 {
//                 //     game.update(args.dt);
//                 //     game.check_collision();
//                 // }
//             }
//
//             Input::Render(_) => {
//                 // window.draw_2d(
//                 //     &e, |c, g| {
//                 //         clear(GREEN, g);
//                 //         image(&house_start, c.transform.scale(0.5, 0.5), g);
//                 //     }
//                 // );
//             }
//             _ => {}
//         }
//     }
// }
