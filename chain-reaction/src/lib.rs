use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use cell::Cell;
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::Serialize;

mod cell;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Game {
    board: Vec<Cell>,
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    height: usize,
    width: usize,
    players: Vec<Player>,
    num_players: u16,
    turn: usize,
    atoms: u16,
    won: bool,
    history: Vec<History>,
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct Player {
    atoms: u16,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Error {
    Occupied,
    GameWon,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Explosion {
    result: Vec<Cell>,
    exploded: HashSet<Coord>,
}

#[derive(Clone, Debug)]
struct History {
    board: Vec<Cell>,
    players: Vec<Player>,
    turn: usize,
    atoms: u16,
}

type Coord = (usize, usize);

impl Game {
    pub fn new(height: usize, width: usize, players: usize) -> Option<Self> {
        if !(3..=18).contains(&height) || !(3..=10).contains(&width) || !(2..=8).contains(&players)
        {
            return None;
        }
        Some(Self {
            board: (0..height)
                .flat_map(|row| (0..width).map(move |col| Cell::new((row, col), height, width)))
                .collect(),
            height,
            width,
            players: vec![Player::default(); players],
            num_players: players as u16,
            turn: 0,
            atoms: 0,
            won: false,
            history: vec![],
        })
    }

    pub fn small(players: usize) -> Self {
        Self::new(11, 6, players).unwrap()
    }

    pub fn large(players: usize) -> Self {
        Self::new(18, 10, players).unwrap()
    }

    fn next_turn(&mut self) {
        let old_turn = self.turn;
        loop {
            self.turn = (self.turn + 1) % self.players.len();
            // se ci sono meno atomi del numero di giocatori significa che nessuno può essere stato
            // eliminato quindi si può passare al turno successivo
            // se invece il giocatore successivo non è stato eliminato tocca a lui
            if self.atoms <= self.num_players || self.players[self.turn].atoms > 0 {
                break;
            }
        }
        if old_turn == self.turn {
            self.won = true;
        }
    }

    pub fn add_atom(&mut self, (row, col): Coord) -> Result<Vec<Explosion>, Error> {
        if self.won {
            return Err(Error::GameWon);
        }
        let index = row * self.width + col;
        let cell = self.board[index];
        // se la cella è già occupata
        if cell.atoms() != 0 && cell.player() != self.turn {
            return Err(Error::Occupied);
        }
        self.history.push(History {
            board: self.board.clone(),
            players: self.players.clone(),
            turn: self.turn,
            atoms: self.atoms,
        });
        let cell = &mut self.board[index];
        cell.set_player(self.turn);
        cell.add_atom();
        self.atoms += 1;
        self.players[self.turn].atoms += 1;
        let result = if cell.must_explode() {
            self.explode(index)
        } else {
            vec![]
        };
        self.next_turn();
        Ok(result)
    }

    fn explode(&mut self, index: usize) -> Vec<Explosion> {
        let mut result = vec![];
        if !self.board[index].must_explode() {
            return result;
        }
        let mut exploded = vec![false; self.board.len()];
        let mut exploded_count_down = self.board.len();
        let mut to_explode = VecDeque::from([index]);
        while !to_explode.is_empty() && exploded_count_down > 0 {
            let mut round = HashSet::new();
            for _ in 0..to_explode.len() {
                let index = to_explode.pop_front().unwrap();
                let cell = &mut self.board[index];
                // se la cella ha ricevuto più di un'esplosione nello stesso round
                if !cell.must_explode() {
                    continue;
                }
                round.insert((index / self.width, index % self.width));
                if !exploded[index] {
                    exploded[index] = true;
                    exploded_count_down -= 1;
                }
                cell.explode();
                for next in [
                    index.wrapping_sub(self.width),
                    index + self.width,
                    if index % self.width == 0 {
                        usize::MAX
                    } else {
                        index - 1
                    },
                    if index % self.width == self.width - 1 {
                        usize::MAX
                    } else {
                        index + 1
                    },
                ] {
                    if next < self.board.len() {
                        let next_cell = &mut self.board[next];
                        if next_cell.atoms() != 0 && next_cell.player() != self.turn {
                            self.players[next_cell.player()].atoms -= next_cell.atoms() as u16;
                            self.players[self.turn].atoms += next_cell.atoms() as u16;
                        }
                        next_cell.set_player(self.turn);
                        next_cell.add_atom();
                        if next_cell.must_explode() {
                            to_explode.push_back(next);
                        }
                    }
                }
            }
            result.push(Explosion::new(self.board.clone(), round));
        }
        if exploded_count_down == 0 {
            self.won = true;
        }
        result
    }

    pub fn undo(&mut self) -> bool {
        if let Some(history) = self.history.pop() {
            self.board = history.board;
            self.players = history.players;
            self.turn = history.turn;
            self.atoms = history.atoms;
            self.won = false;
            true
        } else {
            false
        }
    }

    pub fn get(&self, (row, col): Coord) -> Cell {
        self.board[row * self.width + col]
    }
}

impl Explosion {
    fn new(result: Vec<Cell>, exploded: HashSet<Coord>) -> Self {
        Self { result, exploded }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::small(2)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (i, cell) in self.board.iter().enumerate() {
            result.push_str(&format!("{} ", cell.atoms()));
            if i % self.width == self.width - 1 {
                result.pop();
                result.push('\n');
            }
        }
        result.pop();
        write!(f, "{}", result)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Game {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut game = serializer.serialize_map(Some(7))?;
        game.serialize_entry("height", &self.height)?;
        game.serialize_entry("width", &self.width)?;
        game.serialize_entry("players", &self.players)?;
        game.serialize_entry("turn", &self.turn)?;
        game.serialize_entry("atoms", &self.atoms)?;
        game.serialize_entry("won", &self.won)?;
        game.serialize_entry(
            "board",
            &(0..self.height)
                .map(|row| {
                    (0..self.width)
                        .map(|col| {
                            serde_json::to_value(self.board[row * self.width + col]).unwrap()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )?;
        game.end()
    }
}
