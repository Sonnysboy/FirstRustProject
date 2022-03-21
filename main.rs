use std::io::Write;
use {std, std::io};

struct Move {
    x: usize,
    y: usize,
}

enum Turn {
    X,
    O,
}

impl Turn {
    fn get_marker(_this: Self) -> char {
        match _this {
            Turn::X => 'x',
            Turn::O => 'o',
        }
    }
    fn get_turn(_turn: u8) -> Turn {
        match _turn {
            1 => Self::X,
            _ => Self::O,
        }
    }
}

fn print_board(board: &mut [[char; 3]; 3]) {
    for i in 0..board.len() {
        if i > 0 {
            println!("---+---+---");
        }
        for j in 0..board[i].len() {
            if j > 0 {
                print!("|");
            }
            print!(" {} ", board[i][j]);
            std::io::stdout().flush().unwrap();
        }
        println!("")
    }
}

fn main() {
    let mut board = [['_'; 3]; 3];

    fn do_turn(board: &mut [[char; 3]; 3], _turn: &u8) {
        let mv = get_move();

        if board[mv.x][mv.y] != '_' {
            println!("That is not a valid move (It is taken)");
            do_turn(board, _turn);
            return;
        }

        board[mv.x][mv.y] = Turn::get_marker(Turn::get_turn(*_turn));
        print_board(board);
        do_turn(board, if *_turn == 1 { &2 } else { &1 });
    }

    fn get_move() -> Move {
        let mut m = Move { x: 0, y: 0 };
        println!("Please input row");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error!");

        m.x = input.trim().parse().expect("That is not a number.");

        println!("Please enter column.");

        let mut second_input = String::new();

        io::stdin().read_line(&mut second_input).expect("Error!");

        m.y = second_input.trim().parse().expect("That is not a number.");
        m
    }
	print_board(&mut board);

    do_turn(&mut board, &1);
}
