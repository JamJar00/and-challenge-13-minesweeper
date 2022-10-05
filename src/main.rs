mod solver;

use rand::Rng;
use std::io;

fn main() {
    let mut board = generate_board(5, 0);
    let mined_board = generate_board(5, 5);
    let solution = solver::minesweeper(&mined_board);

    loop {
        print_board(&board);
        process_move(&mut board, &solution).unwrap();
    }
}

fn generate_board(width: u16, mines: u8) -> Vec<char> {
    let mut board: Vec<char> = (0..width * width).map(|_| '-').collect();

    let mut rng = rand::thread_rng();
    let len = board.len();
    for i in 0..mines {
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

fn process_move(board: &mut Vec<char>, solution: &Vec<char>) -> Result<(), String> {
    let mut column_input = String::new();
    println!("Column:");
    io::stdin().read_line(&mut column_input).unwrap();

    let mut row_input = String::new();
    println!("Row:");
    io::stdin().read_line(&mut row_input).unwrap();

    let column = u16::from_str_radix(column_input.trim(), 10).map_err(|e| format!("Invalid column: {}", e))? - 1;
    let row = u16::from_str_radix(row_input.trim(), 10).map_err(|e| format!("Invalid row: {}", e))? - 1;

    let width = f64::sqrt(board.len() as f64) as usize;
    let pos = row as usize* width + column as usize;
    if solution[pos] == 'X' {
        panic!("Boom.");
    } else {
        board[pos] = solution[pos];
    }

    Ok(())
}
