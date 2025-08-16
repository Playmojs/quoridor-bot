use crate::{
    a_star::a_star,
    all_moves::ALL_MOVES,
    data_model::{Game, Player, PlayerMove},
    game_logic::{execute_move_unchecked, is_move_legal},
    render_board,
};
pub const LOOSING_SCORE: isize = isize::MIN + 1;
pub const WINNING_SCORE: isize = -LOOSING_SCORE;

pub fn heuristic_board_score(game: &Game, player: Player) -> isize {
    let opponent = player.opponent();
    let opponent_path = a_star(&game.board, opponent);
    if opponent_path.is_none() {
        println!(
            "Opponent has no path in the following board:\n{}",
            render_board::render_board(&game.board)
        );
    }
    let opponent_distance = opponent_path.unwrap().len() as isize;
    if opponent_distance == 0 {
        return LOOSING_SCORE;
    }
    let player_path = a_star(&game.board, player);
    let player_distance = player_path.unwrap().len() as isize;
    if player_distance == 0 {
        return WINNING_SCORE;
    }
    let player_walls_left = game.walls_left[player.as_index()] as isize;
    let opponent_walls_left = game.walls_left[opponent.as_index()] as isize;
    opponent_distance - player_distance + player_walls_left - opponent_walls_left
}

pub fn best_move_alpha_beta(
    game: &Game,
    player: Player,
    depth: usize,
) -> Option<(PlayerMove, isize)> {
    legal_moves(game, player)
        .into_iter()
        .map(|player_move| {
            let mut game_copy = game.clone();
            execute_move_unchecked(&mut game_copy, player, player_move);
            (
                player_move.clone(),
                alpha_beta(
                    &game_copy,
                    depth,
                    LOOSING_SCORE,
                    WINNING_SCORE,
                    player.opponent(),
                ),
            )
        })
        .max_by_key(|(_, score)| *score)
}

pub fn alpha_beta(game: &Game, depth: usize, alpha: isize, beta: isize, player: Player) -> isize {
    if depth == 0 {
        return heuristic_board_score(game, player);
    }
    let mut alpha = alpha;
    let mut beta = beta;

    match player {
        Player::A => {
            let mut value = LOOSING_SCORE;
            for child_game_state in child_game_states(game, player) {
                value = value.max(alpha_beta(
                    &child_game_state,
                    depth - 1,
                    alpha,
                    beta,
                    player.opponent(),
                ));
                if value >= beta {
                    break;
                }
                alpha = alpha.max(value);
            }
            value
        }
        Player::B => {
            let mut value = WINNING_SCORE;
            for child_game_state in child_game_states(game, player) {
                value = value.min(alpha_beta(
                    &child_game_state,
                    depth - 1,
                    alpha,
                    beta,
                    player.opponent(),
                ));
                if value <= alpha {
                    break;
                }
                beta = beta.min(value);
            }
            value
        }
    }
}

fn child_game_states(game: &Game, player: Player) -> Vec<Game> {
    let mut child_states = vec![];
    for player_move in legal_moves(game, player) {
        let mut game_copy = game.clone();
        execute_move_unchecked(&mut game_copy, player, player_move);
        child_states.push(game_copy);
    }
    child_states
}

fn legal_moves(game: &Game, player: Player) -> Vec<&PlayerMove> {
    ALL_MOVES
        .iter()
        .filter(|player_move| is_move_legal(game, player, player_move))
        .collect::<Vec<_>>()
}
