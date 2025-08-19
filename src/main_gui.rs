use crate::commands::get_human_move;
use crate::data_model::{Game, Player, PlayerMove};
use crate::player_type::PlayerType;
use clap::Parser;
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
}

fn main() {
    let args = Args::parse();
    let mut game = Game::new();

    let (ctx, event_loop) = ContextBuilder::new("quoridor-bot", "Torstein Tenstad")
        .build()
        .unwrap();
    let (tx, rx) = channel::<Game>();
    let gui_state = GuiState {
        rx,
        current_state: game.clone(),
    };

    std::thread::spawn(move || {
        let player_type = |p: Player| match p {
            Player::A => args.player_a,
            Player::B => args.player_b,
        };
        loop {
            let player = game.player;
            println!(
                "{} ({}) to move. Walls: A: {}, B: {}",
                player.to_string(),
                player_type(player),
                game.walls_left[Player::A.as_index()],
                game.walls_left[Player::B.as_index()]
            );
            let player_move = match player_type(player) {
                PlayerType::Human => get_human_move(&game, player),
                PlayerType::Bot => get_bot_move(&game, player, args.depth),
            };
            game_logic::execute_move_unchecked(&mut game, player, &player_move);
            tx.send(game.clone()).unwrap();
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
    println!(
        "Best move: {:?} with score: {} (took {:?})",
        best_move, score, elapsed
    );
    best_move.unwrap()
}
