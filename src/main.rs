#![allow(unused)]

use std::io::prelude::*;
use ndarray::{Array2, ArrayViewMut1, Axis, Slice, s, array};
use rand::prelude::*;

/// Shifts all tiles to the left (in place) and returns whether or not any
/// tiles were shifted
fn slide_tiles(mut tiles: ArrayViewMut1<i32>) -> bool {
  let mut tile_moved = false;
  let mut check_index = 0;
  for i in 1..tiles.len() {
    if tiles[i] == 0 {
      continue;
    } else if tiles[check_index] == tiles[i] {
      tiles[check_index] *= 2;
      tiles[i] = 0;
      check_index += 1;
      tile_moved = true;
    } else if tiles[check_index] == 0 {
      tiles[check_index] = tiles[i];
      tiles[i] = 0;
      tile_moved = true;
    } else if check_index + 1 == i {
      check_index += 1;
    } else {
      tiles[check_index + 1] = tiles[i];
      check_index += 1;
      tiles[i] = 0;
      tile_moved = true;
    }
  }
  tile_moved
}

#[cfg(test)]
mod slide_tiles_tests {
  use super::slide_tiles;
  use ndarray::{array, ArrayViewMut1, Array1};

  fn helper(array: [i32; 4], expected: [i32; 4]) {
    let should_tiles_move = array != expected;
    let mut array: Array1<i32> = array.into_iter().collect();
    let expected: Array1<i32> = expected.into_iter().collect();
    let did_tiles_move = slide_tiles(array.view_mut());
    assert_eq!(array, expected);
    assert_eq!(did_tiles_move, should_tiles_move);
  }

  #[test]
  fn single_tile() {
    helper([0, 0, 2, 0], [2, 0, 0, 0]);
  }

  #[test]
  fn converge() {
    helper([2, 0, 2, 0], [4, 0, 0, 0]);
  }

  #[test]
  fn no_movement() {
    helper([2, 4, 2, 4], [2, 4, 2, 4]);
  }

  #[test]
  fn multiple_converge() {
    helper([2, 2, 2, 2], [4, 4, 0, 0]);
  }

  #[test]
  fn converge_in_middle() {
    helper([2, 4, 4, 2], [2, 8, 2, 0]);
  }

  #[test]
  fn move_without_converge() {
    helper([2, 0, 4, 2], [2, 4, 2, 0]);
  }

  #[test]
  fn converge_order() {
    helper([0, 2, 2, 2], [4, 2, 0, 0]);
  }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
  Down,
  Up,
  Right,
  Left,
}

impl Direction {
  pub fn to_str(&self) -> &'static str {
    match self {
      Direction::Down => "down",
      Direction::Up => "up",
      Direction::Right => "right",
      Direction::Left => "left",
    }
  }
}

fn handle_input(grid: &mut Array2<i32>, slide_direction: Direction) -> bool {
  let mut tiles_moved = false;
  match slide_direction {
    Direction::Down => {
      for mut column in grid.columns_mut() {
        let reversed_column = column.slice_mut(s![..; -1]);
        tiles_moved |= slide_tiles(reversed_column);
      }
    }
    Direction::Up => {
      for column in grid.columns_mut() {
        tiles_moved |= slide_tiles(column);
      }
    }
    Direction::Right => {
      for mut row in grid.rows_mut() {
        let reversed_row = row.slice_mut(s![..; -1]);
        tiles_moved |= slide_tiles(reversed_row);
      }
    }
    Direction::Left => {
      for row in grid.rows_mut() {
        tiles_moved |= slide_tiles(row);
      }
    }
  }
  tiles_moved
}

/// Randomly inserts a tile into the grid in an empty slot and returns whether
/// the insertion was able to happen
fn insert_random_tile(grid: &mut Array2<i32>, tile_value: i32) -> bool {
  // This could use a HashSet instead and maintain it between calls to avoid
  // rebuilding `open_slots` on every call
  let mut open_slots = Vec::new(); 
  for x in 0..grid.shape()[0] {
    for y in 0..grid.shape()[1] {
      if grid[[x, y]] == 0 {
        open_slots.push([x, y]);
      }
    }
  }
  if let Some([x, y]) = open_slots.choose(&mut rand::thread_rng()) {
    grid[[*x, *y]] = tile_value;
    true
  } else {
    false
  } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut grid = array![
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
  ];
  insert_random_tile(&mut grid, 2);
  insert_random_tile(&mut grid, 4);
  println!("(use hjkl to move tiles and q to quit)");
  loop {
    println!("{grid}");
    print!("> ");
    std::io::stdout().flush()?; // needed since stdin is line buffered
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let slide_direction = match input.trim() {
      "j" => Direction::Down,
      "k" => Direction::Up,
      "l" => Direction::Right,
      "h" => Direction::Left,
      "q" | "x" => break,
      invalid => {
        println!("invalid input: {invalid:?}");
        continue;
      }
    };
    if !handle_input(&mut grid, slide_direction) {
      println!("unable to move grid {}, try again", slide_direction.to_str());
      continue;
    }
    if !insert_random_tile(&mut grid, 2) {
      let max_tile = grid.fold(0, |highest, current| i32::max(highest, *current));
      println!("grid has been filled, your highest tile was {max_tile}");
      break;
    }
  }
  Ok(())
}
