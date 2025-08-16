use std::collections::HashMap;

use crate::data_model::{Board, MovePiece, PIECE_GRID_HEIGHT, PiecePosition, Player};
use crate::game_logic::{
    is_move_piece_legal_with_player_at_position, new_position_after_move_piece_unchecked,
};

pub fn heuristic(pos: &PiecePosition, player: Player) -> usize {
    match player {
        Player::A => PIECE_GRID_HEIGHT - 1 - pos.y,
        Player::B => pos.y,
    }
}

pub fn a_star(board: &Board, player: Player) -> Option<Vec<PiecePosition>> {
    let start = board.player_position(player);
    let mut open_set = vec![start.clone()];
    let mut came_from = HashMap::<PiecePosition, PiecePosition>::new();
    let mut g_score = HashMap::<PiecePosition, usize>::new();
    let mut f_score = HashMap::<PiecePosition, usize>::new();
    g_score.insert(start.clone(), 0);
    f_score.insert(start.clone(), heuristic(start, player));

    while let Some(current) = open_set.iter().min_by_key(|pos| f_score[pos]).cloned() {
        if heuristic(&current, player) == 0 {
            return Some(reconstruct_path(&came_from, &current));
        }
        open_set.retain(|pos| pos != &current);
        for neighbor in neighbors(board, player, &current) {
            let tentative_g_score = g_score[&current] + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                f_score.insert(
                    neighbor.clone(),
                    tentative_g_score + heuristic(&neighbor, player),
                );
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }
    None
}

fn reconstruct_path(
    came_from: &HashMap<PiecePosition, PiecePosition>,
    current: &PiecePosition,
) -> Vec<PiecePosition> {
    let mut total_path = vec![current.clone()];
    let mut current = current;
    while let Some(next) = came_from.get(current) {
        total_path.push(next.clone());
        current = next;
    }
    total_path.pop(); // Remove the start position
    total_path.reverse();
    total_path
}

fn neighbors(board: &Board, player: Player, player_position: &PiecePosition) -> Vec<PiecePosition> {
    MovePiece::iter()
        .filter_map(|move_piece| {
            is_move_piece_legal_with_player_at_position(board, player, player_position, &move_piece)
                .then_some(new_position_after_move_piece_unchecked(
                    player_position,
                    &move_piece,
                    board.player_position(player.opponent()),
                ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data_model::{Game, WallOrientation},
        render_board,
    };

    #[test]
    fn single_wall_test() {
        let mut game = Game::new();
        game.board.walls[3][2] = Some(WallOrientation::Horizontal);
        let path = a_star(&game.board, Player::A);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(
            path,
            vec![
                PiecePosition { x: 4, y: 1 },
                PiecePosition { x: 4, y: 2 },
                PiecePosition { x: 5, y: 2 },
                PiecePosition { x: 5, y: 3 },
                PiecePosition { x: 5, y: 4 },
                PiecePosition { x: 5, y: 5 },
                PiecePosition { x: 5, y: 6 },
                PiecePosition { x: 5, y: 7 },
                PiecePosition { x: 5, y: 8 },
            ]
        );
    }

    #[test]
    fn complex_wall_test() {
        let mut game = Game::new();
        game.board.player_positions[Player::A.as_index()] = PiecePosition { x: 4, y: 4 };
        game.board.player_positions[Player::B.as_index()] = PiecePosition { x: 3, y: 4 };
        game.board.walls[2][3] = Some(WallOrientation::Vertical);
        game.board.walls[3][3] = Some(WallOrientation::Vertical);
        game.board.walls[2][5] = Some(WallOrientation::Vertical);
        game.board.walls[4][3] = Some(WallOrientation::Horizontal);
        game.board.walls[4][4] = Some(WallOrientation::Horizontal);
        game.board.walls[5][5] = Some(WallOrientation::Vertical);
        let path = a_star(&game.board, Player::A);
        assert!(path.is_some());
    }

    #[test]
    fn on_goal_test() {
        let mut game = Game::new();
        game.board.player_positions[0] = PiecePosition { x: 4, y: 8 };
        let path = a_star(&game.board, Player::A);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 0);
    }
}
