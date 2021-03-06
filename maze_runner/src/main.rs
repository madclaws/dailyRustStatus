mod cell;
mod grid;
mod binary_tree;
// use cell::*;
use grid::*;
// use binary_tree::*;
fn main() {
    println!("Mazes mazes everywhere..");

    let mut grid = Grid::new(4, 4);
    grid.configure_cells();
    binary_tree::on(&mut grid);
    grid.render();
}
