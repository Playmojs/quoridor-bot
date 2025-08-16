pub const PIECE_GRID_WIDTH: usize = 9;
pub const PIECE_GRID_HEIGHT: usize = 9;
pub const WALL_GRID_WIDTH: usize = PIECE_GRID_WIDTH - 1;
pub const WALL_GRID_HEIGHT: usize = PIECE_GRID_HEIGHT - 1;
pub const PLAYER_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PiecePosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Clone)]
pub struct WallPosition {
    pub x: usize,
    pub y: usize,
}

pub type Walls = [[Option<WallOrientation>; WALL_GRID_HEIGHT]; WALL_GRID_WIDTH];

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub walls: Walls,
    pub player_positions: [PiecePosition; PLAYER_COUNT],
}

#[derive(Default, Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub walls_left: [usize; PLAYER_COUNT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct MovePiece {
    pub direction: Direction,
    pub direction_on_collision: Direction,
}

#[derive(Debug, Clone)]
pub enum PlayerMove {
    PlaceWall {
        orientation: WallOrientation,
        position: WallPosition,
    },
    MovePiece(MovePiece),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    A = 0,
    B = 1,
}

impl Board {
    pub fn new() -> Self {
        Self {
            walls: Default::default(),
            player_positions: [PiecePosition { x: 4, y: 0 }, PiecePosition { x: 4, y: 8 }],
        }
    }

    pub fn wall_at(
        &self,
        wall_orientation: WallOrientation,
        wall_pos_x: isize,
        wall_pos_y: isize,
    ) -> bool {
        wall_pos_x >= 0
            && wall_pos_y >= 0
            && wall_pos_x < WALL_GRID_WIDTH as isize
            && wall_pos_y < WALL_GRID_HEIGHT as isize
            && matches!(
                &self.walls[wall_pos_x as usize][wall_pos_y as usize],
                Some(o) if *o == wall_orientation
            )
    }

    pub fn player_position(&self, player: Player) -> &PiecePosition {
        &self.player_positions[player.as_index()]
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            walls_left: [10, 10],
        }
    }
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
    pub fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    pub fn as_index(self) -> usize {
        self as usize
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Player::A => "A",
            Player::B => "B",
        }
    }
}

impl MovePiece {
    pub fn iter() -> impl Iterator<Item = Self> {
        Direction::iter().flat_map(|direction| {
            Direction::iter().map(move |direction_on_collision| MovePiece {
                direction,
                direction_on_collision,
            })
        })
    }
}
