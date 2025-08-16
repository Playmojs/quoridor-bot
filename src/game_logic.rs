use crate::{
    a_star::a_star,
    data_model::{
        Board, Direction, Game, MovePiece, PIECE_GRID_HEIGHT, PiecePosition, Player, PlayerMove,
        WALL_GRID_HEIGHT, WALL_GRID_WIDTH, WallOrientation,
    },
};

pub fn execute_move_unchecked(game: &mut Game, player: Player, player_move: &PlayerMove) {
    match player_move {
        PlayerMove::PlaceWall {
            orientation,
            position,
        } => {
            game.board.walls[position.x][position.y] = Some(*orientation);
            game.walls_left[player.as_index()] -= 1;
        }
        PlayerMove::MovePiece(move_piece) => {
            let new_position = new_position_after_move_piece_unchecked(
                game.board.player_position(player),
                move_piece,
                game.board.player_position(player.opponent()),
            );
            game.board.player_positions[player.as_index()] = new_position;
        }
    }
}

pub fn is_move_legal(game: &Game, player: Player, player_move: &PlayerMove) -> bool {
    is_move_legal_with_player_at_position(
        game,
        player,
        game.board.player_position(player),
        player_move,
    )
}
pub fn is_move_piece_legal_with_player_at_position(
    board: &Board,
    player: Player,
    player_position: &PiecePosition,
    move_piece: &MovePiece,
) -> bool {
    if is_move_direction_legal_with_player_at_position(
        board,
        player_position,
        &move_piece.direction,
    ) {
        let new_position =
            new_position_after_direction_unchecked(player_position, move_piece.direction);
        if new_position == *board.player_position(player.opponent()) {
            is_move_direction_legal_with_player_at_position(
                board,
                &new_position,
                &move_piece.direction_on_collision,
            )
        } else {
            true
        }
    } else {
        false
    }
}

pub fn is_move_direction_legal_with_player_at_position(
    board: &Board,
    player_position: &PiecePosition,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Up => {
            player_position.y > 0
                && !board.wall_at(
                    WallOrientation::Horizontal,
                    player_position.x as isize - 1,
                    player_position.y as isize - 1,
                )
                && !board.wall_at(
                    WallOrientation::Horizontal,
                    player_position.x as isize,
                    player_position.y as isize - 1,
                )
        }
        Direction::Down => {
            player_position.y < PIECE_GRID_HEIGHT - 1
                && !board.wall_at(
                    WallOrientation::Horizontal,
                    player_position.x as isize - 1,
                    player_position.y as isize,
                )
                && !board.wall_at(
                    WallOrientation::Horizontal,
                    player_position.x as isize,
                    player_position.y as isize,
                )
        }
        Direction::Left => {
            player_position.x > 0
                && !board.wall_at(
                    WallOrientation::Vertical,
                    player_position.x as isize - 1,
                    player_position.y as isize,
                )
                && !board.wall_at(
                    WallOrientation::Vertical,
                    player_position.x as isize - 1,
                    player_position.y as isize - 1,
                )
        }
        Direction::Right => {
            player_position.x < PIECE_GRID_HEIGHT - 1
                && !board.wall_at(
                    WallOrientation::Vertical,
                    player_position.x as isize,
                    player_position.y as isize,
                )
                && !board.wall_at(
                    WallOrientation::Vertical,
                    player_position.x as isize,
                    player_position.y as isize - 1,
                )
        }
    }
}

pub fn is_move_legal_with_player_at_position(
    game: &Game,
    player: Player,
    player_position: &PiecePosition,
    player_move: &PlayerMove,
) -> bool {
    match player_move {
        PlayerMove::MovePiece(move_piece) => is_move_piece_legal_with_player_at_position(
            &game.board,
            player,
            player_position,
            move_piece,
        ),
        PlayerMove::PlaceWall {
            orientation,
            position,
        } => {
            let (offsets_to_check, other_orientation) = match orientation {
                WallOrientation::Horizontal => {
                    ([(-1, 0), (0, 0), (1, 0)], WallOrientation::Vertical)
                }
                WallOrientation::Vertical => {
                    ([(0, -1), (0, 0), (0, 1)], WallOrientation::Horizontal)
                }
            };
            let blocks_path = |player_to_block_check: Player| {
                let mut game_copy = game.clone();
                execute_move_unchecked(
                    &mut game_copy,
                    player,
                    &PlayerMove::PlaceWall {
                        orientation: *orientation,
                        position: position.clone(),
                    },
                );
                a_star(&game_copy.board, player_to_block_check).is_none()
            };
            game.walls_left[player.as_index()] > 0
                && position.x < WALL_GRID_WIDTH
                && position.y < WALL_GRID_HEIGHT
                && !game
                    .board
                    .wall_at(other_orientation, position.x as isize, position.y as isize)
                && offsets_to_check.iter().all(|(dx, dy)| {
                    !game.board.wall_at(
                        *orientation,
                        position.x as isize + dx,
                        position.y as isize + dy,
                    )
                })
                && !blocks_path(player)
                && !blocks_path(player.opponent())
        }
    }
}

pub fn new_position_after_direction_unchecked(
    player_position: &PiecePosition,
    direction: Direction,
) -> PiecePosition {
    let (dx, dy) = direction.to_offset();
    PiecePosition {
        x: (player_position.x as isize + dx) as usize,
        y: (player_position.y as isize + dy) as usize,
    }
}

pub fn new_position_after_move_piece_unchecked(
    player_position: &PiecePosition,
    move_piece: &MovePiece,
    opponent_position: &PiecePosition,
) -> PiecePosition {
    let new_position =
        new_position_after_direction_unchecked(player_position, move_piece.direction);
    if opponent_position == &new_position {
        new_position_after_direction_unchecked(opponent_position, move_piece.direction_on_collision)
    } else {
        new_position
    }
}
