#![allow(unused_variables, dead_code)]
fn main() {
  extern crate cfg_if;
  extern crate wasm_bindgen;

  #[path = "utils.rs"]
  mod utils;

  use cfg_if::cfg_if;
  use wasm_bindgen::prelude::*;

  cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
      extern crate wee_alloc;
      #[global_allocator]
      static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
  }

  // #[wasm_bindgen]
  // extern "C" {
  //   fn alert(s: &str);
  // }

  #[wasm_bindgen]
  #[repr(u8)]
  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  /**
   * This is used to keep track of which of
   * the two states a Cell can be at any time
   */
  pub enum Cell {
    Dead = 0,
    Alive = 1,
  }

  #[wasm_bindgen]
  // This defines the properties of Universe objects
  pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
  }

  // This defines the internal methods of Universe objects
  impl Universe {
    /**
     * Takes in the x and y coordinate of a Cell in a grid or universe and
     * returns the index of the cell inside of wasm's linear memory space
     */
    fn get_index(&self, row: u32, column: u32) -> usize {
      (row * self.width + column) as usize
    }

    /**
     * Takes in the row and column where the Cell exists and
     * returns the number of ajacent cells that are Alive
     */
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
      let mut count = 0;
      // Iterate through the y-axis offset for a Cell
      for delta_row in [self.height - 1, 0, 1].iter().cloned() {
        // Iterate through the x-axis offset for a Cell
        for delta_col in [self.width - 1, 0, 1].iter().cloned() {
          // skip over original cell
          if delta_row == 0 && delta_col == 0 {
            continue;
          }

          // Grabs neighbor location based on the location of the current Cell and the offsets
          // NOTE: The use of the modulo is to avoid special casing the edges of the universe
          let neighbor_row = (row + delta_row) % self.height;
          let neighbor_col = (column + delta_col) % self.width;
          let idx = self.get_index(neighbor_row, neighbor_col);
          count += self.cells[idx] as u8;
        }
      }
      count
    }
  }
}
