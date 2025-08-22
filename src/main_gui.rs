use crate::commands::{Command, Session, execute_command, get_legal_command};
use crate::data_model::{Game, PiecePosition, Player, PlayerMove};
use crate::player_type::PlayerType;
use clap::Parser;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::{Context, ContextBuilder, GameResult};
use std::sync::mpsc::{Receiver, channel};

pub mod a_star;
pub mod all_moves;
pub mod bot;
pub mod commands;
pub mod data_model;
pub mod draw;
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

    #[clap(short, long)]
    skip_initial_moves: bool,
}

fn main() {
    let args = Args::parse();
    let mut game = Game::new();
    if args.skip_initial_moves {
        game.board.player_positions[Player::White.as_index()] = PiecePosition::new(4, 3);
        game.board.player_positions[Player::Black.as_index()] = PiecePosition::new(4, 5);
    }

    let (ctx, event_loop) = ContextBuilder::new("quoridor-bot", "Torstein Tenstad")
        .window_mode(
            WindowMode::default()
                .resizable(true)
                .dimensions(1600.0, 1600.0),
        )
        .build()
        .unwrap();
    let (tx, rx) = channel::<Game>();
    let gui_state = GuiState {
        rx,
        current_state: game.clone(),
    };

    std::thread::spawn(move || {
        let player_type = |p: Player| match p {
            Player::White => args.player_a,
            Player::Black => args.player_b,
        };
        let mut session = Session {
            game_states: vec![game],
        };
        loop {
            let current_game_state = session.game_states.last().unwrap();
            let player = current_game_state.player;
            println!(
                "{} ({}) to move. Walls: A: {}, B: {}",
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
            tx.send(session.game_states.last().unwrap().clone())
                .unwrap();
        }
    });

    event::run(ctx, event_loop, gui_state);
}

struct GuiState {
    rx: Receiver<Game>,
    current_state: Game,
}

impl EventHandler for GuiState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Ok(game) = self.rx.try_recv() {
            self.current_state = game;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        draw::draw(&self.current_state, ctx)
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
