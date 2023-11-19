#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Serialize};

use crate::Coord;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Cell {
    // una cella vuota non viene più riconosciuta dal player fuori range ma da atoms == 0
    //
    // una cella contiene massimo 5 atomi (3 bit)
    //
    // gli unici valori ammissibili per `max_atoms` sono 2, 3 e 4
    // per risparmiare un bit scalo tutto di 2 (0, 1 e 2)
    //
    // giocatori massimi 8, quindi indici arrivano fino a 7 e bastano 3 bit
    //
    // pppmmaaa
    value: u8,
}

impl Cell {
    pub(crate) fn new((row, col): Coord, height: usize, width: usize) -> Self {
        let is_horizontal_edge = row == 0 || row == height - 1;
        let is_vertical_edge = col == 0 || col == width - 1;
        let max_atoms = if is_horizontal_edge && is_vertical_edge {
            2
        } else if is_horizontal_edge || is_vertical_edge {
            3
        } else {
            4
        };
        Self {
            value: (max_atoms - 2) << 3,
        }
    }

    #[cfg(test)]
    pub(crate) fn from(atoms: u8, player: usize, max_atoms: u8) -> Self {
        Self {
            value: ((player as u8) << 5) | ((max_atoms - 2) << 3) | atoms,
        }
    }

    pub fn atoms(&self) -> u8 {
        self.value & 0b00000111
    }

    pub(crate) fn add_atom(&mut self) {
        self.value += 1;
    }

    pub fn max_atoms(&self) -> u8 {
        ((self.value & 0b00011000) >> 3) + 2
    }

    pub fn player(&self) -> usize {
        ((self.value & 0b11100000) >> 5) as usize
    }

    pub(crate) fn set_player(&mut self, player: usize) {
        self.value = (self.value & 0b00011111) | ((player as u8) << 5);
    }

    pub fn must_explode(&self) -> bool {
        self.atoms() >= self.max_atoms()
    }

    pub(crate) fn explode(&mut self) {
        self.value -= self.max_atoms();
    }
}

#[cfg(feature = "serde")]
impl Serialize for Cell {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("atoms", &self.atoms())?;
        map.serialize_entry("max_atoms", &self.max_atoms())?;
        map.serialize_entry("player", &self.player())?;
        map.end()
    }
}
