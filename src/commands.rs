use clap::Parser;

use crate::{
    bot::best_move_alpha_beta,
    data_model::{Direction, Game, MovePiece, Player, PlayerMove, WallOrientation, WallPosition},
    game_logic::{execute_move_unchecked, is_move_legal},
};

#[derive(clap_derive::Subcommand, Debug)]
pub enum AuxCommand {
    Reset {
        #[arg(short, long)]
        skip_initial_moves: bool,
    },
    BotMove {
        #[arg(default_value_t = 4)]
        depth: usize,
    },
    PlayBotMove {
        #[arg(default_value_t = 4)]
        depth: usize,
    },
    Undo {
        #[arg(default_value_t = 1)]
        moves: usize,
    },
}
const AUX_COMMAND_NAME: &str = "aux";

#[derive(clap_derive::Parser, Debug)]
#[command(name = AUX_COMMAND_NAME)]
struct AuxCommandParserHelper {
    #[command(subcommand)]
    command: AuxCommand,
}

pub enum Command {
    PlayMove(PlayerMove),
    AuxCommand(AuxCommand),
}

pub struct Session {
    pub game_states: Vec<Game>,
}

pub fn execute_command(session: &mut Session, command: Command) {
    let current_game_state = session.game_states.last().unwrap();
    let player = current_game_state.player;
    match command {
        Command::PlayMove(player_move) => {
            let mut next_game_state = current_game_state.clone();
            execute_move_unchecked(&mut next_game_state, player, &player_move);
            session.game_states.push(next_game_state);
        }
        Command::AuxCommand(aux_command) => match aux_command {
            AuxCommand::Reset { skip_initial_moves } => {
                let next_game_state = if skip_initial_moves {
                    Game::new_with_initial_moves_skipped()
                } else {
                    Game::new()
                };
                session.game_states.push(next_game_state);
            }
            AuxCommand::BotMove { depth } => {
                let _ = get_bot_move(current_game_state, player, depth);
            }
            AuxCommand::PlayBotMove { depth } => {
                let bot_move = get_bot_move(current_game_state, player, depth);
                let mut next_game_state = current_game_state.clone();
                execute_move_unchecked(&mut next_game_state, player, &bot_move);
                session.game_states.push(next_game_state);
            }
            AuxCommand::Undo { moves } => {
                for _ in 0..moves {
                    if session.game_states.len() == 1 {
                        break;
                    }
                    session.game_states.pop();
                }
            }
        },
    }
}

pub fn parse_command(input: &str) -> Option<Command> {
    match AuxCommandParserHelper::try_parse_from(
        std::iter::once(AUX_COMMAND_NAME).chain(input.split_whitespace()),
    ) {
        Ok(aux_command_parser_helper) => {
            Some(Command::AuxCommand(aux_command_parser_helper.command))
        }
        Err(_) => Some(Command::PlayMove(parse_player_move(input)?)),
    }
}

pub fn get_legal_command(game: &Game, player: Player) -> Command {
    use std::io::{self, Write};

    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Some(command) = parse_command(input) {
            if matches!(&command, Command::PlayMove(player_move) if !is_move_legal(game, player, player_move))
            {
                println!("Invalid move.")
            } else {
                break command;
            }
        } else {
            println!("Invalid input format.");
        }
    }
}
pub fn parse_player_move(input: &str) -> Option<PlayerMove> {
    let mut chars = input.chars();

    let direction_from_char = |c: Option<char>| match c {
        Some('u') => Some(Direction::Up),
        Some('d') => Some(Direction::Down),
        Some('l') => Some(Direction::Left),
        Some('r') => Some(Direction::Right),
        _ => None,
    };

    match chars.next() {
        Some('m') => {
            let direction = direction_from_char(chars.next())?;
            let direction_on_collision = direction_from_char(chars.next()).unwrap_or(direction);
            Some(PlayerMove::MovePiece(MovePiece {
                direction,
                direction_on_collision,
            }))
        }
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

fn get_bot_move(game: &Game, player: Player, depth: usize) -> PlayerMove {
    let start_time = std::time::Instant::now();
    let (score, best_move) = best_move_alpha_beta(game, player, depth);
    let elapsed = start_time.elapsed();
    let best_move = best_move.unwrap();
    println!(
        "Best move: {} with score: {} (took {:?})",
        best_move, score, elapsed
    );
    best_move
}
