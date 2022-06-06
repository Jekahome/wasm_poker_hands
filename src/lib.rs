 
#![allow(unused_imports)]
#![allow(unused_variables)]

use js_sys::{Promise, Uint8ClampedArray, WebAssembly};
 
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
 
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::fmt;

use std::collections::HashMap;
 
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}
 
#[wasm_bindgen]
#[derive(Eq, Hash, Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Combination {
    RoyalFlush = 9,
    StraightFlush = 8,
    FourOfKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfKind = 3,
    TwoPairs = 2,
    Pair = 1,
    HighCard = 0,
}
impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Combination::RoyalFlush => {
                write!(f, "RoyalFlush")
            }
            Combination::StraightFlush => {
                write!(f, "StraightFlush")
            }
            Combination::FourOfKind => {
                write!(f, "FourOfKind")
            }
            Combination::FullHouse => {
                write!(f, "FullHouse")
            }
            Combination::Flush => {
                write!(f, "Flush")
            }
            Combination::Straight => {
                write!(f, "Straight")
            }
            Combination::ThreeOfKind => {
                write!(f, "ThreeOfKind")
            }
            Combination::TwoPairs => {
                write!(f, "TwoPairs")
            }
            Combination::Pair => {
                write!(f, "Pair")
            }
            Combination::HighCard => {
                write!(f, "HighCard")
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FullCombination {
    pub combination: Combination,
    pub key_range:u8,
    cards: Vec<Card>,
}
impl PartialEq<FullCombination> for FullCombination {
    fn eq(&self, other: &Self) -> bool {
        if self.combination == other.combination {
            return self.cards[0].n == other.cards[0].n
                && self.cards[1].n == other.cards[1].n
                && self.cards[2].n == other.cards[2].n
                && self.cards[3].n == other.cards[3].n
                && self.cards[4].n == other.cards[4].n;
        }
        false
    }
}
impl PartialOrd for FullCombination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.combination.partial_cmp(&other.combination).unwrap() {
            Ordering::Equal => self.cards.partial_cmp(&other.cards),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
        }
    }
}

impl fmt::Display for FullCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {},{},{},{},{}",
            self.combination,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4]
        )
    }
}

#[wasm_bindgen]
impl FullCombination {
    #[wasm_bindgen(js_name = get_cards)]
    pub fn getCards(&self) -> js_sys::Array{
        let ret = js_sys::Array::new_with_length(self.cards.len() as u32);
        for (index,c) in self.cards.iter().enumerate(){
             ret.set(index as u32, wasm_bindgen::JsValue::from(c.clone()));
        }
       ret
    }
}


