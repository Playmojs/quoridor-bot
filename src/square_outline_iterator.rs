pub struct SquareOutlineIterator {
    top_left_x: isize,
    top_left_y: isize,
    side_length: usize,
    index: usize,
}

impl SquareOutlineIterator {
    pub fn new(top_left_x: isize, top_left_y: isize, side_length: usize) -> Self {
        SquareOutlineIterator {
            top_left_x,
            top_left_y,
            side_length,
            index: 0,
        }
    }
}

impl Iterator for SquareOutlineIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.side_length < 1 || self.index >= 4 * (self.side_length - 1) {
            return None;
        }
        let l = self.side_length - 1;
        let side_index = self.index / l;
        let position_index = self.index % l;
        let (dx, dy) = match side_index {
            0 => (position_index, 0),
            1 => (l, position_index),
            2 => (l - position_index, l),
            _ => (0, l - position_index),
        };
        self.index += 1;
        Some((self.top_left_x + dx as isize, self.top_left_y + dy as isize))
    }
}

mod test {
    #[test]
    fn test2by2in00() {
        let iter = crate::square_outline_iterator::SquareOutlineIterator::new(0, 0, 2);
        let expected: Vec<(isize, isize)> = vec![(0, 0), (1, 0), (1, 1), (0, 1)];
        let result: Vec<_> = iter.collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn test3by3in42() {
        let iter = crate::square_outline_iterator::SquareOutlineIterator::new(4, 2, 3);
        let expected: Vec<(isize, isize)> = vec![
            (4, 2),
            (5, 2),
            (6, 2),
            (6, 3),
            (6, 4),
            (5, 4),
            (4, 4),
            (4, 3),
        ];
        let result: Vec<_> = iter.collect();
        assert_eq!(result, expected);
    }
}
