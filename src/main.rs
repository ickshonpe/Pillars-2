#![allow(dead_code)]
mod assets;
mod board;
mod drawing;
mod exploding_blocks;
mod game_is_over;
mod game_loop;
mod helpful_things;
mod input;
mod magic_puffs;
mod matching_blocks;
mod pillar;
mod scoring;
mod the_pillar_descending;
mod the_pillar_has_landed;
mod the_rules;
mod timer;
mod title_screen;

use std::{env, path};

use ggez::*;

fn main() -> GameResult {
    let (mut context, event_loop) = {
        let window_setup = ggez::conf::WindowSetup {
            title: "Pillars".to_owned(),
            vsync: true,
            ..Default::default()
        };
        let window_mode = ggez::conf::WindowMode {
            width: 320.0,
            height: 736.0,
            ..Default::default()
        };
        let context_builder = ContextBuilder::new("Pillars", ":/")
            .window_mode(window_mode)
            .window_setup(window_setup);
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let resource_path = path::PathBuf::from(manifest_dir).join("resources");
            context_builder.add_resource_path(resource_path)
        } else {
            context_builder
        }
        .build()?
    };

    let event_handler = game_loop::GameLoop::new(&mut context)?;
    event::run(context, event_loop, event_handler)
}
