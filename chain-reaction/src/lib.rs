use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Game {
    board: Vec<Vec<Cell>>,
    players: Vec<Player>,
    num_players: u16,
    turn: usize,
    // il massimo numero è dato dalla formula 4 + 2 * ((H - 2) * 2 + (W - 2) * 2) + 3 * ((H - 2) * (W - 2))
    // dove H e W sono rispettivamente l'altezza e la larghezza massime della scacchiera, ovvero 18 e 10
    // il risultato è 484, per il quale servono 9 bit
    max_atoms: u16,
    // potrebbe essere u8 ma per evitare conversioni inutili va bene u16
    atoms: u16,
    won: bool,
    #[serde(skip)]
    history: Vec<History>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Cell {
    atoms: u8,
    player: usize,
    max_atoms: u8,
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct Player {
    atoms: u16,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Error {
    Occupied,
    GameWon,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Explosion {
    result: Vec<Vec<Cell>>,
    exploded: HashSet<Coord>,
}

#[derive(Clone, Debug)]
struct History {
    board: Vec<Vec<Cell>>,
    players: Vec<Player>,
    // num_players: u16, // costante
    turn: usize,
    // max_atoms: u16, // costante
    atoms: u16,
    // won: bool, // impossibile che uno stato precedente sia già vinto
}

type Coord = (usize, usize);

impl Game {
    pub fn new(height: usize, width: usize, players: usize) -> Option<Self> {
        if !(3..=18).contains(&height) || !(3..=10).contains(&width) || !(2..=8).contains(&players)
        {
            return None;
        }
        let mut board = vec![vec![Cell::default(); width]; height];
        for (r, row) in board.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                cell.max_atoms = Self::max_atoms((r, c), height, width);
            }
        }
        let (height, width) = (height as u16, width as u16);
        Some(Self {
            board,
            players: vec![Player::default(); players],
            num_players: players as u16,
            turn: 0,
            atoms: 0,
            max_atoms: 4
                + 2 * ((height - 2) * 2 + (width - 2) * 2)
                + 3 * ((height - 2) * (width - 2)),
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

    const fn max_atoms((row, col): Coord, height: usize, width: usize) -> u8 {
        let is_horizontal_edge = row == 0 || row == height - 1;
        let is_vertical_edge = col == 0 || col == width - 1;
        if is_horizontal_edge && is_vertical_edge {
            // angolo
            2
        } else if is_horizontal_edge || is_vertical_edge {
            // bordo
            3
        } else {
            // centro
            4
        }
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

    pub fn add_atom(&mut self, coord @ (row, col): Coord) -> Result<Vec<Explosion>, Error> {
        if self.won {
            return Err(Error::GameWon);
        }
        let cell = self.board[row][col];
        // se la cella è già occupata
        if cell.atoms != 0 && cell.player != self.turn {
            return Err(Error::Occupied);
        }
        self.history.push(History {
            board: self.board.clone(),
            players: self.players.clone(),
            turn: self.turn,
            atoms: self.atoms,
        });
        let cell = &mut self.board[row][col];
        cell.player = self.turn;
        cell.atoms += 1;
        self.atoms += 1;
        self.players[self.turn].atoms += 1;
        let result = if cell.must_explode() {
            self.explode(coord)
        } else {
            vec![]
        };
        self.next_turn();
        Ok(result)
    }

    fn explode(&mut self, coord @ (row, col): Coord) -> Vec<Explosion> {
        let mut result = vec![];
        if !self.board[row][col].must_explode() {
            return result;
        }
        let mut exploded = vec![vec![false; self.board[0].len()]; self.board.len()];
        let mut exploded_count_down = self.board.len() * self.board[0].len();
        let mut to_explode = VecDeque::from([coord]);
        while !to_explode.is_empty() && exploded_count_down > 0 {
            let mut round = HashSet::new();
            for _ in 0..to_explode.len() {
                let coord @ (row, col) = to_explode.pop_front().unwrap();
                let cell = &mut self.board[row][col];
                // se la cella ha subito più di un'esplosione nello stesso round
                if !cell.must_explode() {
                    continue;
                }
                round.insert(coord);
                if !exploded[row][col] {
                    exploded[row][col] = true;
                    exploded_count_down -= 1;
                }
                cell.atoms -= cell.max_atoms;
                if cell.atoms == 0 {
                    cell.player = usize::MAX;
                }
                for next @ (next_row, next_col) in [
                    (row.wrapping_sub(1), col),
                    (row + 1, col),
                    (row, col.wrapping_sub(1)),
                    (row, col + 1),
                ] {
                    if next_row < self.board.len() && next_col < self.board[0].len() {
                        let next_cell = &mut self.board[next_row][next_col];
                        if next_cell.atoms != 0 && next_cell.player != self.turn {
                            self.players[next_cell.player].atoms -= next_cell.atoms as u16;
                            self.players[self.turn].atoms += next_cell.atoms as u16;
                        }
                        next_cell.player = self.turn;
                        next_cell.atoms += 1;
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
}

impl Cell {
    const fn must_explode(&self) -> bool {
        self.atoms >= self.max_atoms
    }
}

impl Explosion {
    fn new(result: Vec<Vec<Cell>>, exploded: HashSet<Coord>) -> Self {
        Self { result, exploded }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::small(2)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 0,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in &self.board {
            for cell in row {
                result.push_str(&format!("{} ", cell.atoms));
            }
            result.pop();
            result.push('\n');
        }
        result.pop();
        write!(f, "{}", result)
    }
}
