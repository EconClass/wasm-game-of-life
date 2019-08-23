#![allow(unused_variables, dead_code)]
fn main() {
  extern crate cfg_if;
  extern crate wasm_bindgen;

  #[path = "utils.rs"]
  mod utils;

  use cfg_if::cfg_if;
  use wasm_bindgen::prelude::*;

  cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
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
  /* This is used to keep track of which of
  the two states a Cell can be at any time */
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

  // This defines the methods of Universe objects
  impl Universe {
    /*
      @params: row and column number of a Cell in a grid
      @returns: index of the cell inside of wasm's linear memory space
    */
    fn get_index(&self, row: u32, column: u32) -> usize {
      (row * self.width + column) as usize
    }
  }
}
