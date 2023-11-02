use crate::{Cell, Game};

#[test]
fn max_atoms() {
    let game = Game::new(3, 3, 4).unwrap();
    assert_eq!(game.max_atoms, 4 + 8 + 3);
    let game = Game::new(4, 4, 4).unwrap();
    assert_eq!(game.max_atoms, 4 + 16 + 12);
}

#[test]
fn max_atoms_cell() {
    fn test(height: usize, width: usize) {
        let game = Game::new(height, width, 4).unwrap();
        // prima controlla manualmente gli angoli
        assert_eq!(game.board[0][0].max_atoms, 2);
        assert_eq!(game.board[0][width - 1].max_atoms, 2);
        assert_eq!(game.board[height - 1][0].max_atoms, 2);
        assert_eq!(game.board[height - 1][width - 1].max_atoms, 2);
        // poi controlla i bordi orizzontali
        for i in 1..width - 1 {
            assert_eq!(game.board[0][i].max_atoms, 3);
            assert_eq!(game.board[height - 1][i].max_atoms, 3);
        }
        // poi controlla i bordi verticali
        for i in 1..height - 1 {
            assert_eq!(game.board[i][0].max_atoms, 3);
            assert_eq!(game.board[i][width - 1].max_atoms, 3);
        }
        // infine controlla il centro
        for i in 1..height - 1 {
            for j in 1..width - 1 {
                assert_eq!(game.board[i][j].max_atoms, 4);
            }
        }
    }
    test(5, 5);
    test(10, 10);
    test(100, 50);
}

#[test]
fn next_turn() {
    let mut game = Game::new(5, 5, 4).unwrap();
    // scorre tutti i giocatori, piazzando manualmente un atomo per ogni giocatore
    // e fa andare avanti il turno
    // l'atomo non viene piazzato davvero ma solo aumentato il contatore globale
    for t in 0..4 {
        assert_eq!(game.turn, t);
        game.atoms += 1;
        game.next_turn();
    }
    assert_eq!(game.turn, 0);
    // aumenta il contatore di atomi di due giocatori ed aggiusta quello globale
    // così che i giocatori 1 e 3 risultino eliminati
    game.players[0].atoms = 3;
    game.players[2].atoms = 3;
    game.atoms = 6;
    game.next_turn();
    assert_eq!(game.turn, 2);
    game.next_turn();
    assert_eq!(game.turn, 0);
}

#[test]
fn add_atom() {
    let mut game = Game::new(5, 5, 2).unwrap();
    game.add_atom((0, 0));
    /*
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    */
    assert_eq!(
        game.board[0][0],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 2,
        }
    );
    assert_eq!(game.players[0].atoms, 1);
    assert_eq!(game.atoms, 1);

    // verifica che non si possa muovere dove è già occupato da un altro giocatore
    assert!(!game.add_atom((0, 0)));

    game.add_atom((4, 4));
    /*
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    */
    assert_eq!(
        game.board[4][4],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 2,
        }
    );
    assert_eq!(game.players[1].atoms, 1);
    assert_eq!(game.atoms, 2);

    // verifica che non si possa muovere dove è già occupato da un altro giocatore
    assert!(!game.add_atom((4, 4)));

    game.add_atom((0, 0));
    /*
    0 1 0 0 0
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    */
    assert_eq!(
        game.board[0][0],
        Cell {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 2,
        }
    );
    assert_eq!(
        game.board[0][1],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[1][0],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(game.players[0].atoms, 2);
    assert_eq!(game.atoms, 3);

    game.add_atom((4, 4));
    /*
    0 1 0 0 0
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    0 0 0 1 0
    */
    assert_eq!(
        game.board[4][4],
        Cell {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 2,
        }
    );
    assert_eq!(
        game.board[4][3],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[3][4],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(game.players[1].atoms, 2);
    assert_eq!(game.atoms, 4);

    game.add_atom((0, 0));
    game.add_atom((4, 4));
    /*
    1 1 0 0 0
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    0 0 0 1 1
    */

    game.add_atom((0, 0));
    /*
    0 2 0 0 0
    2 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    0 0 0 1 1
    */
    assert_eq!(
        game.board[0][0],
        Cell {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 2,
        }
    );
    assert_eq!(
        game.board[0][1],
        Cell {
            atoms: 2,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[1][0],
        Cell {
            atoms: 2,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(game.players[0].atoms, 4);
    assert_eq!(game.atoms, 7);

    game.add_atom((4, 4));
    /*
    0 2 0 0 0
    2 0 0 0 0
    0 0 0 0 0
    0 0 0 0 2
    0 0 0 2 0
    */
    assert_eq!(
        game.board[4][4],
        Cell {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 2,
        }
    );
    assert_eq!(
        game.board[4][3],
        Cell {
            atoms: 2,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[3][4],
        Cell {
            atoms: 2,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(game.players[1].atoms, 4);
    assert_eq!(game.atoms, 8);

    game.add_atom((0, 0));
    game.add_atom((4, 4));
    /*
    1 2 0 0 0
    2 0 0 0 0
    0 0 0 0 0
    0 0 0 0 2
    0 0 0 2 1
    */

    game.add_atom((0, 0));
    /*
    0 1 1 0 0
    1 2 0 0 0
    1 0 0 0 0
    0 0 0 0 2
    0 0 0 2 1
    */
    assert_eq!(
        game.board[0][0],
        Cell {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 2,
        }
    );
    assert_eq!(
        game.board[0][1],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[1][0],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[0][2],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[1][1],
        Cell {
            atoms: 2,
            player: 0,
            max_atoms: 4,
        }
    );
    assert_eq!(
        game.board[2][0],
        Cell {
            atoms: 1,
            player: 0,
            max_atoms: 3,
        }
    );
    assert_eq!(game.players[0].atoms, 6);
    assert_eq!(game.atoms, 11);

    game.add_atom((4, 4));
    /*
    0 1 1 0 0
    1 2 0 0 0
    1 0 0 0 1
    0 0 0 2 1
    0 0 1 1 0
    */
    assert_eq!(
        game.board[4][4],
        Cell {
            atoms: 0,
            player: usize::MAX,
            max_atoms: 2,
        }
    );
    assert_eq!(
        game.board[4][3],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[3][4],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[4][2],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(
        game.board[3][3],
        Cell {
            atoms: 2,
            player: 1,
            max_atoms: 4,
        }
    );
    assert_eq!(
        game.board[2][4],
        Cell {
            atoms: 1,
            player: 1,
            max_atoms: 3,
        }
    );
    assert_eq!(game.players[1].atoms, 6);
    assert_eq!(game.atoms, 12);
}

#[test]
fn elimination() {
    let mut game = Game::new(3, 3, 2).unwrap();
    assert_eq!(game.turn, 0);
    game.add_atom((0, 0));
    assert_eq!(game.turn, 1);
    game.add_atom((0, 1));
    assert_eq!(game.turn, 0);
    game.add_atom((0, 0));
    assert_eq!(game.turn, 0);
}
