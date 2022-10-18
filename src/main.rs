mod solver;

use rand::Rng;
use std::io;

const WIDTH: u16 = 3;
const MINES: u8 = 1;

fn main() {
    let mut board = generate_board(WIDTH, 0);
    let mined_board = generate_board(WIDTH, MINES);
    let solution = solver::minesweeper(&mined_board);

    loop {
        print_board(&board);
        match process_move(&mut board, &solution) {
            Ok(true) => {
                print_board(&board);
                println!();
                println!("Ouch! That's going to hurt in the morning.");
                break;
            },
            Ok(false) => {
                if is_win(&board, &solution) {
                    print_board(&board);
                    println!();
                    println!("Phew! Safe and sound.");
                    break;
                }
            },
            Err(_) => {
                println!();
                println!("Bad input. Bad human.");
            }
        }
    }
}

fn generate_board(width: u16, mines: u8) -> Vec<char> {
    let mut board: Vec<char> = (0..width * width).map(|_| '-').collect();

    let mut rng = rand::thread_rng();
    let len = board.len();
    for _ in 0..mines {
        board[rng.gen_range(0..len)] = 'X';
    }

    return board
}

fn print_board(board: &Vec<char>) {
    let width = f64::sqrt(board.len() as f64) as usize;

    println!();
    println!();
    println!();
    print!("  {}", (1..width + 1).map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
    for i in 0..board.len(){
        if i % width == 0 {
            print!("\n{} ", i / width + 1);
        }
        print!("{} ", board[i]);
    }
    println!();
}

fn process_move(board: &mut Vec<char>, solution: &Vec<char>) -> Result<bool, String> {
    let mut column_input = String::new();
    println!("Column:");
    io::stdin().read_line(&mut column_input).unwrap();

    let mut row_input = String::new();
    println!("Row:");
    io::stdin().read_line(&mut row_input).unwrap();

    let column = u16::from_str_radix(column_input.trim(), 10).map_err(|e| format!("Invalid column: {}", e))? - 1;
    let row = u16::from_str_radix(row_input.trim(), 10).map_err(|e| format!("Invalid row: {}", e))? - 1;

    let width = f64::sqrt(board.len() as f64) as usize;
    if column as usize > width - 1 || row as usize > width - 1 {
        return Err("Out of bounds".to_string());
    }

    let pos = row as usize * width + column as usize;
    if solution[pos] == 'X' {
        board[pos] = 'X';
        Ok(true)
    } else {
        board[pos] = solution[pos];
        Ok(false)
    }
}

fn is_win(board: &Vec<char>, solution: &Vec<char>) -> bool {
    // We're a win if the board equals the solution
    board.len() == solution.len() && board.iter()
                              .zip(solution)
                              .all(|(b, s)| *b == *s || (*b == '-' && *s == 'X'))
}
