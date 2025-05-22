use macroquad::{miniquad::window::{set_window_position, set_window_size}, prelude::*};

/*

    [0][0][0][0][0][0][0]
    [0][0][0][1][0][0][0]       <- 1 represents a sand
    [0][0][0][0][0][0][0]
    [0][0][0][0][0][0][0]
    [0][0][0][0][0][0][0]
    [0][0][0][0][0][0][0]



    CA (Cellular Automata rules):
        1. If a cell is a 1, and the cell below it is a zero:
            - Current cell becomes a 0.
            - Cell below beomces a 1.
        2. If a cell is a 1, and the cellow below is a 1:
            - We are resting, do nothing...

*/


///Takes a grid size and returns a 2 dimensional array populated with 0s
fn pop_2d_grid(cols: usize, rows: usize) -> Vec<Vec<i8>>{

    let mut grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    //Return the grid
    grid
}

///Function to draw a grid of appropraite size to the screen, takes current height and width as parameters
fn d_grid(grid: &Vec<Vec<i8>>, w: f32) {

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let color: Color = if grid[row][col] == 0 {MAROON} else {YELLOW};
            draw_rectangle((col as f32) * w, (row as f32) * w, w, w, color);
        }
    }

}

///Function that takes both grids (borrowed reference to the first) and evaluates
/// what the next grid for the next frame should look like based on CA rules
fn eval_next(grid: &Vec<Vec<i8>>, mut next: Vec<Vec<i8>>) -> Vec<Vec<i8>>{

    //Loop grid and retroactively assign next positions (only filling in 1s)
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let state = grid[row][col];
            if state == 1 {                                             //Sand is present in cell
                if row == grid.len() - 1 {                              //Am I in the bottom row?
                    //settle
                    next[row][col] = 1;
                }
                else if grid[row + 1][col] == 0 && col < grid.len() {   //Can I fall?
                    //fall
                    next[row + 1][col] = 1;
                }
                else if grid[row+1][col] == 1 {                         //Is there sand below me?
                    //settle
                    next[row][col] = 1;
                }
            }
        }
    }

    next
}

#[macroquad::main("MyGame")]
async fn main() {

    let width = 1200;
    let height = 800;
    set_window_size(width, height);

    //Set up left mouse button reading
    let btn: MouseButton = MouseButton::Left;

    //Initialise the grid
    let w: usize = 5;
    let cols = width as usize / w;
    let rows = height as usize / w;
    let mut grid: Vec<Vec<i8>> = pop_2d_grid(cols, rows);


    loop {
        
        //Draw grid and background
        clear_background(LIGHTGRAY);
        d_grid(&grid, w as f32);

        //Create the next grid by checking current grid and reassigning 1s
        let mut next_grid: Vec<Vec<i8>> = pop_2d_grid(cols, rows);

        //spawn sand on mouse position
        if is_mouse_button_down(btn) {
            let mp = mouse_position();
            
            let row = (mp.1 / w as f32).floor();
            let col = (mp.0 / w as f32).floor();

            grid[row as usize][col as usize] = 1;
        }
        
        //keep ownership scope of grid variable
        next_grid = eval_next(&grid, next_grid);

        //grid equals next
        grid = next_grid;

        next_frame().await
    }
}