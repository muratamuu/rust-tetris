#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardType {
    Wall,
    Block,
    Empty,
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<BoardType>>,
}

impl Board {
    fn new(x: usize, y: usize) -> Self {
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

    fn show(&self) {
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

    fn put_mino(&self, mino: &Mino) -> Option<Self> {
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

    fn erase_fill_lines(&mut self) {
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

#[test]
fn test_board_initial_len() {
    let board = Board2::new(10, 20);
    assert_eq!(board.board.len(), 21);
    assert_eq!(board.board[0].len(), 12);
}

#[test]
fn test_board_initial_content() {
    let board = Board2::new(10, 20);
    assert_eq!(board.board[0][0], BoardType::Wall);
    assert_eq!(board.board[0][1], BoardType::Empty);
    assert_eq!(board.board[0][11], BoardType::Wall);
    assert_eq!(board.board[20][0], BoardType::Wall);
    assert_eq!(board.board[20][1], BoardType::Wall);
    assert_eq!(board.board[20][11], BoardType::Wall);
}

#[test]
fn test_board_put_mino() {
    let mut board = Board2::new(10, 20);
    let mino = Mino { type_: MinoType::I, pos: (1, 1), rotation: 0 };
    board = board.put_mino(&mino).unwrap();
    assert_eq!(board.board[0][1], BoardType::Block);
    assert_eq!(board.board[1][1], BoardType::Block);
    assert_eq!(board.board[2][1], BoardType::Block);
    assert_eq!(board.board[3][1], BoardType::Block);
}

#[test]
fn test_board_eralse_fill_lines() {
    let mut board = Board2::new(4, 4);

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

type Pos = (i8, i8);

#[derive(Debug, Clone)]
enum MinoType {
    I, O, S, Z, J, L, T,
}

impl MinoType {
    fn pattern(&self) -> &'static [[Pos; 4]] {
        match self {
            Self::I => { const PAT: [[Pos; 4]; 2] = [
                [( 0,  0), ( 0, -1), ( 0,  1), ( 0,  2)],
                [( 0,  0), (-1,  0), ( 1,  0), ( 2,  0)],
            ]; &PAT },
            Self::O => { const PAT: [[Pos; 4]; 1] = [
                [( 0,  0), ( 1,  0), ( 0, -1), ( 1, -1)],
            ]; &PAT },
            Self::S => { const PAT: [[Pos; 4]; 2] = [
                [( 0,  0), (-1,  0), ( 0, -1), ( 1, -1)],
                [( 0,  0), ( 0, -1), ( 1,  0), ( 1,  1)],
            ]; &PAT },
            Self::Z => { const PAT: [[Pos; 4]; 2] = [
                [( 0,  0), ( 1,  0), ( 0, -1), (-1, -1)],
                [( 0,  0), ( 0,  1), ( 1,  0), ( 1, -1)],
            ]; &PAT },
            Self::J => { const PAT: [[Pos; 4]; 4] = [
                [( 0,  0), ( 0, -1), ( 0,  1), (-1,  1)],
                [( 0,  0), ( 1,  0), (-1,  0), (-1, -1)],
                [( 0,  0), ( 0,  1), ( 0, -1), ( 1, -1)],
                [( 0,  0), (-1,  0), ( 1,  0), ( 1,  1)],
            ]; &PAT },
            Self::L => { const PAT: [[Pos; 4]; 4] = [
                [( 0,  0), ( 0, -1), ( 0,  1), ( 1,  1)],
                [( 0,  0), ( 1,  0), (-1,  0), (-1,  1)],
                [( 0,  0), ( 0,  1), ( 0, -1), (-1, -1)],
                [( 0,  0), (-1,  0), ( 1,  0), ( 1, -1)],
            ]; &PAT },
            Self::T => { const PAT: [[Pos; 4]; 4] = [
                [( 0,  0), ( 0, -1), ( 1,  0), (-1,  0)],
                [( 0,  0), ( 1,  0), ( 0,  1), ( 0, -1)],
                [( 0,  0), ( 0,  1), (-1,  0), ( 1,  0)],
                [( 0,  0), (-1,  0), ( 0, -1), ( 0,  1)],
            ]; &PAT },
        }
    }
}

enum MoveCommand {
    Down,
    Left,
    Right,
    Rotate,
}

#[derive(Debug, Clone)]
struct Mino {
    type_: MinoType,
    pos: Pos,
    rotation: usize,
}

impl Mino {
    fn random() -> Self {
        use rand::prelude::*;
        let type_ = match rand::thread_rng().gen_range(0, 7) {
            0 => MinoType::I,
            1 => MinoType::O,
            2 => MinoType::S,
            3 => MinoType::Z,
            4 => MinoType::J,
            5 => MinoType::L,
            6 => MinoType::T,
            _ => MinoType::I,
        };
        Self { type_: type_, pos: (5, 1), rotation: 0 }
    }

    fn positions(&self) -> Vec<Pos> {
        let pat_idx = self.rotation % self.type_.pattern().len();
        self.type_.pattern()[pat_idx].iter()
            .map(|(dx, dy)| (self.pos.0 + dx, self.pos.1 + dy))
            .collect()
    }

    fn moved(&self, command: MoveCommand) -> Mino {
        let mut mino = self.clone();
        match command {
            MoveCommand::Left   => mino.pos.0 -= 1,
            MoveCommand::Right  => mino.pos.0 += 1,
            MoveCommand::Down   => mino.pos.1 += 1,
            MoveCommand::Rotate => mino.rotation += 1,
        }
        mino
    }
}

fn clear_screen() {
    print!("\x1b[2J");
    move_cursor(0, 0);
}

fn move_cursor(x: u8, y: u8) {
    print!("\x1b[{};{}H", x, y);
}

fn str_to_command(line: String) -> Option<MoveCommand> {
    match line.trim() {
        "a" => Some(MoveCommand::Left),
        "d" => Some(MoveCommand::Right),
        "s" => Some(MoveCommand::Rotate),
        ""  => Some(MoveCommand::Down),
        _   => None,
    }
}

#[tokio::main]
async fn main() {
    use futures::{FutureExt, StreamExt}; // fuse(), next()
    use tokio::io::AsyncBufReadExt;

    let down_timer = async_stream::stream! {
        loop {
            let delay = futures_timer::Delay::new(std::time::Duration::from_secs(1));
            delay.await;
            yield ();
        }
    };

    tokio::pin!(down_timer);

    let mut lines_from_stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    let mut mino = Mino::random();
    let mut board = Board::new(10, 20);

    loop {
        if let Some(new_board) = board.put_mino(&mino) {
            clear_screen();
            new_board.show();
        } else {
            break;
        }
        tokio::select! {
            Ok(Some(line)) = lines_from_stdin.next_line().fuse() => {
                if let Some(command) = str_to_command(line) {
                    let tmp_mino = mino.moved(command);
                    if board.put_mino(&tmp_mino).is_some() {
                        mino = tmp_mino;
                    }
                }
            }
            _ = down_timer.next() => {
                let tmp_mino = mino.moved(MoveCommand::Down);
                if board.put_mino(&tmp_mino).is_some() {
                    mino = tmp_mino;
                } else {
                    board = board.put_mino(&mino).unwrap();
                    board.erase_fill_lines();
                    mino = Mino::random();
                }
            }
        }
    }
    println!("GameOver");
}

