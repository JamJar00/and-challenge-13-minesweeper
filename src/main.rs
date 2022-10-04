fn main() {
    panic!("Main function not defined, consider running 'cargo test'");
}

pub fn minesweeper(grid: &Vec<char>) -> Vec<char> {
    let mut solved: Vec<char> = vec![];

    // Iterate over all tiles and compile into output, any - characters need solving
    for (pos, tile) in grid.iter().enumerate() {
        let solved_tile = match tile {
            'X' => 'X',
            '-' => calculate_mine_count(grid, pos).to_string().chars().last().unwrap(),
            _ => panic!("Invalid character")
        };

        solved.push(solved_tile);
    }

    return solved;
}

fn calculate_mine_count(grid: &Vec<char>, pos: usize) -> usize {
    // Check all neighbouring tiles for mines
    let n = try_get_tile(grid, pos, 0, -1) == 'X';
    let ne = try_get_tile(grid, pos, 1, -1) == 'X';
    let e = try_get_tile(grid, pos,  1, 0) == 'X';
    let se = try_get_tile(grid, pos, 1, 1) == 'X';
    let s = try_get_tile(grid, pos, 0, 1) == 'X';
    let sw = try_get_tile(grid, pos, -1, 1) == 'X';
    let w = try_get_tile(grid, pos, -1, 0) == 'X';
    let nw = try_get_tile(grid, pos, -1, -1) == 'X';

    // Count how many of the neighbouring tiles were actually mines
    return [n, ne, e, se, s, sw, w, nw].into_iter().filter(|b| *b).count();
}

fn try_get_tile(grid: &Vec<char>, pos: usize, offset_x: i8, offset_y: i8) -> char {
    // Provides a safe way of accessing a tile at a given position in relation to another

    // Work out the x/y coordinate of wherever we're trying to access
    let width = f64::sqrt(grid.len() as f64) as usize;
    let x = (pos % width) as isize + offset_x as isize;
    let y = (pos / width) as isize + offset_y as isize;

    // Check the x/y is in bounds of the board
    if x < 0 || x as usize >= width || y < 0 || y as usize >= width {
        return '-';
    }

    // Return the content
    return grid[y as usize * width + x as usize];
}


#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let input: Vec<char> = vec![
            '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-',
            'X', 'X', '-', '-', '-',
            '-', '-', '-', '-', '-',
            '-', '-', '-', '-', 'X'
        ];

        assert_eq!(minesweeper(&input),
        [
            '0', '0', '0', '0', '0',
            '2', '2', '1', '0', '0',
            'X', 'X', '1', '0', '0',
            '2', '2', '1', '1', '1',
            '0', '0', '0', '1', 'X'
        ]);
    }
}
