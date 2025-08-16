use crate::data_model::Player;

pub mod a_star;
pub mod all_moves;
pub mod bot;
pub mod data_model;
pub mod game_logic;
pub mod render_board;
fn main() {
    let mut game = data_model::Game::new();
    let mut player = Player::A;
    loop {
        println!("{}", render_board::render_board(&game.board));
        println!("{} to move.", player.to_string(),);
        let start_time = std::time::Instant::now();
        let (best_move, score) = bot::best_move_alpha_beta(&game, player, 1).unwrap();
        let elapsed = start_time.elapsed();
        println!(
            "Best move: {:?} with score: {} (took {:?})",
            best_move, score, elapsed
        );
        if score == bot::LOOSING_SCORE {
            println!("Player {} concedes!", player.to_string());
            break;
        }
        game_logic::execute_move_unchecked(&mut game, player, &best_move);
        player = player.opponent();
        render_board::render_board(&game.board);
    }
}
