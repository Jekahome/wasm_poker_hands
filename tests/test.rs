use poker_hands::{Pot, Win};
use wasm_bindgen_test::*;

// wasm-pack test --node
// wasm-pack test --node -- -- test::it_split_pot

#[wasm_bindgen_test]
fn it_split_pot() {
    let mut pot: Pot = Pot::new(2300);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(3_u32);
    arr.set_index(0u32, 1_i32);
    arr.set_index(1u32, 2_i32);
    arr.set_index(2u32, 3_i32);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    let res_js = pot.calculate().unwrap();
    let res_jsvalue: Vec<wasm_bindgen::JsValue> = res_js.to_vec();
    let res = res_jsvalue
        .into_iter()
        .map(|js| {
            let win: Win = js.into_serde().unwrap();
            win
        })
        .collect::<Vec<Win>>();

    assert_eq!(res[0].id, 1);
    assert_eq!(res[0].pot, 210);
    assert_eq!(res[1].id, 2);
    assert_eq!(res[1].pot, 836);
    assert_eq!(res[2].id, 3);
    assert_eq!(res[2].pot, 1254);
}

#[wasm_bindgen_test]
fn it_pot_construct_fail() {
    let mut pot: Pot = Pot::new(2200);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(3_u32);
    arr.set_index(0u32, 1_i32);
    arr.set_index(1u32, 2_i32);
    arr.set_index(2u32, 3_i32);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    assert_eq!(None, pot.calculate());

    let mut pot: Pot = Pot::new(2300);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(3_u32);
    arr.set_index(0u32, 1_i32);
    arr.set_index(1u32, 2_i32);
    arr.set_index(2u32, 3_i32);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    assert_eq!(None, pot.calculate());

    let mut pot: Pot = Pot::new(2300);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 700);
    let arr = js_sys::Int32Array::new_with_length(3_u32);
    arr.set_index(0u32, 1_i32);
    arr.set_index(1u32, 2_i32);
    arr.set_index(2u32, 3_i32);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    assert_eq!(None, pot.calculate());
}

#[wasm_bindgen_test]
fn it_big_pot() {
    let mut pot: Pot = Pot::new(2700);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(3_u32);
    arr.set_index(0u32, 1_i32);
    arr.set_index(1u32, 2_i32);
    arr.set_index(2u32, 3_i32);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    let res_js = pot.calculate().unwrap();
    let res_jsvalue: Vec<wasm_bindgen::JsValue> = res_js.to_vec();
    let res = res_jsvalue
        .into_iter()
        .map(|js| {
            let win: Win = js.into_serde().unwrap();
            win
        })
        .collect::<Vec<Win>>();

    assert_eq!(res[0].id, 1);
    assert_eq!(res[0].pot, 343);
    assert_eq!(res[1].id, 2);
    assert_eq!(res[1].pot, 969);
    assert_eq!(res[2].id, 3);
    assert_eq!(res[2].pot, 1387);
}

#[wasm_bindgen_test]
fn it_queue_pot() {
    let mut pot: Pot = Pot::new(2300);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 1);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 2);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 3);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    let res_js = pot.calculate().unwrap();
    let res_jsvalue: Vec<wasm_bindgen::JsValue> = res_js.to_vec();
    let res = res_jsvalue
        .into_iter()
        .map(|js| {
            let win: Win = js.into_serde().unwrap();
            win
        })
        .collect::<Vec<Win>>();
    assert_eq!(res[0].id, 1);
    assert_eq!(res[0].pot, 500);
    assert_eq!(res[1].id, 2);
    assert_eq!(res[1].pot, 1200);
    assert_eq!(res[2].id, 3);
    assert_eq!(res[2].pot, 600);

    let mut pot: Pot = Pot::new(2300);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 2);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 3);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 1);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    let res_js = pot.calculate().unwrap();
    let res_jsvalue: Vec<wasm_bindgen::JsValue> = res_js.to_vec();
    let res = res_jsvalue
        .into_iter()
        .map(|js| {
            let win: Win = js.into_serde().unwrap();
            win
        })
        .collect::<Vec<Win>>();
    assert_eq!(res[0].id, 2);
    assert_eq!(res[0].pot, 1700);
    assert_eq!(res[1].id, 3);
    assert_eq!(res[1].pot, 600);

    let mut pot: Pot = Pot::new(2300);
    pot.add_player(1, 100);
    pot.add_player(2, 400);
    pot.add_player(3, 600);
    pot.add_player(4, 600);
    pot.add_player(5, 600);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 3);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 2);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 1);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 4);
    pot.add_next_group_win(arr);
    let arr = js_sys::Int32Array::new_with_length(1u32);
    arr.set_index(0u32, 5);
    pot.add_next_group_win(arr);
    let res_js = pot.calculate().unwrap();
    let res_jsvalue: Vec<wasm_bindgen::JsValue> = res_js.to_vec();
    let res = res_jsvalue
        .into_iter()
        .map(|js| {
            let win: Win = js.into_serde().unwrap();
            win
        })
        .collect::<Vec<Win>>();
    assert_eq!(res[0].id, 3);
    assert_eq!(res[0].pot, 2300);
}
