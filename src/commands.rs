use crate::{
    data_model::{Direction, Game, MovePiece, Player, PlayerMove, WallOrientation, WallPosition},
    game_logic::is_move_legal,
};

pub fn get_human_move(game: &Game, player: Player) -> PlayerMove {
    use std::io::{self, Write};

    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Some(player_move) = parse_player_move(input, player) {
            if is_move_legal(game, player, &player_move) {
                return player_move;
            } else {
                println!("Illegal move.");
            }
        } else {
            println!("Invalid input format.");
        }
    }
}
pub fn parse_player_move(input: &str, player: Player) -> Option<PlayerMove> {
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
