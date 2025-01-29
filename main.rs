use std::io::{self};

fn main() {
    // Get grid size from user
    println!("Enter grid size (rows cols): ");
    let (rows, cols) = get_grid_size();

    // Initialize grid with user input
    let mut grid = vec![vec![false; cols]; rows];
    println!("Enter live cells (row col). Type 'done' to finish:");
    initialize_grid(&mut grid);

    // Get the number of generations to simulate
    println!("Enter the number of generations to simulate:");
    let generations = get_number_of_generations();

    // Simulate and display the generations
    for gen in 1..=generations {
        println!("\nGeneration {}:", gen);
        display_grid(&grid);
        grid = next_generation(&grid);
    }
}

// Function to get grid size from user
fn get_grid_size() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    if parts.len() != 2 {
        panic!("Invalid input. Please provide two numbers for rows and columns.");
    }
    (parts[0], parts[1])
}

// Function to initialize grid with user input
fn initialize_grid(grid: &mut Vec<Vec<bool>>) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("done") {
            break;
        }

        let parts: Vec<usize> = input
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        if parts.len() != 2 {
            println!("Invalid input. Enter row and column (e.g., '2 3') or 'done' to finish.");
            continue;
        }

        let (row, col) = (parts[0], parts[1]);
        if row >= grid.len() || col >= grid[0].len() {
            println!("Invalid cell position. Please enter values within the grid size.");
        } else {
            grid[row][col] = true;
        }
    }
}

// Function to get the number of generations from user
fn get_number_of_generations() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().expect("Invalid number.")
}

// Function to display the grid
fn display_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &cell in row {
            print!("{}", if cell { "■ " } else { "□ " });
        }
        println!();
    }
}

// Function to compute the next generation
fn next_generation(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![false; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            let live_neighbors = count_live_neighbors(grid, row, col);
            new_grid[row][col] = match (grid[row][col], live_neighbors) {
                (true, 2) | (true, 3) => true,  // Survives
                (false, 3) => true,             // Becomes alive
                _ => false,                     // Dies or remains dead
            };
        }
    }

    new_grid
}

// Function to count live neighbors for a cell
fn count_live_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    directions.iter().filter_map(|&(dr, dc)| {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            Some(grid[new_row as usize][new_col as usize])
        } else {
            None
        }
    }).filter(|&cell| cell).count()
}

