use crate::mino::*;
use crate::move_command::*;
use crate::board::*;

pub async fn run() {
    use tokio::io::AsyncBufReadExt;

    let mut down_timer = tokio::time::interval(std::time::Duration::from_secs(1));
    let mut lines_from_stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    let mut mino = Mino::random();
    let mut board = Board::new(10, 20);

    loop {
        if let Some(new_board) = board.put_mino(&mino) {
            new_board.show();
        } else {
            break;
        }
        tokio::select! {
            Ok(Some(line)) = lines_from_stdin.next_line() => {
                if let Ok(command) = line.try_into() {
                    let tmp_mino = mino.moved(command);
                    if board.put_mino(&tmp_mino).is_some() {
                        mino = tmp_mino;
                    }
                }
            }
            _ = down_timer.tick() => {
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
}
