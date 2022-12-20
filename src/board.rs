use crate::utils::{clear_screen};
use crate::mino::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardType {
    Wall,
    Block,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<BoardType>>,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Self {
        let width  = x + 2;
        let height = y + 1;
        let mut board = Self { board: vec![vec![BoardType::Empty; width]; height] };

        for y in 0..height {
            board.board[y][0]         = BoardType::Wall;
            board.board[y][width - 1] = BoardType::Wall;
        }

        for x in 0..width {
            board.board[height - 1][x] = BoardType::Wall;
        }

        board
    }

    pub fn show(&self) {
        clear_screen();
        for line in &self.board {
            for block in line {
                match block {
                    BoardType::Wall => print!("@"),
                    BoardType::Block => print!("\u{25AE}"),
                    BoardType::Empty => print!(" "),
                }
            }
            println!("");
        }
    }

    pub fn put_mino(&self, mino: &Mino) -> Option<Self> {
        let width  = self.board[0].len();
        let height = self.board.len();
        let mut new_board = self.clone();
        for (x, y) in mino.positions() {
            if x < 0 || x >= width as i8 || y < 0 || y >= height as i8 {
                return None
            }
            if new_board.board[y as usize][x as usize] != BoardType::Empty {
                return None
            }
            new_board.board[y as usize][x as usize] = BoardType::Block;
        }
        Some(new_board)
    }

    pub fn erase_fill_lines(&mut self) {
        let mut board = self.board
            .clone()
            .into_iter()
            .filter(|line|
                !line[1..line.len()-1]
                    .iter()
                    .all(|&c| c == BoardType::Block))
            .collect::<Vec<Vec<BoardType>>>();

        let erased_count = self.board.len() - board.len();
        let width = self.board[0].len();
        let mut empty_line = vec![BoardType::Empty; width];
        empty_line[0] = BoardType::Wall;
        empty_line[width - 1] = BoardType::Wall;
        for _ in 0..erased_count {
            board.insert(0, empty_line.clone());
        }

        self.board = board;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initial_len() {
        let board = Board::new(10, 20);
        assert_eq!(board.board.len(), 21);
        assert_eq!(board.board[0].len(), 12);
    }

    #[test]
    fn test_board_initial_content() {
        let board = Board::new(10, 20);
        assert_eq!(board.board[0][0], BoardType::Wall);
        assert_eq!(board.board[0][1], BoardType::Empty);
        assert_eq!(board.board[0][11], BoardType::Wall);
        assert_eq!(board.board[20][0], BoardType::Wall);
        assert_eq!(board.board[20][1], BoardType::Wall);
        assert_eq!(board.board[20][11], BoardType::Wall);
    }

    #[test]
    fn test_board_put_mino() {
        let mut board = Board::new(10, 20);
        let mino = Mino { type_: MinoType::I, pos: (1, 1), rotation: 0 };
        board = board.put_mino(&mino).unwrap();
        assert_eq!(board.board[0][1], BoardType::Block);
        assert_eq!(board.board[1][1], BoardType::Block);
        assert_eq!(board.board[2][1], BoardType::Block);
        assert_eq!(board.board[3][1], BoardType::Block);
    }

    #[test]
    fn test_board_eralse_fill_lines() {
        let mut board = Board::new(4, 4);

        let mino = Mino { type_: MinoType::I, pos: (2, 3), rotation: 1 };
        board = board.put_mino(&mino).unwrap();

        board.erase_fill_lines();
        assert_eq!(board.board[3][0], BoardType::Wall);
        assert_eq!(board.board[3][1], BoardType::Empty);
        assert_eq!(board.board[3][2], BoardType::Empty);
        assert_eq!(board.board[3][3], BoardType::Empty);
        assert_eq!(board.board[3][4], BoardType::Empty);
        assert_eq!(board.board[3][5], BoardType::Wall);
    }
}