impl FullCombination {
    fn new(combination: Combination, cards: Vec<Card>) -> Self {
        Self { combination, key_range:0,cards }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum M {
    S = 1,    /*Spades*/
    H = 10,   /*Hearts*/
    D = 100,  /*Diamonds*/
    C = 1000, /*Clubs*/
}
impl fmt::Display for M {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            M::S => {
                write!(f, "s")
            }
            M::H => {
                write!(f, "h")
            }
            M::D => {
                write!(f, "d")
            }
            M::C => {
                write!(f, "c")
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum N {
    TWO = 1,
    THREE = 2,
    FOUR = 4,
    FIVE = 8,
    SIX = 16,
    SEVEN = 32,
    EIGHT = 64,
    NINE = 128,
    TEN = 256,
    J = 512,
    Q = 1024,
    K = 2048,
    A = 4096,
}
impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            N::TWO => {
                write!(f, "2")
            }
            N::THREE => {
                write!(f, "3")
            }
            N::FOUR => {
                write!(f, "4")
            }
            N::FIVE => {
                write!(f, "5")
            }
            N::SIX => {
                write!(f, "6")
            }
            N::SEVEN => {
                write!(f, "7")
            }
            N::EIGHT => {
                write!(f, "8")
            }
            N::NINE => {
                write!(f, "9")
            }
            N::TEN => {
                write!(f, "10")
            }
            N::J => {
                write!(f, "J")
            }
            N::Q => {
                write!(f, "Q")
            }
            N::K => {
                write!(f, "K")
            }
            N::A => {
                write!(f, "A")
            }
        }
    }
}

use std::convert::TryFrom;
impl TryFrom<i32> for N {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(N::TWO),
            2 => Ok(N::THREE),
            4 => Ok(N::FOUR),
            8 => Ok(N::FIVE),
            16 => Ok(N::SIX),
            32 => Ok(N::SEVEN),
            64 => Ok(N::EIGHT),
            128 => Ok(N::NINE),
            256 => Ok(N::TEN),
            512 => Ok(N::J),
            1024 => Ok(N::Q),
            2048 => Ok(N::K),
            4096 => Ok(N::A),
            _ => Err("N only accepts value superior than zero!"),
        }
    }
}
use serde::{Deserialize,Deserializer, Serialize};
#[wasm_bindgen]  
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Card {
    pub n: N,
    pub m: M,
}
impl PartialEq<Card> for Card {
    fn eq(&self, other: &Card) -> bool {
        self.n == other.n && self.m == other.m
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.n.partial_cmp(&other.n)
    }
}
 

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(n: N, m: M) -> Self {
        Self { n, m }
    }
    pub fn get(self)->wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from(self)   
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.n, self.m)
    }
}

#[wasm_bindgen(skip)]
#[derive(Debug)]
pub struct Hand {
    key: &'static str,
    cards: Vec<Card>,
    prepare_n: Option<Vec<(i32, i32)>>,
    prepare_flash: Option<Vec<Card>>,
    count_n: Option<i32>,
    count_m: Option<i32>,
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hand{{ key:{}, cards:{:?} }}", self.key, self.cards)
    }
}

#[wasm_bindgen]
impl Hand {
    #[wasm_bindgen(constructor)]
    pub fn new(
        key: &str,
        c1: Card,
        c2: Card,
        c3: Card,
        c4: Card,
        c5: Card,
        c6: Card,
        c7: Card,
    ) -> Self {
        let mut h = Hand {
            key: Box::leak(String::from(key).into_boxed_str()),
            cards: vec![c1, c2, c3, c4, c5, c6, c7],
            prepare_n: None,
            prepare_flash: None,
            count_n: None,
            count_m: None,
        };
        h.cards
            .sort_by(|a, b| (b.n as i32).partial_cmp(&(a.n as i32)).unwrap());
        h.cards.dedup();
        if h.cards.len() < 7 {
            panic!("Cards not correct");
        }
        h
    }
}

impl Hand {

    pub fn find(&mut self) -> Option<FullCombination> {
        self.find_royal_and_staight_flash()
            .or(self.find_four_of_kind())
            .or(self.find_full_house())
            .or(self.find_flash())
            .or(self.find_straight())
            .or(self.find_three_of_kind())
            .or(self.find_two_pair())
            .or(self.find_pair())
            .or_else(|| self.find_high_cards())
    }

    fn count_n(&self) -> i32 {
        let mut res = 0b0000000000000;
        match self.cards[0].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }

