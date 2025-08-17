use crate::{
    a_star::a_star,
    data_model::{
        Direction, Game, MovePiece, Player, PlayerMove, WALL_GRID_HEIGHT, WALL_GRID_WIDTH,
        WallOrientation, WallPosition,
    },
    game_logic::{
        execute_move_unchecked, is_move_piece_legal_with_player_at_position,
        room_for_wall_placement,
    },
    render_board,
    square_outline_iterator::SquareOutlineIterator,
};
pub const LOOSING_SCORE: isize = isize::MIN + 1;
pub const WINNING_SCORE: isize = -LOOSING_SCORE;

pub fn heuristic_board_score(game: &Game) -> isize {
    let opponent_path = a_star(&game.board, Player::B);
    let player_path = a_star(&game.board, Player::A);
    if player_path.is_none() {
        println!(
            "Opponent has no path in the following board:\n{}",
            render_board::render_board(&game.board)
        );
    }
    let opponent_distance = opponent_path.unwrap().len() as isize;
    if opponent_distance == 0 {
        return LOOSING_SCORE;
    }
    let player_distance = player_path.unwrap().len() as isize;
    if player_distance == 0 {
        return WINNING_SCORE;
    }
    let player_walls_left = game.walls_left[Player::A.as_index()] as isize;
    let opponent_walls_left = game.walls_left[Player::B.as_index()] as isize;
    let distance_score = opponent_distance - player_distance;
    let wall_score = player_walls_left - opponent_walls_left;
    let (distance_priority, wall_priority) = (1, 0);
    distance_priority * distance_score + wall_priority * wall_score
}

pub fn best_move_alpha_beta(
    game: &Game,
    player: Player,
    depth: usize,
) -> (isize, Option<PlayerMove>) {
    alpha_beta(game, depth, LOOSING_SCORE, WINNING_SCORE, player)
}

pub fn alpha_beta(
    game: &Game,
    depth: usize,
    alpha: isize,
    beta: isize,
    player: Player,
) -> (isize, Option<PlayerMove>) {
    if depth == 0 {
        return (heuristic_board_score(game), None);
    }
    let mut alpha = alpha;
    let mut beta = beta;
    let mut best_move = None;
    let score = match player {
        Player::A => {
            let mut value = LOOSING_SCORE;
            for player_move in moves_ordered_by_heuristic_quality(game, player) {
                let mut child_game_state = game.clone();
                execute_move_unchecked(&mut child_game_state, player, &player_move);
                if a_star(&child_game_state.board, player).is_none()
                    || a_star(&child_game_state.board, player.opponent()).is_none()
                {
                    continue;
                }
                let (score, _) =
                    alpha_beta(&child_game_state, depth - 1, alpha, beta, player.opponent());
                if score > value {
                    best_move = Some(player_move);
                }
                value = value.max(score);
                if value >= beta {
                    break;
                }
                alpha = alpha.max(value);
            }
            value
        }
        Player::B => {
            let mut value = WINNING_SCORE;
            for player_move in moves_ordered_by_heuristic_quality(game, player) {
                let mut child_game_state = game.clone();
                execute_move_unchecked(&mut child_game_state, player, &player_move);
                if a_star(&child_game_state.board, player).is_none()
                    || a_star(&child_game_state.board, player.opponent()).is_none()
                {
                    continue;
                }
                let (score, _) =
                    alpha_beta(&child_game_state, depth - 1, alpha, beta, player.opponent());
                if score < value {
                    best_move = Some(player_move);
                }
                value = value.min(score);
                if value <= alpha {
                    break;
                }
                beta = beta.min(value);
            }
            value
        }
    };
    (score, best_move)
}

fn moves_ordered_by_heuristic_quality(game: &Game, player: Player) -> Vec<PlayerMove> {
    let mut moves: Vec<PlayerMove> = Default::default();
    let player_position = game.board.player_position(player);
    let opponent_position = game.board.player_position(player.opponent());
    let x_diff = opponent_position.x() as isize - player_position.x() as isize;
    let y_diff = opponent_position.y() as isize - player_position.y() as isize;

    let push_if_move_piece_is_legal =
        |moves: &mut Vec<PlayerMove>, direction: Direction, direction_on_collision: Direction| {
            let move_piece = MovePiece {
                direction,
                direction_on_collision,
            };
            if is_move_piece_legal_with_player_at_position(
                &game.board,
                player,
                player_position,
                &move_piece,
            ) {
                moves.push(PlayerMove::MovePiece(move_piece));
            }
        };

    if let Some(jump_direction) = match (x_diff, y_diff) {
        (0, 1) => Some(Direction::Down),
        (0, -1) => Some(Direction::Up),
        (1, 0) => Some(Direction::Right),
        (-1, 0) => Some(Direction::Left),
        _ => None,
    } {
        for direction in Direction::iter() {
            push_if_move_piece_is_legal(&mut moves, jump_direction, direction);
        }
        for direction in Direction::iter().filter(|&d| d != jump_direction) {
            push_if_move_piece_is_legal(&mut moves, direction, Direction::Up);
        }
    } else {
        for direction in Direction::iter() {
            push_if_move_piece_is_legal(&mut moves, direction, Direction::Up);
        }
    }

    let origin = opponent_position;
    for i in 1.. {
        let top_left_x = origin.x() as isize - i as isize;
        let top_left_y = origin.y() as isize - i as isize;
        let side_length = 2 * i;
        let mut some_in_bounds = false;
        for (x, y) in SquareOutlineIterator::new(top_left_x, top_left_y, side_length) {
            let in_bounds =
                x >= 0 && y >= 0 && x < WALL_GRID_WIDTH as isize && y < WALL_GRID_HEIGHT as isize;
            if !in_bounds {
                continue;
            }
            some_in_bounds = true;
            for orientation in [WallOrientation::Horizontal, WallOrientation::Vertical] {
                let player_move = PlayerMove::PlaceWall {
                    orientation,
                    position: WallPosition {
                        x: x as usize,
                        y: y as usize,
                    },
                };
                if game.walls_left[player.as_index()] > 0
                    && room_for_wall_placement(&game.board, orientation, x, y)
                {
                    moves.push(player_move);
                }
            }
        }
        if !some_in_bounds {
            break;
        }
    }
    moves
}
