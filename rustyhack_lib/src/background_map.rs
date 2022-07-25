pub mod character_map;
pub mod tiles;

use crate::background_map::tiles::Tile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundMap {
    pub data: Vec<Vec<Tile>>,
}

impl BackgroundMap {
    #[must_use]
    pub fn data(&self) -> &Vec<Vec<Tile>> {
        &self.data
    }

    #[must_use]
    pub fn get_tile_at(&self, x: u32, y: u32) -> Tile {
        self.data[y as usize][x as usize]
    }
}

pub type AllMaps = HashMap<String, BackgroundMap>;

pub type AllMapsChunk = (usize, Vec<u8>);
