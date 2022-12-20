use tetris::game;

#[tokio::main]
async fn main() {
    game::run().await;
    println!("GameOver");
}

