use std::collections::VecDeque;

#[cfg(target_family = "wasm")]
use serde::Serialize;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
#[cfg_attr(target_family = "wasm", derive(Serialize))]
pub struct Game {
    board: Vec<Vec<Cell>>,
    players: Vec<Player>,
    turn: usize,
    atoms: usize,
    max_atoms: usize,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(target_family = "wasm", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq, Eq))]
struct Cell {
    atoms: usize,
    player: usize,
    max_atoms: usize,
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(target_family = "wasm", derive(Serialize))]
struct Player {
    atoms: usize,
}

type Coord = (usize, usize);

impl Game {
    pub fn new(height: usize, width: usize, players: usize) -> Self {
        let mut board = vec![vec![Cell::default(); width]; height];
        for (r, row) in board.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                cell.max_atoms = Self::max_atoms((r, c), height, width);
            }
        }
        Self {
            board,
            players: vec![Player::default(); players],
            turn: 0,
            atoms: 0,
            max_atoms: 4
                + 2 * ((height - 2) * 2 + (width - 2) * 2)
                + 3 * ((height - 2) * (width - 2)),
        }
    }

    const fn max_atoms((row, col): Coord, height: usize, width: usize) -> usize {
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
        loop {
            self.turn = (self.turn + 1) % self.players.len();
            // se ci sono meno atomi del numero di giocatori significa che nessuno può essere stato
            // eliminato quindi si può passare al turno successivo
            // se invece il giocatore successivo non è stato eliminato tocca a lui
            if self.atoms <= self.players.len() || self.players[self.turn].atoms > 0 {
                break;
            }
        }
    }

    pub fn add_atom(&mut self, coord @ (row, col): Coord) -> bool {
        let cell = &mut self.board[row][col];
        // se la cella è già occupata
        if cell.atoms != 0 && cell.player != self.turn {
            return false;
        }
        // se la scacchiera è piena si andrebbe in un loop infinito
        if self.atoms == self.max_atoms {
            return false;
        }
        cell.player = self.turn;
        self.players[self.turn].atoms += 1;
        self.atoms += 1;
        if cell.is_unstable() {
            self.explode(coord);
        } else {
            cell.atoms += 1;
        }
        self.next_turn();
        true
    }

    fn explode(&mut self, coord: Coord) {
        let mut queue = VecDeque::from([coord]);
        while let Some((row, col)) = queue.pop_front() {
            let cell = &mut self.board[row][col];
            if cell.atoms != 0 && cell.player != self.turn {
                self.players[cell.player].atoms -= cell.atoms;
                self.players[self.turn].atoms += cell.atoms;
            }
            cell.player = self.turn;
            cell.atoms += 1;
            if cell.must_explode() {
                cell.atoms -= cell.max_atoms;
                if cell.atoms == 0 {
                    cell.player = usize::MAX;
                }
                if row > 0 {
                    queue.push_back((row - 1, col));
                }
                if row < self.board.len() - 1 {
                    queue.push_back((row + 1, col));
                }
                if col > 0 {
                    queue.push_back((row, col - 1));
                }
                if col < self.board[0].len() - 1 {
                    queue.push_back((row, col + 1));
                }
            }
        }
    }
}

impl Cell {
    const fn is_unstable(&self) -> bool {
        self.atoms + 1 == self.max_atoms
    }

    const fn must_explode(&self) -> bool {
        self.atoms >= self.max_atoms
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
