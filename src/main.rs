use std::io;
use tic_tac_toe::Game;
use tic_tac_toe::Point;


fn player_helper() -> Point {
    let mut s = String::new();
    io::stdin().read_line(&mut s);
    let point:Vec<&str> = s.trim_end().split(",").collect();
    let first = point[0].parse::<usize>().expect("数値ではありません。");
    let second = point[1].parse::<usize>().expect("数値ではありません。");
    let point = Point {
        column:first,
        row:second,
    };
    return point
}

fn main() {
    let mut game = Game {
        board:vec![vec!['■';3];3]
    };
    game.display_to_board();
    loop {
        let point = player_helper();
        if let Err(a) = game.player_put_piece(point,'○') {
            println!("{}",a);
        }
        game.display_to_board();
        if let Some(msg) = game.becama_bingo() {
            println!("{}",msg);
            break;
        }
        println!("-------------");
        let point = player_helper();
        if let Err(a) = game.player_put_piece(point,'✕') {
            println!("{}",a);
        }
        game.display_to_board();
        if let Some(msg) = game.becama_bingo() {
            println!("{}",msg);
            break;
        }
    }
   
}
