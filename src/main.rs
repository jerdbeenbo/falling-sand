use macroquad::{miniquad::window::set_window_size, prelude::*, rand::gen_range};
use std::thread;

/*

    [0][0][0][0][0][0][0]
    [0][0][0][1][0][0][0]       <- 1 represents a sand
    [0][0][0][0][2][0][0]       <- 2 represents water :TODO:
    [0][0][0][0][0][0][0]
    [0][0][0][0][0][0][0]
    [0][0][0][0][0][0][0]



    CA (Cellular Automata rules):
    SAND
        1. If a cell is a 1, and the cell below it is a zero:
            - Current cell becomes a 0.
            - Cell below beomces a 1.
        2. If a cell is a 1, and the cellow below is a 1:
            - We are resting, do nothing...

    WATER
        1. TODO...

*/

//TODO: Implement dirty-rectangle tracking


struct ParticleGrid {
    grid: Vec<Vec<i8>>,
    cols: usize,
    rows: usize,
    cell_size: usize,
}

impl ParticleGrid {

    ///Creates a new grid with specified cols, rows and cell size
    fn new_grid(cell_size: usize) -> Self {

        let width = 1200;
        let height = 800;


        let _cell_size: usize = 4;
        let _cols = width as usize / cell_size;
        let _rows = height as usize / cell_size;

        let _grid: Vec<Vec<i8>> = vec![vec![0; _cols]; _rows];


        ParticleGrid {              //Return
            grid: _grid,
            cols: _cols,
            rows: _rows,
            cell_size: _cell_size,
        }
    }


}

///Function to draw a grid of appropraite size to the screen, takes current height and width as parameters
fn d_grid(grid: &Vec<Vec<i8>>, w: f32, c: Color) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 { //is sand
                let color: Color = if grid[row][col] == 0 { c } else { YELLOW };
                draw_rectangle((col as f32) * w, (row as f32) * w, w, w, color);            
            }
        }
    }
}

///Function that takes both grids (borrowed reference to the first) and evaluates
/// what the next grid for the next frame should look like based on CA rules
fn eval_next(grid: ParticleGrid, mut next: ParticleGrid) -> ParticleGrid {
    //Loop grid and retroactively assign next positions (only filling in 1s)
    for row in 0..grid.grid.len() {
        for col in 0..grid.grid[row].len() {
            let state = grid.grid[row][col];
            if state == 1 {
                //Sand is present in cell
                if row == grid.grid.len() - 1 {
                    //Am I in the bottom row?
                    //settle
                    next.grid[row][col] = 1;
                } else if grid.grid[row + 1][col] == 0 && col < grid.grid[0].len() {
                    //Can I fall?
                    //fall
                    next.grid[row + 1][col] = 1;
                } else if grid.grid[row + 1][col] == 1 {
                    //Is there sand below me?
                    //sand is bellow

                    //check if only 1, or both directions are free
                    if col <= 0 {
                        //we are on the left wall

                        //see if we can fall right, otherwise stay in place
                        if grid.grid[row + 1][col + 1] == 0 {
                            //fall right
                            next.grid[row + 1][col + 1] = 1;
                        } else {
                            next.grid[row][col] = 1;
                        }
                    } else if col >= grid.grid[row].len() - 1 {
                        //we are on the right wall

                        //see if we can fall left, otherwise stay in place
                        if grid.grid[row + 1][col - 1] == 0 {
                            //fall right
                            next.grid[row + 1][col - 1] = 1;
                        } else {
                            next.grid[row][col] = 1;
                        }
                    } else {
                        //we aren't on an edge
                        //left or right entropy
                        let mut left = true;
                        let rand = gen_range(0, 2);
                        if rand == 1 {
                            left = false;
                        }

                        let belowL = grid.grid[row + 1][col - 1];
                        let belowR = grid.grid[row + 1][col + 1];

                        //check if both are free
                        if belowL == 0 && belowR == 0 {
                            //choose which way to fall
                            if left {
                                next.grid[row + 1][col - 1] = 1;
                            } else {
                                next.grid[row + 1][col + 1] = 1;
                            }
                        } else if belowL == 0 {
                            next.grid[row + 1][col - 1] = 1;
                        } else if belowR == 0 {
                            next.grid[row + 1][col + 1] = 1;
                        } else {
                            //can't fall further, settle
                            next.grid[row][col] = 1;
                        }
                    }
                }
            }
        }
    }
    
    next            //return the vec to keep ownership
}

#[macroquad::main("MyGame")]
async fn main() {
    let backgroundc: Color = Color::new(148.0, 146.0, 142.0, 0.0);

    let width = 1200;
    let height = 800;
    set_window_size(width, height);

    //Set up left mouse button reading
    let btn: MouseButton = MouseButton::Left;

    //Initialise the grid
    let mut grid: ParticleGrid = ParticleGrid::new_grid(4);

    loop {
        //Draw grid and background
        clear_background(LIGHTGRAY);
        d_grid(&grid.grid, grid.cell_size as f32, backgroundc);

        //Create the next grid by checking current grid and reassigning 1s
        let mut next_grid: ParticleGrid = ParticleGrid::new_grid(4);

        //spawn sand on mouse position
        if is_mouse_button_down(btn) {
            let mp = mouse_position();

            let row = (mp.1 / grid.cell_size as f32).floor();
            let col = (mp.0 / grid.cell_size as f32).floor();

            grid.grid[row as usize][col as usize] = 1;
        }

        //keep ownership scope of grid variable
        next_grid = eval_next(grid, next_grid);

        //grid equals next
        grid = next_grid;

        next_frame().await
    }
}
