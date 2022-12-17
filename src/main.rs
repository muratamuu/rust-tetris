#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardType {
    Wall,
    Block,
    Empty,
}

#[derive(Debug, Clone)]
struct Board<const X: usize, const Y: usize> {
    board: [[BoardType; X]; Y],
}

impl<const X: usize, const Y: usize> Board<X, Y> {
    fn new() -> Self {
        let mut board = Self {
            board: [[BoardType::Empty; X]; Y],
        };

        for y in 0..Y {
            board.board[y][0] = BoardType::Wall;
            board.board[y][X - 1] = BoardType::Wall;
        }
        for x in 0..X {
            board.board[Y - 1][x] = BoardType::Wall;
        }

        board
    }

    fn show(&self) {
        for line in self.board {
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
        let mut new_board = self.clone();
        for (x, y) in mino.positions() {
            if x < 0 || x >= X as i8 || y < 0 || y >= Y as i8 {
                return None
            }
            if new_board.board[y as usize][x as usize] != BoardType::Empty {
                return None
            }
            new_board.board[y as usize][x as usize] = BoardType::Block;
        }
        Some(new_board)
    }
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
    let mut board = Board::<{10+2}, {20+1}>::new();

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
                    mino = Mino::random();
                }
            }
        }
    }
    println!("GameOver");
}

