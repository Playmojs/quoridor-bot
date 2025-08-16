use crate::data_model::{
    Direction, Game, MovePiece, Player, PlayerMove, WallOrientation, WallPosition,
};

pub mod a_star;
pub mod all_moves;
pub mod bot;
pub mod data_model;
pub mod game_logic;
pub mod render_board;
pub mod square_outline_iterator;
fn main() {
    let mut game = Game::new();
    let mut player = Player::A;
    loop {
        println!("{}", render_board::render_board(&game.board));
        println!(
            "{} to move. Walls: A: {}, B: {}",
            player.to_string(),
            game.walls_left[Player::A.as_index()],
            game.walls_left[Player::B.as_index()]
        );

        let player_move = match player {
            Player::A => {
                let start_time = std::time::Instant::now();
                let (score, best_move) = bot::best_move_alpha_beta(&game, player, 2);
                let elapsed = start_time.elapsed();
                println!(
                    "Best move: {:?} with score: {} (took {:?})",
                    best_move, score, elapsed
                );
                best_move.unwrap()
            }
            Player::B => get_human_move(&game, player),
        };
        game_logic::execute_move_unchecked(&mut game, player, &player_move);
        player = player.opponent();
        render_board::render_board(&game.board);
    }
}

fn get_human_move(game: &Game, player: Player) -> PlayerMove {
    use std::io::{self, Write};

    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Some(player_move) = parse_player_move(input, player) {
            if game_logic::is_move_legal(game, player, &player_move) {
                return player_move;
            } else {
                println!("Illegal move.");
            }
        } else {
            println!("Invalid input format.");
        }
    }
}

fn parse_player_move(input: &str, player: Player) -> Option<PlayerMove> {
    let mut chars = input.chars();

    let direction_from_char = |c: Option<char>| match c {
        Some('u') => Some(Direction::Up),
        Some('d') => Some(Direction::Down),
        Some('l') => Some(Direction::Left),
        Some('r') => Some(Direction::Right),
        _ => None,
    };

    let default_direction = match player {
        Player::A => Direction::Down,
        Player::B => Direction::Up,
    };
    match chars.next() {
        Some('m') => Some(PlayerMove::MovePiece(MovePiece {
            direction: direction_from_char(chars.next())?,
            direction_on_collision: direction_from_char(chars.next()).unwrap_or(default_direction),
        })),
        Some('h') => match (chars.next(), chars.next()) {
            (Some(x), Some(y)) => {
                let x = x.to_digit(10)? as usize;
                let y = y.to_digit(10)? as usize;
                Some(PlayerMove::PlaceWall {
                    orientation: WallOrientation::Horizontal,
                    position: WallPosition { x, y },
                })
            }
            _ => None,
        },
        Some('v') => match (chars.next(), chars.next()) {
            (Some(x), Some(y)) => {
                let x = x.to_digit(10)? as usize;
                let y = y.to_digit(10)? as usize;
                Some(PlayerMove::PlaceWall {
                    orientation: WallOrientation::Vertical,
                    position: WallPosition { x, y },
                })
            }
            _ => None,
        },
        _ => None,
    }
}
