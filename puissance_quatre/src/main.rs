mod gui;
mod grid;
//use gui::creer_fenetre;
use grid::{grid_create, grid_main};

const GRID_COLUMN_NB: i32 = 7;
const GRID_ROW_NB: i32 = 6;

fn main() {
    println!("Hello, world!");
    //creer_fenetre();
    let game_grid = grid_create(GRID_COLUMN_NB, GRID_ROW_NB);
    grid_main(game_grid);
}
