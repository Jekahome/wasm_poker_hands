use poker_hands::{Card, Combination, Hand, Menager, Pot, Total, M, N};
use wasm_bindgen_test::*;

// wasm-pack test --node
// wasm-pack test --node -- -- test::it_manager_range

#[wasm_bindgen_test]
fn it_manager_range() {
    let pot: i32 = 2300;
    let mut manager: Menager = Menager::new(pot);
    let flop_1 = Card::new(N::K, M::S);
    let flop_2 = Card::new(N::THREE, M::C);
    let flop_3 = Card::new(N::FOUR, M::S);
    let retn = Card::new(N::FIVE, M::D);
    let river = Card::new(N::SIX, M::H);
    // [1,2,3] [4] [5]
    {
        let hand = Hand::new(
            "1".to_owned(),
            100,
            Card::new(N::SEVEN, M::S),
            Card::new(N::A, M::H),
            flop_1,
            flop_2,
            flop_3,
            retn,
            river,
        );
        manager.add_hand(hand);
    }
    {
        let hand = Hand::new(
            "2".to_owned(),
            400,
            Card::new(N::SEVEN, M::D),
            Card::new(N::A, M::D),
            flop_1,
            flop_2,
            flop_3,
            retn,
            river,
        );
        manager.add_hand(hand);
    }
    {
        let hand = Hand::new(
            "3".to_owned(),
            600,
            Card::new(N::SEVEN, M::C),
            Card::new(N::A, M::C),
            flop_1,
            flop_2,
            flop_3,
            retn,
            river,
        );
        manager.add_hand(hand);
    }
    {
        let hand = Hand::new(
            "4".to_owned(),
            600,
            Card::new(N::A, M::S),
            Card::new(N::Q, M::D),
            flop_1,
            flop_2,
            flop_3,
            retn,
            river,
        );
        manager.add_hand(hand);
    }
    {
        let hand = Hand::new(
            "5".to_owned(),
            600,
            Card::new(N::J, M::C),
            Card::new(N::Q, M::C),
            flop_1,
            flop_2,
            flop_3,
            retn,
            river,
        );
        manager.add_hand(hand);
    }

    let totals: js_sys::Array = manager.calculate_test().unwrap();
    assert!(totals.length() == 3);
    let iterator = totals.values();

    let first: Total = serde_wasm_bindgen::from_value(iterator.next().unwrap().value()).unwrap();
    assert_eq!(first.combination, Combination::Straight);
    assert_eq!(first.key_range_group, 0u8);
    assert_eq!(first.get_player_id(), String::from("1"));
    assert_eq!(first.get_win_pot(), 210_i32);
    let cards: Vec<String> = first
        .show_cards()
        .to_vec()
        .iter()
        .map(|c| c.as_string().unwrap())
        .collect();
    assert_eq!(
        cards[..],
        [
            String::from("7s"),
            String::from("6h"),
            String::from("5d"),
            String::from("4s"),
            String::from("3c")
        ]
    );

    let second: Total = serde_wasm_bindgen::from_value(iterator.next().unwrap().value()).unwrap();
    assert_eq!(second.combination, Combination::Straight);
    assert_eq!(second.key_range_group, 0u8);
    assert_eq!(second.get_player_id(), String::from("2"));
    assert_eq!(second.get_win_pot(), 836_i32);
    let cards: Vec<String> = second
        .show_cards()
        .to_vec()
        .iter()
        .map(|c| c.as_string().unwrap())
        .collect();
    assert_eq!(
        cards[..],
        [
            String::from("7d"),
            String::from("6h"),
            String::from("5d"),
            String::from("4s"),
            String::from("3c")
        ]
    );

    let third: Total = serde_wasm_bindgen::from_value(iterator.next().unwrap().value()).unwrap();
    assert_eq!(third.combination, Combination::Straight);
    assert_eq!(third.key_range_group, 0u8);
    assert_eq!(third.get_player_id(), String::from("3"));
    assert_eq!(third.get_win_pot(), 1254_i32);
    let cards: Vec<String> = third
        .show_cards()
        .to_vec()
        .iter()
        .map(|c| c.as_string().unwrap())
        .collect();
    assert_eq!(
        cards[..],
        [
            String::from("7c"),
            String::from("6h"),
            String::from("5d"),
            String::from("4s"),
            String::from("3c")
        ]
    );

    /*totals.for_each(&mut |value, index, _| {
        let total: Total = serde_wasm_bindgen::from_value(value).unwrap();
        match index {
            0 => {
                assert_eq!(total.combination, Combination::Straight);
            }
            _ => {}
        }
    });*/
}

