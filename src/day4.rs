/// Notes for part 1
///
/// Board needs to count down as numbers are called, so we want to map each
/// number to its row and column, and to have a countdown for each row and
/// column. As each number lands, the appropriate row and column are
/// decremented.
///
pub mod part1 {
    use std::collections::HashMap;

    pub const BOARD_SIZE: usize = 5;

    #[derive(Default)]
    pub struct Board {
        pub numbers: HashMap<usize, (usize, usize)>,
        pub rows: [usize; BOARD_SIZE],
        pub cols: [usize; BOARD_SIZE],
        pub is_finished: bool,
    }

    impl Board {
        pub fn from_strings(strings: &[&str]) -> Self {
            let mut result = Board::default();

            for (row_id, row) in strings.iter().enumerate() {
                for (col_id, col) in row.split_whitespace().enumerate() {
                    let number = col.parse::<usize>().unwrap();
                    result.numbers.insert(number, (row_id, col_id));
                }
            }
            result
        }

        pub fn play(&mut self, num: usize) {
            if let Some((row, col)) = self.numbers.get(&num) {
                self.rows[*row] += 1;
                self.cols[*col] += 1;

                if self.rows[*row] == BOARD_SIZE || self.cols[*col] == BOARD_SIZE {
                    self.is_finished = true;
                }
            }
        }
    }

    pub struct Game {
        boards: Vec<Board>,
        turns: Vec<usize>
    }

    impl Game {
        pub fn from_strings(strings: &[&str]) -> Self {
            let turns = (&strings[0])
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let mut result = Game {
                turns: turns,
                boards: Vec::new(),
            };
            for board_src in strings[1..].chunks(6) {
                result.boards.push(Board::from_strings(&board_src[1..]));
            }
            result
        }

        pub fn sizes(&self) -> (usize, usize) {
            (self.turns.len(), self.boards.len())
        }

        pub fn play(&mut self) -> Option<usize> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part1::*;

    #[test]
    fn sets_up_a_board() {
        let board = Board::from_strings(sample_board());

        assert_eq!(false, board.is_finished);

        for (pos, num) in [
            ((0, 0), 22),
            ((1, 1), 2),
            ((2, 3), 16),
            ((2, 4), 7),
            ((4, 4), 19),
        ] {
            assert_eq!(board.numbers[&num], pos);
        }
    }

    #[test]
    fn completed_row_wins() {
        // given
        let mut board = Board::from_strings(sample_board());

        // play middle row
        board.play(21);
        board.play(9);
        board.play(14);
        board.play(16);
        assert_eq!(false, board.is_finished);

        board.play(7);
        assert_eq!(true, board.is_finished);
    }

    #[test]
    fn sets_up_a_game() {
        let game = Game::from_strings(sample_game());
        assert_eq!((27, 3), game.sizes());
    }

    #[test]
    fn plays_game() {
        let mut game = Game::from_strings(sample_game());

        assert_eq!(Some(4512), game.play());
    }

    fn sample_board() -> &'static [&'static str] {
        &[
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
    }

    fn sample_game() -> &'static [&'static str] {
        &[
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
    }
}
