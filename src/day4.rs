/// Notes for part 1
///
/// Board needs to count down as numbers are called, so we want to map each
/// number to its row and column, and to have a countdown for each row and
/// column. As each number lands, the appropriate row and column are
/// decremented.
///
use std::collections::HashMap;

pub const BOARD_SIZE: usize = 5;

#[derive(Default)]
pub struct Board {
    pub numbers: HashMap<usize, (usize, usize)>,
    pub rows: [usize; BOARD_SIZE],
    pub cols: [usize; BOARD_SIZE],
}

impl Board {
    pub fn from_strings(input: Vec<String>) -> Self {
        let mut result = Board::default();

        for (row, row_str) in input.iter().enumerate() {
            for (col, col_str) in row_str.split_whitespace().enumerate() {
                let number = col_str.parse::<usize>().unwrap();
                result.numbers.insert(number, (row, col));
            }
        }
        result
    }

    pub fn play(&mut self, num: usize) -> Option<usize> {
        if let Some((row, col)) = self.numbers.remove(&num) {
            self.rows[row] += 1;
            self.cols[col] += 1;

            if self.rows[row] == BOARD_SIZE || self.cols[col] == BOARD_SIZE {
                let unmarked_sum: usize = self.numbers.keys().sum();
                return Some(unmarked_sum * num);
            }
        }
        None
    }
}

pub struct Game {
    pub boards: Vec<Board>,
    pub turns: Vec<usize>,
}

impl Game {
    pub fn from_strings(strings: &Vec<String>) -> Self {
        let turns = (&strings[0])
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut result = Game {
            turns: turns,
            boards: Vec::new(),
        };
        for board_src in strings[1..].chunks(6) {
            result
                .boards
                .push(Board::from_strings(board_src[1..].to_vec()));
        }
        result
    }

    pub fn play_to_win(&mut self) -> Option<usize> {
        for turn in &self.turns {
            for board in self.boards.iter_mut() {
                if let Some(result) = board.play(*turn) {
                    return Some(result);
                }
            }
        }
        None
    }

    pub fn play_to_lose(&mut self) -> Option<usize> {
        for turn in &self.turns {
            let mut i = 0;

            // iterate through the boards, removing any that complete
            while i < self.boards.len() {
                if let Some(result) = self.boards[i].play(*turn) {
                    if self.boards.len() == 1 {
                        // last board standing!
                        return Some(result);
                    } else {
                        self.boards.remove(i);
                    }
                } else {
                    i += 1;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_up_a_board() {
        let board = Board::from_strings(sample_board());

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
        let result = board.play(16);
        assert_eq!(None, result);

        let result = board.play(7);
        assert_eq!(Some(1631), result);
    }

    #[test]
    fn sets_up_a_game() {
        let game = Game::from_strings(&sample_game());
        assert_eq!(27, game.turns.len());
        assert_eq!(3, game.boards.len());
    }

    #[test]
    fn plays_game_to_win() {
        let mut game = Game::from_strings(&sample_game());
        assert_eq!(Some(4512), game.play_to_win());
    }

    #[test]
    fn plays_game_to_lose() {
        let mut game = Game::from_strings(&sample_game());
        assert_eq!(Some(1924), game.play_to_lose());
    }

    fn sample_board() -> Vec<String> {
        [
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .map(|s| s.to_string())
        .to_vec()
    }

    fn sample_game() -> Vec<String> {
        [
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
        .map(|s| s.to_string())
        .to_vec()
    }
}