        match self.cards[1].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }
        match self.cards[2].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }
        match self.cards[3].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }
        match self.cards[4].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }
        match self.cards[5].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }
        match self.cards[6].n as i32 {
            1 => {
                res = res | 0b0000000000001;
            }
            2 => {
                res = res | 0b0000000000010;
            }
            4 => {
                res = res | 0b0000000000100;
            }
            8 => {
                res = res | 0b0000000001000;
            }
            16 => {
                res = res | 0b0000000010000;
            }
            32 => {
                res = res | 0b0000000100000;
            }
            64 => {
                res = res | 0b0000001000000;
            }
            128 => {
                res = res | 0b0000010000000;
            }
            256 => {
                res = res | 0b0000100000000;
            }
            512 => {
                res = res | 0b0001000000000;
            }
            1024 => {
                res = res | 0b0010000000000;
            }
            2048 => {
                res = res | 0b0100000000000;
            }
            4096 => {
                res = res | 0b1000000000000;
            }
            _ => {}
        }
        res
    }

    fn count_m(&self) -> i32 {
        let mut res: i32 = 0;
        res += self.cards[0].m as i32;
        res += self.cards[1].m as i32;
        res += self.cards[2].m as i32;
        res += self.cards[3].m as i32;
        res += self.cards[4].m as i32;
        res += self.cards[5].m as i32;
        res += self.cards[6].m as i32;
        res
    }

    fn check(&self, n: &[N; 5], m: M) -> bool {
        let mut count = 0;
        for i in n.iter() {
            for c in self.cards.iter() {
                if &c.n == i && c.m == m {
                    count += 1;
                }
            }
        }
        count == 5
    }

    fn find_royal_and_staight_flash(&mut self) -> Option<FullCombination> {
        if let Some(flash) = self.find_flash() {
            let hand = self.count_n();
            self.count_n = Some(hand);
            //  {:013b} (7 cards) QJ109823 = 1024+512+256+128+64+2+1 = 1987 = 0b0011111000011

            let m = flash.cards[0].m;
            for total in [
                0b1111100000000,
                0b0111110000000,
                0b0011111000000,
                0b0001111100000,
                0b0000111110000,
                0b0000011111000,
                0b0000001111100,
                0b0000000111110,
                0b0000000011111,
                0b1000000001111,
            ]
            .iter()
            {
                if &(hand & total) == total {
                    match total {
                        7936 if self.check(&[N::A, N::K, N::Q, N::J, N::TEN], m) => {
                            return Some(FullCombination::new(
                                Combination::RoyalFlush,
                                vec![
                                    Card::new(N::A, m),
                                    Card::new(N::K, m),
                                    Card::new(N::Q, m),
                                    Card::new(N::J, m),
                                    Card::new(N::TEN, m),
                                ],
                            ));
                        }
                        4111 if self.check(&[N::A, N::FIVE, N::FOUR, N::THREE, N::TWO], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::FIVE, m),
                                    Card::new(N::FOUR, m),
                                    Card::new(N::THREE, m),
                                    Card::new(N::TWO, m),
                                    Card::new(N::A, m),
                                ],
                            ));
                        }
                        3968 if self.check(&[N::K, N::Q, N::J, N::TEN, N::NINE], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::K, m),
                                    Card::new(N::Q, m),
                                    Card::new(N::J, m),
                                    Card::new(N::TEN, m),
                                    Card::new(N::NINE, m),
                                ],
                            ));
                        }
                        1984 if self.check(&[N::Q, N::J, N::TEN, N::NINE, N::EIGHT], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::Q, m),
                                    Card::new(N::J, m),
                                    Card::new(N::TEN, m),
                                    Card::new(N::NINE, m),
                                    Card::new(N::EIGHT, m),
                                ],
                            ));
                        }
                        992 if self.check(&[N::J, N::TEN, N::NINE, N::EIGHT, N::SEVEN], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::J, m),
                                    Card::new(N::TEN, m),
                                    Card::new(N::NINE, m),
                                    Card::new(N::EIGHT, m),
                                    Card::new(N::SEVEN, m),
                                ],
                            ));
                        }
                        496 if self.check(&[N::TEN, N::NINE, N::EIGHT, N::SEVEN, N::SIX], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::TEN, m),
                                    Card::new(N::NINE, m),
                                    Card::new(N::EIGHT, m),
                                    Card::new(N::SEVEN, m),
                                    Card::new(N::SIX, m),
                                ],
                            ));
                        }
                        248 if self.check(&[N::NINE, N::EIGHT, N::SEVEN, N::SIX, N::FIVE], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::NINE, m),
                                    Card::new(N::EIGHT, m),
                                    Card::new(N::SEVEN, m),
                                    Card::new(N::SIX, m),
                                    Card::new(N::FIVE, m),
                                ],
                            ));
                        }
                        124 if self.check(&[N::EIGHT, N::SEVEN, N::SIX, N::FIVE, N::FOUR], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::EIGHT, m),
                                    Card::new(N::SEVEN, m),
                                    Card::new(N::SIX, m),
                                    Card::new(N::FIVE, m),
                                    Card::new(N::FOUR, m),
                                ],
                            ));
                        }
                        62 if self.check(&[N::SEVEN, N::SIX, N::FIVE, N::FOUR, N::THREE], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::SEVEN, m),
                                    Card::new(N::SIX, m),
                                    Card::new(N::FIVE, m),
                                    Card::new(N::FOUR, m),
                                    Card::new(N::THREE, m),
                                ],
                            ));
                        }
                        31 if self.check(&[N::SIX, N::FIVE, N::FOUR, N::THREE, N::TWO], m) => {
                            return Some(FullCombination::new(
                                Combination::StraightFlush,
                                vec![
                                    Card::new(N::SIX, m),
                                    Card::new(N::FIVE, m),
                                    Card::new(N::FOUR, m),
                                    Card::new(N::THREE, m),
                                    Card::new(N::TWO, m),
                                ],
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    fn find_four_of_kind(&mut self) -> Option<FullCombination> {
        let mut res_count: (N, i32) = (N::TWO, 0);
        for i in self.cards.iter() {
            if res_count.1 == 0 {
                res_count = (i.n, 1);
            } else {
                if res_count.0 == i.n {
                    res_count.1 += 1;
                } else {
                    res_count = (i.n, 1);
                }
            }
        }

        if res_count.1 == 4 {
            return Some(FullCombination::new(
                Combination::FourOfKind,
                vec![
                    Card::new(res_count.0, M::S),
                    Card::new(res_count.0, M::H),
                    Card::new(res_count.0, M::D),
                    Card::new(res_count.0, M::C),
                    *self.cards.iter().find(|c| c.n != res_count.0).unwrap(),
                ],
            ));
        }

        let mut map: HashMap<i32, i32> = HashMap::with_capacity(7);
        for i in self.cards.iter() {
            if !map.contains_key(&(i.n as i32)) {
                map.insert(i.n as i32, 1);
            } else {
                if let Some(x) = map.get_mut(&(i.n as i32)) {
                    *x += 1;
                }
            }
        }
        let mut prepare_n: Vec<(i32, i32)> = Vec::with_capacity(7);
        for (key, value) in map.iter() {
            prepare_n.push((*key, *value));
        }
        prepare_n.sort_by(|a, b| (b.0).partial_cmp(&(a.0)).unwrap());

        self.prepare_n = Some(prepare_n);
        None
    }

    fn find_full_house(&self) -> Option<FullCombination> {
        if let Some(ref map) = self.prepare_n {
            let mut full_house: (i32, i32) = (0, 0);

            for (key, value) in map.iter() {
                if value == &3 && full_house.0 == 0 {
                    full_house.0 = *key;
                } else if value >= &2 && full_house.1 == 0 {
                    full_house.1 = *key;
                }
            }
            if full_house.0 > 0 && full_house.1 > 0 {
                let mut cards_three_of_kind: Vec<Card> = self
                    .cards
                    .iter()
                    .filter(|c| c.n as i32 == full_house.0)
                    .map(|c| *c)
                    .collect();

                let cards_pair: Vec<Card> = self
                    .cards
                    .iter()
                    .filter(|c| c.n as i32 == full_house.1)
                    .map(|c| *c)
                    .take(2)
                    .collect();
                cards_three_of_kind.extend_from_slice(&cards_pair);
                if cards_three_of_kind.len() == 5 {
                    return Some(FullCombination::new(
                        Combination::FullHouse,
                        cards_three_of_kind,
                    ));
                }
            }
        }
        None
    }

    fn find_flash(&mut self) -> Option<FullCombination> {
        if let Some(prepare_flash) = self.prepare_flash.clone() {
            return Some(FullCombination::new(Combination::Flush, prepare_flash));
        }
        let mut m: Option<M> = None;
        let count: i32;
        if let Some(c) = self.count_n {
            count = c;
        } else {
            count = self.count_m();
        }

        if count > 10000 && count % 10000 / 1000 > 4 {
            m = Some(M::C);
        }
        if count > 1000 && count % 1000 / 100 > 4 {
            m = Some(M::D);
        }
        if count > 100 && count % 100 / 10 > 4 {
            m = Some(M::H);
        }
        if count > 10 && count % 10 / 1 > 4 {
            m = Some(M::S);
        }
        if m.is_none() {
            return None;
        }
        let m: M = m.unwrap();

        let res: Vec<Card> = self
            .cards
            .iter()
            .filter(|c| c.m == m)
            .map(|c| *c)
            .take(5)
            .collect();

        if res.len() == 5 {
            self.prepare_flash = Some(res.clone());
            return Some(FullCombination::new(Combination::Flush, res));
        }
        None
    }

    fn get_card(&self, n: N) -> Card {
        *self.cards.iter().find(|c| c.n == n).unwrap()
    }

    fn find_straight(&self) -> Option<FullCombination> {
        let hand: i32;
        if let Some(h) = self.count_n {
            hand = h;
        } else {
            hand = self.count_n();
        }

        for total in [
            0b1111100000000,
            0b0111110000000,
            0b0011111000000,
            0b0001111100000,
            0b0000111110000,
            0b0000011111000,
            0b0000001111100,
            0b0000000111110,
            0b0000000011111,
            0b1000000001111,
        ]
        .iter()
        {
            if &(hand & total) == total {
                match total {
                    7936 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::A),
                                self.get_card(N::K),
                                self.get_card(N::Q),
                                self.get_card(N::J),
                                self.get_card(N::TEN),
                            ],
                        ));
                    }
                    4111 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::FIVE),
                                self.get_card(N::FOUR),
                                self.get_card(N::THREE),
                                self.get_card(N::TWO),
                                self.get_card(N::A),
                            ],
                        ));
                    }
                    3968 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::K),
                                self.get_card(N::Q),
                                self.get_card(N::J),
                                self.get_card(N::TEN),
                                self.get_card(N::NINE),
                            ],
                        ));
                    }
                    1984 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::Q),
                                self.get_card(N::J),
                                self.get_card(N::TEN),
                                self.get_card(N::NINE),
                                self.get_card(N::EIGHT),
                            ],
                        ));
                    }
                    992 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::J),
                                self.get_card(N::TEN),
                                self.get_card(N::NINE),
                                self.get_card(N::EIGHT),
                                self.get_card(N::SEVEN),
                            ],
                        ));
                    }
                    496 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::TEN),
                                self.get_card(N::NINE),
                                self.get_card(N::EIGHT),
                                self.get_card(N::SEVEN),
                                self.get_card(N::SIX),
                            ],
                        ));
                    }
                    248 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::NINE),
                                self.get_card(N::EIGHT),
                                self.get_card(N::SEVEN),
                                self.get_card(N::SIX),
                                self.get_card(N::FIVE),
                            ],
                        ));
                    }
                    124 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::EIGHT),
                                self.get_card(N::SEVEN),
                                self.get_card(N::SIX),
                                self.get_card(N::FIVE),
                                self.get_card(N::FOUR),
                            ],
                        ));
                    }
                    62 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::SEVEN),
                                self.get_card(N::SIX),
                                self.get_card(N::FIVE),
                                self.get_card(N::FOUR),
                                self.get_card(N::THREE),
                            ],
                        ));
                    }
                    31 => {
                        return Some(FullCombination::new(
                            Combination::Straight,
                            vec![
                                self.get_card(N::SIX),
                                self.get_card(N::FIVE),
                                self.get_card(N::FOUR),
                                self.get_card(N::THREE),
                                self.get_card(N::TWO),
                            ],
                        ));
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn find_three_of_kind(&self) -> Option<FullCombination> {
        if let Some(ref map) = self.prepare_n {
            for (key, value) in map.iter() {
                if value == &3 {
                    let mut cards_three_of_kind: Vec<Card> = self
                        .cards
                        .iter()
                        .filter(|c| c.n as i32 == *key)
                        .map(|c| *c)
                        .collect();
                    let cards_hight_cards: Vec<Card> = self
                        .cards
                        .iter()
                        .filter(|c| c.n as i32 != *key)
                        .map(|c| *c)
                        .take(2)
                        .collect();
                    cards_three_of_kind.extend_from_slice(&cards_hight_cards);
                    if cards_three_of_kind.len() == 5 {
                        return Some(FullCombination::new(
                            Combination::ThreeOfKind,
                            cards_three_of_kind,
                        ));
                    }
                }
            }
        }
        None
    }

    fn find_two_pair(&self) -> Option<FullCombination> {
        if let Some(ref map) = self.prepare_n {
            let mut two_pair: (i32, i32) = (0, 0);

            for (key, value) in map.iter() {
                if value == &2 && two_pair.1 == 0 {
                    if two_pair.0 == 0 {
                        two_pair.0 = *key;
                    } else {
                        two_pair.1 = *key;
                    }
                }
            }

            if two_pair.1 != 0 {
                let mut cards_two_pair: Vec<Card> = self
                    .cards
                    .iter()
                    .filter(|c| c.n as i32 == two_pair.0 || c.n as i32 == two_pair.1)
                    .map(|c| *c)
                    .collect();
                cards_two_pair.push(
                    *self
                        .cards
                        .iter()
                        .find(|c| c.n as i32 != two_pair.0 && c.n as i32 != two_pair.1)
                        .unwrap(),
                );

                if cards_two_pair.len() == 5 {
                    return Some(FullCombination::new(Combination::TwoPairs, cards_two_pair));
                }
            }
        }
        None
    }

    fn find_pair(&self) -> Option<FullCombination> {
        if let Some(ref map) = self.prepare_n {
            let mut pair: Option<i32> = None;

            for (key, value) in map.iter() {
                if value == &2 && pair.is_none() {
                    pair = Some(*key);
                }
            }
            if let Some(n) = pair {
                let cards_pair: Vec<Card> = self
                    .cards
                    .iter()
                    .filter(|c| c.n as i32 == n)
                    .map(|c| *c)
                    .take(2)
                    .collect();

                let high_cards: Vec<Card> = self
                    .cards
                    .iter()
                    .filter(|c| c.n as i32 != n)
                    .map(|c| *c)
                    .take(3)
                    .collect();

                if cards_pair.len() == 2 && high_cards.len() == 3 {
                    return Some(FullCombination::new(
                        Combination::Pair,
                        vec![
                            cards_pair[0],
                            cards_pair[1],
                            high_cards[0],
                            high_cards[1],
                            high_cards[2],
                        ],
                    ));
                }
            }
        }
        None
    }

    fn find_high_cards(&self) -> Option<FullCombination> {
        return Some(FullCombination::new(
            Combination::HighCard,
            vec![
                self.cards[0],
                self.cards[1],
                self.cards[2],
                self.cards[3],
                self.cards[4],
            ],
        ));
    }
}

