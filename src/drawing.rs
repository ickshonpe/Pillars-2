use ggez::{
    graphics::{
        self, spritebatch::SpriteBatch, Color, DrawMode, Drawable, Image, MeshBuilder, Rect,
        StrokeOptions, Text, WHITE,
    },
    Context, GameResult,
};
use glam::{vec2, vec4, Vec2};

use crate::{
    assets::Assets,
    board::Board,
    game_loop::CommonState,
    helpful_things::{Center, HalfSizeCtx},
    pillar::Pillar,
};

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}

pub const RED: Color = rgb(1.0, 0.0, 0.0);
pub const GREEN: Color = rgb(0.0, 1.0, 0.0);
pub const BLUE: Color = rgb(0.0, 0.0, 1.0);
pub const ORANGE: Color = rgb(1.0, 69.0 / 255.0, 1.0);
pub const PURPLE: Color = rgb(1.0, 0.0, 1.0);
pub const CYAN: Color = rgb(0.0, 1.0, 1.0);
pub const YELLOW: Color = rgb(1.0, 1.0, 0.0);
pub const VIOLET: Color = rgb(0.5, 0.0, 1.0);
pub const GOLD: Color = rgb(212. / 255., 175. / 255., 37. / 255.);

#[derive(Clone, Copy, Debug)]
pub struct DrawingPlans {
    pub board_pos: Vec2,
    pub board_rect: Rect,
    pub board_ctr: Vec2,
    pub spawn_rect: Rect,
    pub line_of_death: f32,
}

impl DrawingPlans {
    pub fn new(
        ctx: &mut Context,
        tile_sz: Vec2,
        board_sz: [usize; 2],
        spawn_pt: (usize, f32),
        pillar_sz: usize,
        offset: Vec2,
    ) -> Self {
        let board_width = board_sz[0] as f32 * tile_sz.x;
        let board_height = board_sz[1] as f32 * tile_sz.y;
        let board_ctr = 0.5 * vec2(board_width, board_height);
        let board_x = graphics::screen_coordinates(ctx).center().x - board_ctr.x;
        let board_y = 16.0; // + (1 + pillar_sz) as f32 * tile_sz.x;
        let board_pos = vec2(board_x, board_y) + offset;

        let board_rect = Rect {
            x: 0.,
            y: 0.,
            w: board_width,
            h: board_height,
        };
        let line_of_death = (pillar_sz + 1) as f32 * tile_sz.y;
        let spawn_rect = Rect {
            x: spawn_pt.0 as f32 * tile_sz.x,
            y: spawn_pt.1 * tile_sz.y - pillar_sz as f32 * tile_sz.y,
            w: 1.0 * tile_sz.x,
            h: pillar_sz as f32 * tile_sz.y,
        };
        Self {
            board_pos,
            board_rect,
            board_ctr,
            spawn_rect,
            line_of_death,
        }
    }
}

pub fn draw_borders(ctx: &mut Context, plans: DrawingPlans, thickness: f32) -> GameResult {
    let m = 2.0 * vec2(thickness, thickness);
    let board_lines = expand_rect(plans.board_rect, m);
    let life_lines = Rect {
        y: board_lines.y + plans.line_of_death,
        h: board_lines.h - plans.line_of_death,
        ..board_lines
    };
    let spawn_lines = expand_rect(plans.spawn_rect, m);
    let stroke_options = StrokeOptions::default().with_line_width(thickness);
    let mut primitives_mesh_builder = MeshBuilder::new();
    primitives_mesh_builder.rectangle(DrawMode::Stroke(stroke_options), life_lines, BLUE);
    primitives_mesh_builder.rectangle(DrawMode::Stroke(stroke_options), spawn_lines, RED);
    let mesh = primitives_mesh_builder.build(ctx)?;
    graphics::draw(ctx, &mesh, (plans.board_pos,))?;
    Ok(())
}

pub fn expand_rect(r: Rect, e: Vec2) -> Rect {
    Rect {
        x: r.x - e.x,
        y: r.y - e.y,
        w: r.w + e.x * 2.0,
        h: r.h + e.y * 2.0,
    }
}

pub fn draw_board(board: &Board, tile_sz: Vec2, tile_image: &Image) -> impl Drawable {
    let mut sprite_batch = SpriteBatch::new(tile_image.clone());
    for x_idx in 0..board.x_len() {
        for y_idx in 0..board.y_len() {
            if let Some(block) = board[[x_idx, y_idx]] {
                let target = vec2(x_idx as f32 * tile_sz.x, y_idx as f32 * tile_sz.y);
                sprite_batch.add((target, block.color()));
            }
        }
    }
    sprite_batch
}

