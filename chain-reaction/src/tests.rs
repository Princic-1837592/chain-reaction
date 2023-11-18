use std::collections::HashSet;

use crate::{Cell, Game};

#[test]
fn max_atoms_cell() {
    fn test(height: usize, width: usize) {
        let game = Game::new(height, width, 4).unwrap();
        // prima controlla manualmente gli angoli
        assert_eq!(game.get((0, 0)).max_atoms(), 2);
        assert_eq!(game.get((0, width - 1)).max_atoms(), 2);
        assert_eq!(game.get((height - 1, 0)).max_atoms(), 2);
        assert_eq!(game.get((height - 1, width - 1)).max_atoms(), 2);
        // poi controlla i bordi orizzontali
        for i in 1..width - 1 {
            assert_eq!(game.get((0, i)).max_atoms(), 3);
            assert_eq!(game.get((height - 1, i)).max_atoms(), 3);
        }
        // poi controlla i bordi verticali
        for i in 1..height - 1 {
            assert_eq!(game.get((i, 0)).max_atoms(), 3);
            assert_eq!(game.get((i, width - 1)).max_atoms(), 3);
        }
        // infine controlla il centro
        for i in 1..height - 1 {
            for j in 1..width - 1 {
                assert_eq!(game.get((i, j)).max_atoms(), 4);
            }
        }
    }
    test(5, 5);
    test(10, 10);
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

    assert!(game.add_atom((0, 0)).unwrap().is_empty());
    /*
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    */
    assert_eq!(game.get((0, 0)), Cell::from(1, 0, 2));
    assert_eq!(game.players[0].atoms, 1);
    assert_eq!(game.atoms, 1);

    // verifica che non si possa muovere dove è già occupato da un altro giocatore
    assert!(game.add_atom((0, 0)).is_err());

    assert!(game.add_atom((4, 4)).unwrap().is_empty());
    /*
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    */
    assert_eq!(game.get((4, 4)), Cell::from(1, 1, 2));
    assert_eq!(game.players[1].atoms, 1);
    assert_eq!(game.atoms, 2);

    // verifica che non si possa muovere dove è già occupato da un altro giocatore
    assert!(game.add_atom((4, 4)).is_err());

    assert_eq!(
        game.add_atom((0, 0)).unwrap()[0].exploded,
        HashSet::from([(0, 0)])
    );
    /*
    0 1 0 0 0
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    */
    assert_eq!(game.get((0, 0)), Cell::from(0, 0, 2));
    assert_eq!(game.get((0, 1)), Cell::from(1, 0, 3));
    assert_eq!(game.get((1, 0)), Cell::from(1, 0, 3));
    assert_eq!(game.players[0].atoms, 2);
    assert_eq!(game.atoms, 3);

    assert_eq!(
        game.add_atom((4, 4)).unwrap()[0].exploded,
        HashSet::from([(4, 4)])
    );
    /*
    0 1 0 0 0
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    0 0 0 1 0
    */
    assert_eq!(game.get((4, 4)), Cell::from(0, 1, 2));
    assert_eq!(game.get((4, 3)), Cell::from(1, 1, 3));
    assert_eq!(game.get((3, 4)), Cell::from(1, 1, 3));
    assert_eq!(game.players[1].atoms, 2);
    assert_eq!(game.atoms, 4);

    assert!(game.add_atom((0, 0)).unwrap().is_empty());
    assert!(game.add_atom((4, 4)).unwrap().is_empty());
    /*
    1 1 0 0 0
    1 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    0 0 0 1 1
    */

    assert_eq!(
        game.add_atom((0, 0)).unwrap()[0].exploded,
        HashSet::from([(0, 0)])
    );
    /*
    0 2 0 0 0
    2 0 0 0 0
    0 0 0 0 0
    0 0 0 0 1
    0 0 0 1 1
    */
    assert_eq!(game.get((0, 0)), Cell::from(0, 0, 2));
    assert_eq!(game.get((0, 1)), Cell::from(2, 0, 3));
    assert_eq!(game.get((1, 0)), Cell::from(2, 0, 3));
    assert_eq!(game.players[0].atoms, 4);
    assert_eq!(game.atoms, 7);

    assert_eq!(
        game.add_atom((4, 4)).unwrap()[0].exploded,
        HashSet::from([(4, 4)])
    );
    /*
    0 2 0 0 0
    2 0 0 0 0
    0 0 0 0 0
    0 0 0 0 2
    0 0 0 2 0
    */
    assert_eq!(game.get((4, 4)), Cell::from(0, 1, 2));
    assert_eq!(game.get((4, 3)), Cell::from(2, 1, 3));
    assert_eq!(game.get((3, 4)), Cell::from(2, 1, 3));
    assert_eq!(game.players[1].atoms, 4);
    assert_eq!(game.atoms, 8);

    assert!(game.add_atom((0, 0)).unwrap().is_empty());
    assert!(game.add_atom((4, 4)).unwrap().is_empty());
    /*
    1 2 0 0 0
    2 0 0 0 0
    0 0 0 0 0
    0 0 0 0 2
    0 0 0 2 1
    */

    assert_eq!(
        game.add_atom((0, 0))
            .unwrap()
            .into_iter()
            .map(|e| e.exploded)
            .collect::<Vec<HashSet<_>>>(),
        vec![
            HashSet::from([(0, 0)]),
            HashSet::from([(0, 1), (1, 0)]),
            HashSet::from([(0, 0)])
        ]
    );
    /*
    0 1 1 0 0
    1 2 0 0 0
    1 0 0 0 0
    0 0 0 0 2
    0 0 0 2 1
    */
    assert_eq!(game.get((0, 0)), Cell::from(0, 0, 2));
    assert_eq!(game.get((0, 1)), Cell::from(1, 0, 3));
    assert_eq!(game.get((1, 0)), Cell::from(1, 0, 3));
    assert_eq!(game.get((0, 2)), Cell::from(1, 0, 3));
    assert_eq!(game.get((1, 1)), Cell::from(2, 0, 4));
    assert_eq!(game.get((2, 0)), Cell::from(1, 0, 3));
    assert_eq!(game.players[0].atoms, 6);
    assert_eq!(game.atoms, 11);

    assert_eq!(
        game.add_atom((4, 4))
            .unwrap()
            .into_iter()
            .map(|e| e.exploded)
            .collect::<Vec<HashSet<_>>>(),
        vec![
            HashSet::from([(4, 4)]),
            HashSet::from([(4, 3), (3, 4)]),
            HashSet::from([(4, 4)])
        ]
    );
    /*
    0 1 1 0 0
    1 2 0 0 0
    1 0 0 0 1
    0 0 0 2 1
    0 0 1 1 0
    */
    assert_eq!(game.get((4, 4)), Cell::from(0, 1, 2));
    assert_eq!(game.get((4, 3)), Cell::from(1, 1, 3));
    assert_eq!(game.get((3, 4)), Cell::from(1, 1, 3));
    assert_eq!(game.get((4, 2)), Cell::from(1, 1, 3));
    assert_eq!(game.get((3, 3)), Cell::from(2, 1, 4));
    assert_eq!(game.get((2, 4)), Cell::from(1, 1, 3));
    assert_eq!(game.players[1].atoms, 6);
    assert_eq!(game.atoms, 12);

    let mut game = Game::small(2);
    for coord in [
        (0, 5),
        (0, 0),
        (10, 5),
        (10, 0),
        (10, 4),
        (10, 1),
        (10, 4),
        (10, 1),
        (10, 3),
        (10, 2),
        (10, 3),
        (10, 2),
        (9, 5),
        (9, 0),
        (9, 4),
        (9, 1),
        (9, 4),
        (9, 1),
        (9, 4),
        (9, 1),
    ] {
        assert!(game.add_atom(coord).unwrap().is_empty());
    }
    assert_eq!(
        game.to_string(),
        r"
1 0 0 0 0 1
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
1 3 0 0 3 1
1 2 2 2 2 1
"
        .trim()
    );
    assert_eq!(
        game.add_atom((10, 5))
            .unwrap()
            .into_iter()
            .map(|e| e.exploded)
            .collect::<Vec<HashSet<_>>>(),
        vec![
            HashSet::from([(10, 5)]),
            HashSet::from([(10, 4)]),
            HashSet::from([(9, 4), (10, 3)]),
            HashSet::from([(10, 2), (9, 5)]),
            HashSet::from([(10, 1), (10, 5)]),
            HashSet::from([(9, 1), (10, 0), (10, 4)]),
            HashSet::from([(9, 0)]),
        ]
    );
    assert_eq!(
        game.to_string(),
        r"
1 0 0 0 0 1
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
1 1 0 0 1 1
0 1 2 2 2 1
1 2 1 2 0 1
"
        .trim()
    );
}

#[test]
fn elimination() {
    let mut game = Game::new(3, 3, 2).unwrap();
    assert_eq!(game.turn, 0);
    assert!(game.add_atom((0, 0)).is_ok());
    assert_eq!(game.turn, 1);
    assert!(game.add_atom((0, 1)).is_ok());
    assert_eq!(game.turn, 0);
    assert!(game.add_atom((0, 0)).is_ok());
    assert_eq!(game.turn, 0);
    assert!(game.add_atom((0, 0)).is_err());
    assert_eq!(game.turn, 0);
}