/*
 impl From<JsValue> for Hand {
    #[inline]
    fn from(h: JsValue) -> Hand {
        JsValue::from_str(&s)
    }
}

impl  Into<JsValue> for Card {
    fn into(self) -> Card {
        Card{n:N::TWO,m:M::D}
    }
}
*/

#[wasm_bindgen(skip)]
pub struct Menager {
    hands: Vec<Hand>,
}

#[wasm_bindgen]
impl Menager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { hands:vec![] }
    }
    pub fn add(&mut self,hand:Hand){
       self.hands.push(hand);
    }

    pub fn calculate_wasm(&mut self) -> Option<js_sys::Array> {
        let mut map: HashMap<Combination, bool> =
            HashMap::with_capacity(self.hands.len());

        let ret = js_sys::Array::new_with_length(self.hands.len() as u32);
        let mut last_index:u8 = 0;
        for (index,hand) in self.hands.iter_mut().enumerate() {
            let mut full = hand.find().unwrap();
            
            if map.contains_key(&full.combination) {
                full.key_range = last_index;
                ret.set(index as u32,wasm_bindgen::JsValue::from(full) );
            } else {
                last_index = index as u8;
                full.key_range = last_index;
                map.insert(full.combination, true);
                ret.set(index as u32, wasm_bindgen::JsValue::from(full) ); 
            }
        }
/*
        let mut result: Vec<Vec<FullCombination>> = vec![];
        for (key, val) in map.iter() {
            result.push(val.clone());
        }
        result.sort_by(|a, b| (b[0].combination).partial_cmp(&(a[0].combination)).unwrap());

        let ret = js_sys::Array::new_with_length(result.len() as u32);
        for (index,value) in result.iter().enumerate(){
           let item = js_sys::Array::new_with_length(value.len() as u32);
           for (index2,value2) in value.into_iter().enumerate(){
            item.set(index2 as u32,(self.n as i32).into());
          }
          ret.set(index as u32,item);
        }
       
        Some(result)*/
        Some(ret)
        
    }
}
 