pub fn draw_board_with_highlight(
    board: &Board,
    tile_sz: Vec2,
    tile_image: Image,
    highlighted: &[[usize; 2]],
    s: f32,
    h: Color,
) -> impl Drawable {
    let mut sprite_batch = SpriteBatch::new(tile_image);
    for x_idx in 0..board.x_len() {
        for y_idx in 0..board.y_len() {
            let idx = [x_idx, y_idx];
            if let Some(block) = board[idx] {
                let color: Color = if highlighted.contains(&idx) {
                    let c = block.color();
                    // let cv = vec4(c.r, c.g, c.b, c.a);
                    // let h = vec4(h.r, h.g, h.b, h.a);
                    // let r = cv.lerp(h, s);
                    // Color::new(r.x, r.y, r.z, r.w)
                    lerp_color(c, h, s)
                } else {
                    block.color()
                };
                let target = vec2(x_idx as f32 * tile_sz.x, y_idx as f32 * tile_sz.y);
                sprite_batch.add((target, color));
            }
        }
    }
    sprite_batch
}

pub fn lerp_color(c: Color, d: Color, s: f32) -> Color {
    let cv = vec4(c.r, c.g, c.b, c.a);
    let dv = vec4(d.r, d.g, d.b, d.a);
    let r = cv.lerp(dv, s);
    Color::new(r.x, r.y, r.z, r.w)
}

pub fn draw_game_play(
    ctx: &mut Context,
    assets: &Assets,
    common: &CommonState,
    pillar: Option<(&Pillar, usize, f32)>,
    highlights: Option<(&[[usize; 2]], f32)>,
) -> GameResult {
    let board = &common.board;
    let tile_sz = assets.tile_sz;
    let plans = DrawingPlans::new(
        ctx,
        tile_sz,
        board.sz(),
        common.rules.pillar_spawn_pt.into(),
        common.rules.pillar_sz,
        vec2(0.0, 0.0),
    );
    draw_borders(ctx, plans, 3.0)?;
    let mut sprite_batch = SpriteBatch::new(assets.block_image.clone());
    let next_pillar_pos = vec2(
        common.rules.pillar_spawn_pt.x as f32 * tile_sz.x,
        (common.rules.pillar_spawn_pt.y - common.rules.pillar_sz as f32) * tile_sz.y,
    );
    for (row, block) in common.next_pillar.iter().enumerate() {
        let target = next_pillar_pos + vec2(0.0, row as f32 * tile_sz.y);
        sprite_batch.add((target, block.color()));
    }

    if let Some((pillar, x, y)) = pillar {
        let pillar_pos = vec2(
            x as f32 * tile_sz.x,
            (y - common.rules.pillar_sz as f32) * tile_sz.y,
        );
        for (row, block) in pillar.iter().enumerate() {
            let target = pillar_pos + vec2(0.0, row as f32 * tile_sz.y);
            sprite_batch.add((target, block.color()));
        }
    }
    graphics::draw(ctx, &sprite_batch, (plans.board_pos,))?;

    if let Some((matches, s)) = highlights {
        let d = &draw_board_with_highlight(
            board,
            tile_sz,
            assets.block_image.clone(),
            matches,
            s,
            WHITE,
        );
        graphics::draw(ctx, d, (plans.board_pos,))?;
    } else {
        graphics::draw(
            ctx,
            &draw_board(board, tile_sz, &assets.block_image),
            (plans.board_pos,),
        )?;
    }

    let level_message = graphics::Text::new((format!("level {}", common.level), assets.font, 25.0));
    let target = vec2(
        graphics::screen_coordinates(ctx).center().x - level_message.half_sz(ctx).x,
        plans.board_pos.y + plans.board_rect.bottom() as f32 + 15.0
    );
    graphics::draw(ctx, &level_message, (target,))?;
    Ok(())
}

pub fn draw_centered_text(ctx: &mut Context, t: &Text, y: f32, color: Color) -> GameResult {
    let x = graphics::screen_coordinates(ctx).center().x - 0.5 * t.width(ctx) as f32;
    let v = vec2(x, y);
    graphics::draw(ctx, t, (v, color))
}
