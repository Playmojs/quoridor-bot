use clap::Parser;

use crate::{
    commands::{Command, Session, execute_command, get_legal_command},
    data_model::{Game, Player, PlayerMove},
    player_type::PlayerType,
};

pub mod a_star;
pub mod all_moves;
pub mod bot;
pub mod commands;
pub mod data_model;
pub mod game_logic;
pub mod player_type;
pub mod render_board;
pub mod square_outline_iterator;

#[derive(clap_derive::Parser, Debug)]
struct Args {
    #[clap(short, long, default_value_t = 4)]
    depth: usize,

    #[clap(short='a', long, default_value_t = PlayerType::Human)]
    player_a: PlayerType,

    #[clap(short='b', long, default_value_t = PlayerType::Bot)]
    player_b: PlayerType,

    #[clap(short, long)]
    end_after_moves: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let game = Game::new();

    let player_type = |p: Player| match p {
        Player::White => args.player_a,
        Player::Black => args.player_b,
    };
    let mut session = Session {
        game_states: vec![game],
    };

    for move_number in 0.. {
        let current_game_state = session.game_states.last().unwrap();
        let player = current_game_state.player;
        if let Some(end_after_moves) = args.end_after_moves {
            if move_number >= end_after_moves {
                break;
            }
        }
        println!("{}", render_board::render_board(&current_game_state.board));
        println!(
            "{} ({}) to move. Walls: White: {}, Black: {}",
            player.to_string(),
            player_type(player),
            current_game_state.walls_left[Player::White.as_index()],
            current_game_state.walls_left[Player::Black.as_index()]
        );

        let command = match player_type(player) {
            PlayerType::Human => get_legal_command(current_game_state, player),
            PlayerType::Bot => {
                Command::AuxCommand(commands::AuxCommand::PlayBotMove { depth: args.depth })
            }
        };
        execute_command(&mut session, command);
    }
}

fn get_bot_move(game: &Game, player: Player, depth: usize) -> PlayerMove {
    let start_time = std::time::Instant::now();
    let (score, best_move) = bot::best_move_alpha_beta(game, player, depth);
    let elapsed = start_time.elapsed();
    let best_move = best_move.unwrap();
    println!(
        "Best move: {} with score: {} (took {:?})",
        best_move, score, elapsed
    );
    best_move
}