impl Menager {
 
    pub fn calculate(&mut self) -> Option<Vec<Vec<FullCombination>>> {
        let mut map: HashMap<Combination, Vec<FullCombination>> =
            HashMap::with_capacity(self.hands.len());
        for hand in self.hands.iter_mut() {
            let full = hand.find().unwrap();
            if map.contains_key(&full.combination) {
                if let Some(value) = map.get_mut(&full.combination) {
                    (*value).push(full);
                }
            } else {
                map.insert(full.combination, vec![full]);
            }
        }
        let mut result: Vec<Vec<FullCombination>> = vec![];
        for (key, val) in map.iter() {
            result.push(val.clone());
        }
        result.sort_by(|a, b| (b[0].combination).partial_cmp(&(a[0].combination)).unwrap());
        Some(result)
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_partiql_eq_combination() {
        let mut hand = Hand::new(
            "Pair 10c,10s,Ad,Js,9c",
            Card::new(N::J, M::S),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::TEN, M::S),
            Card::new(N::A, M::D),
            Card::new(N::NINE, M::C),
        );
        let pair10: FullCombination = hand.find().unwrap();

        let mut hand = Hand::new(
            "Pair 9s,9d,Ad,10c,4c",
            Card::new(N::NINE, M::S),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::NINE, M::D),
            Card::new(N::A, M::D),
            Card::new(N::FOUR, M::C),
        );
        let pair9s: FullCombination = hand.find().unwrap();
        assert!(pair9s < pair10);

