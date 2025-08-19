use crate::data_model::{Game, PIECE_GRID_HEIGHT, PIECE_GRID_WIDTH, Player, WallOrientation};
use ggez::graphics::{self};
use ggez::{Context, GameResult};

const PIECE_SQUARE_SIZE: f32 = 50.0;
const WALL_THICKNESS: f32 = 15.0;
const WALL_LENGTH: f32 = 2.0 * PIECE_SQUARE_SIZE + WALL_THICKNESS;
const PIECE_RADIUS: f32 = PIECE_SQUARE_SIZE / 3.0;

enum Color {
    PlayerA,
    PlayerB,
    PieceSquare,
    Wall,
    Background,
}

impl Color {
    fn to_ggez_color(&self) -> graphics::Color {
        match self {
            Color::PlayerA => graphics::Color::from_rgb(248, 248, 248),
            Color::PlayerB => graphics::Color::from_rgb(86, 83, 82),
            Color::PieceSquare => graphics::Color::from_rgb(240, 217, 181),
            Color::Wall => graphics::Color::from_rgb(181, 136, 99),
            Color::Background => graphics::Color::from_rgb(38, 38, 38),
        }
    }
}

pub fn draw(game: &Game, ctx: &mut Context) -> GameResult {
    let mut canvas = graphics::Canvas::from_frame(ctx, Color::Background.to_ggez_color());
    for x in 0..PIECE_GRID_WIDTH {
        for y in 0..PIECE_GRID_HEIGHT {
            let screen_x = x as f32 * (PIECE_SQUARE_SIZE + WALL_THICKNESS);
            let screen_y = y as f32 * (PIECE_SQUARE_SIZE + WALL_THICKNESS);
            let rect =
                graphics::Rect::new(screen_x, screen_y, PIECE_SQUARE_SIZE, PIECE_SQUARE_SIZE);
            canvas.draw(
                &graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    Color::PieceSquare.to_ggez_color(),
                )?,
                graphics::DrawParam::default(),
            );
        }
    }
    for (i, piece_position) in game.board.player_positions.iter().enumerate() {
        let point = [
            piece_position.x() as f32 * (PIECE_SQUARE_SIZE + WALL_THICKNESS)
                + PIECE_SQUARE_SIZE / 2.0,
            piece_position.y() as f32 * (PIECE_SQUARE_SIZE + WALL_THICKNESS)
                + PIECE_SQUARE_SIZE / 2.0,
        ];
        let color = if i == Player::A.as_index() {
            Color::PlayerA
        } else {
            Color::PlayerB
        }
        .to_ggez_color();
        canvas.draw(
            &graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                point,
                PIECE_RADIUS,
                0.1,
                color,
            )?,
            graphics::DrawParam::default(),
        );
    }
    for (x, col) in game.board.walls.iter().enumerate() {
        for (y, wall) in col.iter().enumerate() {
            if let Some(wall) = wall {
                let screen_x = x as f32 * (PIECE_SQUARE_SIZE + WALL_THICKNESS) + PIECE_SQUARE_SIZE;
                let screen_y = y as f32 * (PIECE_SQUARE_SIZE + WALL_THICKNESS) + PIECE_SQUARE_SIZE;

                let rect = match wall {
                    WallOrientation::Horizontal => graphics::Rect::new(
                        screen_x - PIECE_SQUARE_SIZE,
                        screen_y,
                        WALL_LENGTH,
                        WALL_THICKNESS,
                    ),
                    WallOrientation::Vertical => graphics::Rect::new(
                        screen_x,
                        screen_y - PIECE_SQUARE_SIZE,
                        WALL_THICKNESS,
                        WALL_LENGTH,
                    ),
                };
                canvas.draw(
                    &graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        Color::Wall.to_ggez_color(),
                    )?,
                    graphics::DrawParam::default(),
                );
            }
        }
    }
    canvas.finish(ctx)
}
