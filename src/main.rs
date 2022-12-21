use nannou::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;


struct Life {
    grid: Vec<Vec<bool>>,
}

impl Life {
    fn is_alive(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }

    // todo fix this
    fn is_alive_i32(&self, x: i32, y: i32) -> bool {
        self.grid[y as usize][x as usize]
    }

    fn _is_inside(&self, x: i32, y: i32) -> bool {
        if x >= 0 && y >= 0 && y < self.grid.len() as i32 && x < self.grid[y as usize].len() as i32
        {
            true
        } else {
            false
        }
    }

    fn alive_neighbor_count(&self, _x: usize, _y: usize) -> i32 {
        let x: i32 = _x as i32;
        let y: i32 = _y as i32;

        let topleft: bool = if self._is_inside(x - 1, y - 1) {
            self.is_alive_i32(x - 1, y - 1)
        } else {
            false
        };
        let top: bool = if self._is_inside(x, y - 1) {
            self.is_alive_i32(x, y - 1)
        } else {
            false
        };
        let topright: bool = if self._is_inside(x + 1, y - 1) {
            self.is_alive_i32(x + 1, y - 1)
        } else {
            false
        };
        let midleft: bool = if self._is_inside(x - 1, y) {
            self.is_alive_i32(x - 1, y)
        } else {
            false
        };
        let midright: bool = if self._is_inside(x + 1, y) {
            self.is_alive_i32(x + 1, y)
        } else {
            false
        };
        let bottomleft: bool = if self._is_inside(x - 1, y + 1) {
            self.is_alive_i32(x - 1, y + 1)
        } else {
            false
        };
        let bottom: bool = if self._is_inside(x, y + 1) {
            self.is_alive_i32(x, y + 1)
        } else {
            false
        };
        let bottomright: bool = if self._is_inside(x + 1, y + 1) {
            self.is_alive_i32(x + 1, y + 1)
        } else {
            false
        };

        let neighbors = [
            topleft,
            top,
            topright,
            midleft,
            midright,
            bottomleft,
            bottom,
            bottomright,
        ];
        //println!("{:?}", (x, y, neighbors));

        neighbors.into_iter().fold(0, |mut n, neighbor_is_alive| {
            if neighbor_is_alive {
                n += 1;
            }
            n
        })
    }

    fn get_max_x(&self) -> usize {
        let mut max_x: usize = 0;
        for y in &self.grid {
            for (i, x) in y.into_iter().enumerate() {
                if max_x < i {
                    max_x = i;
                }
            }
        }
        max_x
    }

    fn get_max_y(&self) -> usize {
        self.grid.len()
    }

    fn new(path: String) -> Life {
        let mut grid: Vec<Vec<bool>> = Vec::new();

        if let Ok(lines) = read_lines(path) {
            for (i, line) in lines.enumerate() {
                if let Ok(ip) = line {
                    grid.push(Vec::new());
                    for (_j, c) in ip.chars().enumerate() {
                        match c {
                            '-' => grid[i].push(false),
                            _ => grid[i].push(true),
                        }
                    }
                }
            }
        }
        Life { grid: grid }
    }

    // 1) Any live cell with fewer than two live neighbours dies (referred to as underpopulation or exposure[2]).
    // 2) Any live cell with more than three live neighbours dies (referred to as overpopulation or overcrowding).
    // 3) Any live cell with two or three live neighbours lives, unchanged, to the next generation.
    // 4) Any dead cell with exactly three live neighbours will come to life.
    // See https://conwaylife.com/wiki/Conway%27s_Game_of_Life#Rules
    fn compute_next(&mut self) -> () {
        let mut next_grid = self.grid.clone();
        for (y, row) in self.grid.clone().into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                let live_neighbors = self.alive_neighbor_count(x, y);

                if cell {
                    match live_neighbors {
                        // rule 1: underpop
                        n if n < 2 => next_grid[y][x] = false,
                        // rule 2: overpop
                        n if n > 3 => next_grid[y][x] = false,
                        // rule 3: continutation
                        _ => (),
                    };
                    continue;
                }
                // rule 4 spawn
                if live_neighbors == 3 {
                    next_grid[y][x] = true;
                }
            }
        }
        self.grid = next_grid;
    }
}

fn main() {
    // let mut game: Life = Life::new("./init.txt".to_string());
    // println!("{:?}", game.grid);
    nannou::app(state).update(update).simple_window(view).run();
}

struct State {
    compute_grid: Life,
    is_first_frame: bool
}

fn state(_app: &App) -> State {
    _app.main_window().set_title("Game of Life");

    State {
        compute_grid: Life::new("./init.txt".to_string()),
        is_first_frame: true
    }
}

fn update(_app: &App, state: &mut State, _update: Update) {
    thread::sleep(Duration::from_millis(256));

    if !state.is_first_frame {
        state.compute_grid.compute_next();

    }

    // println!("tick");
    state.is_first_frame = false; 
}

fn view(app: &App, state: &State, frame: Frame) {
    // frame.clear(BLACK);

    let draw = app.draw();
    draw.background().color(BLACK);

    let mut block_width = app.window_rect().w() / state.compute_grid.get_max_x() as f32 * 0.5;
    let mut block_height = app.window_rect().h() / state.compute_grid.get_max_y() as f32 * 0.5;
    if block_width > block_height {
        block_height = block_width;
    }
    if block_height > block_width {
        block_width = block_height;
    }

    // screen draws from center of shape and screen space
    // therefore, push to bottom-left corner; block_y does reflection
    let bottom_edge = block_height * 0.5;
    let left_edge = block_width * 0.5;

    let screen_bottom_edge = app.window_rect().h() * 0.5;
    let screen_left_edge = app.window_rect().w() * 0.5;

    for i in 0..state.compute_grid.grid.len() {
        for j in 0..state.compute_grid.grid[i].len() {
            if state.compute_grid.is_alive(j, i) {
                let block_x = j as f32 * block_width + left_edge - screen_left_edge;
                let block_y = -(i as f32 * block_height) - bottom_edge + screen_bottom_edge;

                draw.rect()
                    .color(WHITE)
                    .x(block_x)
                    .y(block_y)
                    .w(block_width)
                    .h(block_height);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
