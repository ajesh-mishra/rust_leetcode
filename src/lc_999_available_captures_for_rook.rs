enum Moves {
    UpDown,
    LeftRight
}

pub struct Solution {}

impl Solution {
    fn move_up_down(board: &Vec<Vec<char>>, moves: impl Iterator<Item = usize>, pos: usize, to_move: Moves) -> i32 {
        for m in moves {
            let value = match to_move {
                Moves::UpDown => board[m][pos],
                Moves::LeftRight => board[pos][m],
            };
            if value == 'B' {
                break;
            }
            if value == 'p' {
                return 1;
            }
        }
        0
    }
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (mut count, mut row, mut col) = (0, 0, 0);
        for (r, line) in board.iter().enumerate() {
            for (c, value) in line.iter().enumerate() {
                if *value == 'R' {
                    row = r; 
                    col = c;
                }
            }
        }
        count += Solution::move_up_down(&board, (0..row).rev(), col, Moves::UpDown);
        count += Solution::move_up_down(&board, row..8, col, Moves::UpDown);
        count += Solution::move_up_down(&board, (0..col).rev(), row, Moves::LeftRight);
        count += Solution::move_up_down(&board, col..8, row, Moves::LeftRight);

        count
    }
}