// wasm-pack test --node -- -- test::it_split_diff_more_pot
#[wasm_bindgen_test]
fn it_split_diff_more_pot() {
    let mut pot: Pot = Pot::new(120);
    pot.add_player("1", 40);
    pot.add_player("2", 10);
    pot.add_player("3", 10);
    pot.add_player("4", 40);
    pot.add_player("5", 20);
    
    let arr = vec![String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("2"),String::from("5")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("1")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 3_usize);
    assert_eq!(res[0], (String::from("3"), 50));
    assert_eq!(res[1], (String::from("5"), 30));
    assert_eq!(res[2], (String::from("1"), 40));
}

// wasm-pack test --node -- -- test::it_split_diff_less_pot
#[wasm_bindgen_test]
fn it_split_diff_less_pot() {
    let mut pot: Pot = Pot::new(100);
    pot.add_player("1", 40);
    pot.add_player("2", 10);
    pot.add_player("3", 10);
    pot.add_player("4", 40);
    
    let arr = vec![String::from("2")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("1")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 2_usize);
    assert_eq!(res[0], (String::from("2"), 40));
    assert_eq!(res[1], (String::from("1"), 60));
    
}

// wasm-pack test --node -- -- test::it_split_pot
#[wasm_bindgen_test]
fn it_split_pot() {
    let mut pot: Pot = Pot::new(2300);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("1"), String::from("2"), String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 3_usize);
    assert_eq!(res[0], (String::from("1"), 210));
    assert_eq!(res[1], (String::from("2"), 836));
    assert_eq!(res[2], (String::from("3"), 1254));
}
// wasm-pack test --node -- -- test::it_pot_construct_fail
#[wasm_bindgen_test]
fn it_pot_construct_fail() {
    let mut pot: Pot = Pot::new(2200);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("1"), String::from("2"), String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    assert_eq!(None, pot.calculate());

    let mut pot: Pot = Pot::new(2300);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("1"), String::from("2"), String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    assert_eq!(None, pot.calculate());

    let mut pot: Pot = Pot::new(2300);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 700);
    let arr = vec![String::from("1"), String::from("2"), String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    assert_eq!(None, pot.calculate());
}
// wasm-pack test --node -- -- test::it_extra_pot
#[wasm_bindgen_test]
fn it_extra_pot() {
    let mut pot: Pot = Pot::new(2700);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("1"), String::from("2"), String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 3_usize);
    assert_eq!(res[0], (String::from("1"), 343));
    assert_eq!(res[1], (String::from("2"), 969));
    assert_eq!(res[2], (String::from("3"), 1387));
}
// wasm-pack test --node -- -- test::it_queue_pot
#[wasm_bindgen_test]
fn it_queue_pot() {
    let mut pot: Pot = Pot::new(2300);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("1")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("2")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 3_usize);
    assert_eq!(res[0], (String::from("1"), 500));
    assert_eq!(res[1], (String::from("2"), 1200));
    assert_eq!(res[2], (String::from("3"), 600));

    let mut pot: Pot = Pot::new(2300);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("2")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("1")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 2_usize);
    assert_eq!(res[0], (String::from("2"), 1700));
    assert_eq!(res[1], (String::from("3"), 600));

    let mut pot: Pot = Pot::new(2300);
    pot.add_player("1", 100);
    pot.add_player("2", 400);
    pot.add_player("3", 600);
    pot.add_player("4", 600);
    pot.add_player("5", 600);
    let arr = vec![String::from("3")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("2")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("1")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("4")];
    pot.add_next_group_win_vec(arr);
    let arr = vec![String::from("5")];
    pot.add_next_group_win_vec(arr);
    let res: Vec<(String, i32)> = pot.calculate().unwrap();
    assert_eq!(res.len(), 1_usize);
    assert_eq!(res[0], (String::from("3"), 2300));
}