        let mut hand = Hand::new(
            "Pair 9h,9d,Ad,10c,4c",
            Card::new(N::NINE, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::NINE, M::D),
            Card::new(N::A, M::D),
            Card::new(N::FOUR, M::C),
        );

        let pair9h: FullCombination = hand.find().unwrap();
        assert!(pair9s == pair9h);
    }

    #[test]
    fn it_partiql_eq_card() {
        assert!(Card::new(N::J, M::S) > Card::new(N::TEN, M::S));
        assert_ne!(Card::new(N::J, M::S), Card::new(N::J, M::D));
    }

    #[test]
    fn it_success_combination_high_card() {
        let mut hand = Hand::new(
            "HighCard Kd,Qd,10c,9h,4c",
            Card::new(N::NINE, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::FOUR, M::C),
        );
        let high_card: FullCombination = hand.find().unwrap();
        assert!(high_card.combination == Combination::HighCard);
    }

    #[test]
    fn it_success_combination_pair() {
        let mut hand = Hand::new(
            "Pair 4h,4c,Kd,Qd,10c",
            Card::new(N::FOUR, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::FOUR, M::C),
        );
        let pair: FullCombination = hand.find().unwrap();
        assert!(pair.combination == Combination::Pair);
    }

    #[test]
    fn it_success_combination_two_pair() {
        let mut hand = Hand::new(
            "TwoPairs Ks,Kd,4h,4c,Qd",
            Card::new(N::FOUR, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::K, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::FOUR, M::C),
        );
        let two_pair: FullCombination = hand.find().unwrap();
        assert!(two_pair.combination == Combination::TwoPairs);
    }

    #[test]
    fn it_success_combination_three_of_kind() {
        let mut hand = Hand::new(
            "ThreeOfKind 4h,4s,4c,Kd,Qd",
            Card::new(N::FOUR, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::FOUR, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::FOUR, M::C),
        );
        let three_of_kind: FullCombination = hand.find().unwrap();
        assert!(three_of_kind.combination == Combination::ThreeOfKind);
    }

    #[test]
    fn it_success_combination_straight() {
        let mut hand = Hand::new(
            "Straight Kd,Qd,Js,10c,9h",
            Card::new(N::NINE, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::J, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::FOUR, M::C),
        );
        let straight: FullCombination = hand.find().unwrap();
        assert!(straight.combination == Combination::Straight);
    }

    #[test]
    fn it_success_combination_flash() {
        let mut hand = Hand::new(
            "Flush Kd,Qd,9d,4d,3d",
            Card::new(N::NINE, M::D),
            Card::new(N::THREE, M::D),
            Card::new(N::J, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::FOUR, M::D),
        );
        let flash: FullCombination = hand.find().unwrap();
        println!("{}", flash);
        assert!(flash.combination == Combination::Flush);
    }

    #[test]
    fn it_success_combination_full_house() {
        let mut hand = Hand::new(
            "FullHouse 3d,3s,3c,Js,Jc",
            Card::new(N::THREE, M::D),
            Card::new(N::THREE, M::S),
            Card::new(N::J, M::S),
            Card::new(N::J, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::THREE, M::C),
        );
        let full_house: FullCombination = hand.find().unwrap();
        assert!(full_house.combination == Combination::FullHouse);
    }

    #[test]
    fn it_success_combination_four_of_kind() {
        let mut hand = Hand::new(
            "FourOfKind 3s,3h,3d,3c,Qd",
            Card::new(N::THREE, M::D),
            Card::new(N::THREE, M::S),
            Card::new(N::J, M::S),
            Card::new(N::J, M::C),
            Card::new(N::THREE, M::H),
            Card::new(N::Q, M::D),
            Card::new(N::THREE, M::C),
        );
        let four_of_kind: FullCombination = hand.find().unwrap();
        println!("{}", four_of_kind);
        assert!(four_of_kind.combination == Combination::FourOfKind);
    }

    #[test]
    fn it_success_combination_straight_flush() {
        let mut hand = Hand::new(
            "StraightFlush Kd,Qd,Jd,10d,9d",
            Card::new(N::TEN, M::D),
            Card::new(N::THREE, M::S),
            Card::new(N::J, M::D),
            Card::new(N::J, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::NINE, M::D),
        );
        let straight_flush: FullCombination = hand.find().unwrap();
        assert!(straight_flush.combination == Combination::StraightFlush);
    }

    #[test]
    fn it_success_combination_royal_flush() {
        let mut hand = Hand::new(
            "RoyalFlush Ad,Kd,Qd,Jd,10d",
            Card::new(N::TEN, M::D),
            Card::new(N::THREE, M::S),
            Card::new(N::J, M::D),
            Card::new(N::J, M::C),
            Card::new(N::K, M::D),
            Card::new(N::Q, M::D),
            Card::new(N::A, M::D),
        );
        let royal_flush: FullCombination = hand.find().unwrap();
        assert!(royal_flush.combination == Combination::RoyalFlush);
    }
}
