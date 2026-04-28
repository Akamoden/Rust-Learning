

// preface : this project was out of my scope. i did my best but had Claude redo my logic for alot of it.
//despite all this i can follow whats happening pretty well. but i wish i could write it as well as i understand it.

// todo : rewrite this 100% for myself and add a player and movments. ~2 weeks




use macroquad::prelude::*;
const CELL_SIZE: f32 = 8.0;
#[derive(PartialEq)]
enum Cell {
    Open,
    Wall,
}
    

struct Grid {
    width:usize,
    height:usize,
    barrier:Vec<Vec<Cell>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let barrier = (0..height).map(|_| {
            (0..width).map(|_| Cell::Wall).collect()
        }).collect();

        Grid { width, height, barrier }
    }
}

fn carve(grid: &mut Grid, x: usize, y: usize) {
    grid.barrier[y][x] = Cell::Open;
    
    let mut rng = ::rand::rng();
    let mut directions = [(0i32, -2), (0, 2), (-2, 0), (2, 0)];
    ::rand::seq::SliceRandom::shuffle(&mut directions[..], &mut rng);
    
    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        
        if new_x >= 0 && new_x < grid.width as i32 -1
           && new_y >= 0 && new_y < grid.height as i32 -1 {
            
            let nx = new_x as usize;
            let ny = new_y as usize;
            
            if grid.barrier[ny][nx] == Cell::Wall {
                let wall_x = (x as i32 + dx / 2) as usize;
                let wall_y = (y as i32 + dy / 2) as usize;
                grid.barrier[wall_y][wall_x] = Cell::Open;
                
                carve(grid, nx, ny);
            }
        }
    }
}

#[macroquad::main("Maze Runner")]
async fn main() {
    let mut grid = Grid::new(99, 71);
    carve(&mut grid, 1, 1);
    grid.barrier[0][1] = Cell::Open;                                    // start
    grid.barrier[grid.height - 1][grid.width - 2] = Cell::Open; //end

    loop {
        clear_background(BLACK);
        
        for (row_index, row) in grid.barrier.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                let x = col_index as f32 * CELL_SIZE;
                let y = row_index as f32 * CELL_SIZE;
                let color = match cell {
                    Cell::Open => WHITE,
                    Cell::Wall => BLACK,
                };
                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
            }
        }

        next_frame().await;
    }
}