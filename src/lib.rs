use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

///An algebraic cell data type to hold information on if it is alive and its current neighbours
#[derive(Clone)]
struct Cell {
    alive: bool,
    alive_neighbours: Vec<(usize,usize)>,   //record a tuple of their position in the grid
}

impl Cell {
    fn blank_slate(rows: usize, cols: usize) -> Vec<Vec<Cell>> {
        let blank_cell: Cell = Cell {
            alive: false,
            alive_neighbours: Vec::new(),
        };

        vec![vec![blank_cell; cols]; rows]
    }
}

static mut STATE: Option<(Vec<Vec<Cell>>, Vec<Vec<Cell>>)> = None;

#[derive(Serialize, Deserialize)]
pub struct JObject {
    rows: usize,
    cols: usize,
    active_particles: Vec<(usize,usize)>,
}

impl JObject {
    fn new(rows: usize, cols: usize) -> Self {
        JObject { 
            rows, 
            cols, 
            active_particles: Vec::new() 
        }
    }
}

///Init function
#[wasm_bindgen]
pub fn wasm_bridge_init() {
    unsafe {
        if STATE.is_none() {
            let cell_size: usize = 5;
            let width = 1200;
            let height = 800;
            let cols = width / cell_size;
            let rows = height / cell_size;
            
            let grid = Cell::blank_slate(rows, cols);
            let next_grid = Cell::blank_slate(rows, cols);
            STATE = Some((grid, next_grid));
        }
    }
}

#[wasm_bindgen]
pub fn get_current_state() -> Result<JsValue, JsValue> {
    let cell_size: usize = 5;
    let width = 1200;
    let height = 800;
    let cols = width / cell_size;
    let rows = height / cell_size;

    unsafe {
        if let Some((ref grid, _)) = STATE {
            let jsobj: JObject = create_json_object(grid, rows, cols);
            Ok(serde_wasm_bindgen::to_value(&jsobj)?)
        } else {
            Err(JsValue::from_str("Failed to initialize simulation"))
        }
    }
}

#[wasm_bindgen]
pub fn add_cell(row: usize, col: usize) -> Result<(), JsValue> {
    unsafe {
        if let Some((ref mut grid, _)) = STATE {
            if row < grid.len() && col < grid[0].len() {
                grid[row][col].alive = true;
                Ok(())
            } else {
                Err(JsValue::from_str("Coordinates out of bounds"))
            }
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    }
}

///Web Assembly wrapping layer
#[wasm_bindgen]
pub fn wasm_bridge_update() -> Result<JsValue, JsValue> {
    let cell_size: usize = 5;
    let width = 1200;
    let height = 800;
    let cols = width / cell_size;
    let rows = height / cell_size;

    unsafe {
        if let Some((ref mut grid, ref mut next_grid)) = STATE {
            // Update neighbor counts
            update_neighbor_counts(grid);
            
            // Evaluate next generation
            eval_next(grid, next_grid);

            // Swap grids
            std::mem::swap(grid, next_grid);

            let jsobj: JObject = create_json_object(grid, rows, cols);
            
            Ok(serde_wasm_bindgen::to_value(&jsobj)?)
        } else {
            Err(JsValue::from_str("Failed to initialize simulation"))
        }
    }
}

/// Create a json object that can be sent to the visual layer (javascript) via wasm
fn create_json_object(grid: &Vec<Vec<Cell>>, rows: usize, cols: usize) -> JObject {
    let mut obj: JObject = JObject::new(rows, cols);
    
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col].alive {
                obj.active_particles.push((row, col));     
            }
        }
    }

    obj
}

fn update_neighbor_counts(cells: &mut Vec<Vec<Cell>>) {
    // Clear previous neighbor data
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            cells[i][j].alive_neighbours.clear();
        }
    }

    // Count neighbors for each cell
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            // Check all 8 neighbors
            for di in -1i32..=1i32 {
                for dj in -1i32..=1i32 {
                    if di == 0 && dj == 0 { continue; } // Skip self
                    
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    
                    // Check bounds
                    if ni >= 0 && ni < cells.len() as i32 && 
                       nj >= 0 && nj < cells[0].len() as i32 {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        
                        if cells[ni][nj].alive {
                            cells[i][j].alive_neighbours.push((ni, nj));
                        }
                    }
                }
            }
        }
    }
}

fn eval_next(cells: &Vec<Vec<Cell>>, next: &mut Vec<Vec<Cell>>) {
    // Clear the next grid
    for i in 0..next.len() {
        for j in 0..next[i].len() {
            next[i][j].alive = false;
            next[i][j].alive_neighbours.clear();
        }
    }

    // Apply Conway's Game of Life rules
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            let neighbor_count = cells[i][j].alive_neighbours.len();
            let is_alive = cells[i][j].alive;
            
            if is_alive {
                // Rules for live cells
                if neighbor_count == 2 || neighbor_count == 3 {
                    next[i][j].alive = true;  // Survival
                }
                // Otherwise cell dies (underpopulation or overpopulation)
            } else {
                // Rules for dead cells
                if neighbor_count == 3 {
                    next[i][j].alive = true;  // Reproduction
                }
                // Otherwise cell stays dead
            }
        }
    }
}