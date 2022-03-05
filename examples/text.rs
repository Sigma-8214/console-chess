use console_chess::Game;

fn main() {
    println!();
    let game = Game::parse_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");
    game.display();
    print!("\x1B[9A");
}
