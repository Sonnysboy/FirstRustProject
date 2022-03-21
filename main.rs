use std::io::Write;
use {std, std::io};

struct Move {
    x: usize,
    y: usize,
}

enum Turn {
    X,
    O,
	DRAW // too lazy to make a state enum.
}

impl Turn {
    fn get_marker(_this: Self) -> char {
        match _this {
            Turn::X => 'x',
            Turn::O => 'o',
			_ => '_'
        }
    }
    fn get_turn(_turn: u8) -> Turn {
        match _turn {
            1 => Self::X,
            _ => Self::O,
        }
    }

	fn from_marker(marker: char) -> Option<Turn> {

		match marker {
			'x' => Some(Turn::X),
			'o' => Some(Turn::O),
			_ => None
		}
	}
}


// calcs if theres a winner.
fn get_winner(board: &mut [[char; 3]; 3], mv: Move, turn_count: u8, marker: char) -> Option<Turn> {


	let n = 3usize;
	let turn = Turn::from_marker(marker);

	for i in 0..n {
		if board[mv.x][i] != marker {
			break;
		}
		if i == n - 1 { return turn; }
	}
	for i in 0..n {
		if board[i][mv.y] != marker {
			break;
		}
		if i == n - 1 { return turn; }
	}
	if mv.x == mv.y {

		for i in 0..n {
			if board[i][i] != marker {
				break;
			}
			if i == n - 1 { return turn; }
		}
	}
	// thank you stackoverflow for this one.
	if mv.x + mv.y == n - 1 {
		for i in 0..n {

			if board[i]
				[(n - 1) - i] != marker {
				break;
				}
			
		
		if i == n - 1 {
			return turn;
		}	
	}
	}
	if usize::from(turn_count) == (n * n) {
		return Some(Turn::DRAW);
		// todo make a draw state.
	}
	None
	
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
	
    fn do_turn(board: &mut [[char; 3]; 3], _turn: &u8, total_turns: u8) {
        let mv = get_move();

        if board[mv.x][mv.y] != '_' {
            println!("That is not a valid move (It is taken)");
            do_turn(board, _turn, total_turns);
            return;
        }

		let marker = Turn::get_marker(Turn::get_turn(*_turn));

		board[mv.x][mv.y] = marker;


		match get_winner(board, mv, total_turns, marker) {
			Some(x) =>  { 
				match x {
					Turn::X => println!("X WINS"),
					Turn::O => println!("O WINS!"),
					Turn::DRAW => println!("It's a draw!")
				}
				print_board(board);
				return;
			}
			_ => ()
		}
		
        print_board(board);
        do_turn(board, if *_turn == 1 { &2 } else { &1 }, total_turns + 1);
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

    do_turn(&mut board, &1, 1);
}
