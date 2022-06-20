mod utils;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
#[derive(Hash, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
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
/*
impl PartialEq<Combination> for Combination {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}
impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self as u8).partial_cmp(&(*other as u8)).unwrap() {
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
        }
    }
}
impl Eq for Combination {}
*/
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

#[wasm_bindgen(skip)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Total {
    pub combination: Combination,
    pub key_range_group: u8,
    player_id: String,
    cards: Vec<Card>,
    win_pot: i32,
}
impl PartialEq<Total> for Total {
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
impl PartialOrd for Total {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.combination.partial_cmp(&other.combination).unwrap() {
            Ordering::Equal => self.cards.partial_cmp(&other.cards),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
        }
    }
}

impl fmt::Display for Total {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} win:{} player_id:{} cards:{},{},{},{},{}",
            self.combination,
            self.win_pot,
            self.player_id,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4]
        )
    }
}

#[wasm_bindgen]
impl Total {
    #[wasm_bindgen(js_name = get_cards)]
    pub fn get_cards(&self) -> js_sys::Array {
        let ret = js_sys::Array::new_with_length(self.cards.len() as u32);
        for (index, c) in self.cards.iter().enumerate() {
            ret.set(index as u32, wasm_bindgen::JsValue::from(*c));
        }
        ret
    }
    pub fn show_cards(&self) -> js_sys::Array {
        let ret = js_sys::Array::new_with_length(self.cards.len() as u32);
        for (index, c) in self.cards.iter().enumerate() {
            ret.set(index as u32, wasm_bindgen::JsValue::from(format!("{}", *c)));
        }
        ret
    }
    pub fn show_combination(&self) -> String {
        format!("{}", self.combination)
    }
    pub fn get_player_id(&self) -> String {
        self.player_id.clone()
    }
    pub fn get_win_pot(&self) -> i32 {
        self.win_pot
    }
}

impl Total {
    fn new(combination: Combination, player_id: String /*&str*/, cards: Vec<Card>) -> Self {
        Self {
            combination,
            key_range_group: 0,
            player_id, /*: Box::leak(String::from(player_id).into_boxed_str())*/
            cards,
            win_pot: 0,
        }
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
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.n, self.m)
    }
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(n: N, m: M) -> Self {
        Self { n, m }
    }
    pub fn get(self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from(self)
    }
    pub fn show_card(&self) -> String {
        format!("{}", self)
    }
}

#[wasm_bindgen(skip)]
pub struct Hand {
    player_id: String,
    cards: Vec<Card>,
    prepare_n: Option<Vec<(i32, i32)>>,
    prepare_flash: Option<Vec<Card>>,
    count_n: Option<i32>,
    total_bet: i32,
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hand{{ player_id:{},total_bet:{}, cards:{},{},{},{},{},{},{} }}",
            self.player_id,
            self.total_bet,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
            self.cards[5],
            self.cards[6]
        )
    }
}

#[wasm_bindgen]
impl Hand {
    #[wasm_bindgen(constructor)]
    pub fn new(
        player_id: String,
        total_bet: i32,
        c1: Card,
        c2: Card,
        c3: Card,
        c4: Card,
        c5: Card,
        c6: Card,
        c7: Card,
    ) -> Self {
        let mut h = Hand {
            player_id, /*: Box::leak(String::from(player_id).into_boxed_str()),*/
            cards: vec![c1, c2, c3, c4, c5, c6, c7],
            prepare_n: None,
            prepare_flash: None,
            count_n: None,
            total_bet: total_bet,
        };
        h.cards
            .sort_by(|a, b| (b.n as i32).partial_cmp(&(a.n as i32)).unwrap());
        h.cards.dedup();
        if h.cards.len() < 7 {
            panic!("Cards not correct");
        }
        h
    }
    pub fn show_hand(&self) -> String {
        format!("{}", self)
    }
}

impl Hand {
    pub fn find(&mut self) -> Option<Total> {
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

    fn find_royal_and_staight_flash(&mut self) -> Option<Total> {
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
                            return Some(Total::new(
                                Combination::RoyalFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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
                            return Some(Total::new(
                                Combination::StraightFlush,
                                self.player_id.clone(),
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

    fn find_four_of_kind(&mut self) -> Option<Total> {
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
            return Some(Total::new(
                Combination::FourOfKind,
                self.player_id.clone(),
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

    fn find_full_house(&self) -> Option<Total> {
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
                    return Some(Total::new(
                        Combination::FullHouse,
                        self.player_id.clone(),
                        cards_three_of_kind,
                    ));
                }
            }
        }
        None
    }

    fn find_flash(&mut self) -> Option<Total> {
        if let Some(ref prepare_flash) = self.prepare_flash {
            return Some(Total::new(
                Combination::Flush,
                self.player_id.clone(),
                prepare_flash.clone(),
            ));
        }
        let mut m: Option<M> = None;
        let count: i32;
        if let Some(c) = self.count_n {
            count = c;
        } else {
            count = self.count_m();
        }

        {
            let count: String = format!("{:04}", count);
            for (index, c) in count.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    if digit > 4 {
                        match index {
                            0 => {
                                m = Some(M::C);
                            }
                            1 => {
                                m = Some(M::D);
                            }
                            2 => {
                                m = Some(M::H);
                            }
                            3 => {
                                m = Some(M::S);
                            }
                            _ => {}
                        }
                    }
                }
            }
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
            return Some(Total::new(Combination::Flush, self.player_id.clone(), res));
        }
        None
    }

    fn get_card(&self, n: N) -> Card {
        *self.cards.iter().find(|c| c.n == n).unwrap()
    }

    fn find_straight(&self) -> Option<Total> {
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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
                        return Some(Total::new(
                            Combination::Straight,
                            self.player_id.clone(),
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

    fn find_three_of_kind(&self) -> Option<Total> {
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
                        return Some(Total::new(
                            Combination::ThreeOfKind,
                            self.player_id.clone(),
                            cards_three_of_kind,
                        ));
                    }
                }
            }
        }
        None
    }

    fn find_two_pair(&self) -> Option<Total> {
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
                    return Some(Total::new(
                        Combination::TwoPairs,
                        self.player_id.clone(),
                        cards_two_pair,
                    ));
                }
            }
        }
        None
    }

    fn find_pair(&self) -> Option<Total> {
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
                    return Some(Total::new(
                        Combination::Pair,
                        self.player_id.clone(),
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

    fn find_high_cards(&self) -> Option<Total> {
        return Some(Total::new(
            Combination::HighCard,
            self.player_id.clone(),
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

#[wasm_bindgen(skip)]
pub struct Menager {
    hands: Vec<Hand>,
    pot: i32,
}

#[wasm_bindgen]
impl Menager {
    #[wasm_bindgen(constructor)]
    pub fn new(pot: i32) -> Self {
        set_panic_hook();
        Self {
            hands: vec![],
            pot: pot, /*,win_combinations:None*/
        }
    }
    pub fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }
    pub fn calculate(mut self) -> Option<js_sys::Array> {
        // Total
        let mut totals: Vec<Total> = self.prepare_calculate();
        // console_log!("totals={}",format!("{:#?}",totals));
        let mut pot: Pot = Pot::new(self.pot);
        let mut group: Vec<String> = vec![];
        group.push(totals[0].get_player_id());
        let mut key_range_group = totals[0].key_range_group;
        for w in totals.iter().skip(1) {
            if key_range_group != w.key_range_group {
                pot.add_next_group_win_vec(group.clone());
                key_range_group = w.key_range_group;
                group = vec![];
                group.push(w.get_player_id());
            } else {
                group.push(w.get_player_id());
            }
        }
        if group.len() > 0 {
            pot.add_next_group_win_vec(group.clone());
        }
        for player_hand in self.hands.iter() {
            pot.add_player(&player_hand.player_id, player_hand.total_bet);
        }
        let pot_calculate: Vec<(String, i32)> = pot.calculate()?;
        for player_win in pot_calculate.iter() {
            for total_win in totals.iter_mut() {
                if total_win.player_id == player_win.0 {
                    total_win.win_pot = player_win.1;
                    break;
                }
            }
        }
        totals = totals.into_iter().filter(|pl| pl.win_pot > 0).collect();

        let ret = js_sys::Array::new_with_length(totals.len() as u32);
        for (index, el) in totals.into_iter().enumerate() {
            ret.set(
                index as u32,
                wasm_bindgen::JsValue::from_serde(&el).unwrap(),
            );
        }
        Some(ret)
    }
}

impl Menager {
    pub fn prepare_calculate(&mut self) -> Vec<Total> {
        let mut key_range_group: u8 = 0;
        let mut ret: Vec<Total> = Vec::with_capacity(self.hands.len() as usize);
        for hand in self.hands.iter_mut() {
            let mut c = hand.find().unwrap();
            c.key_range_group = key_range_group;
            ret.push(c);
        }
        ret.sort_by(|a, b| {
            let partial = (b.combination).partial_cmp(&(a.combination)).unwrap();
            if partial == Ordering::Equal {
                return b.cards.partial_cmp(&a.cards).unwrap();
            }
            partial
        });
        let mut current: Total = ret.first().unwrap().clone();
        for el in ret.iter_mut().skip(1) {
            if &current != el {
                key_range_group += 1;
                el.key_range_group = key_range_group;
            } else {
                el.key_range_group = key_range_group;
            }
            current = el.clone();
        }
        ret
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(skip)]
pub struct Pot {
    pot: i32,
    players: Vec<(String, i32)>,
    win_queue_group: Vec<Vec<String>>,
}

#[wasm_bindgen]
impl Pot {
    #[wasm_bindgen(constructor)]
    pub fn new(pot: i32) -> Self {
        Pot {
            pot,
            players: vec![],
            win_queue_group: vec![],
        }
    }
}

impl Pot {
    pub fn add_player(&mut self, id: &str, bet: i32) {
        self.players.push((id.to_owned(), bet));
    }
    pub fn add_next_group_win_vec(&mut self, win: Vec<String>) {
        self.win_queue_group.push(win);
    }

    pub fn calculate(mut self) -> Option<Vec<(String, i32)> /*js_sys::Array*/> {
        if !self.check() {
            console_log!("{}", format!("Fail validation params\n {:#?}", self));
            return None;
        }
        let mut result: Vec<(String, i32)> = vec![];
        let mut pot: i32 = self.pot;
        for w in self.win_queue_group.iter() {
            if pot > 0 && self.players.len() > 0 {
                let ids_group = w.iter().map(|v| v.to_owned()).collect::<Vec<String>>();
                let group_sum_bet: i32 = self
                    .players
                    .iter()
                    .filter(|u| ids_group.contains(&u.0))
                    .map(|u| u.1)
                    .sum::<i32>();

                let group_max_pot: i32 =
                    w.iter()
                        .map(|id| {
                            (
                                id,
                                self.players.iter().find_map(|u| {
                                    if &u.0 == id {
                                        Some(u.1)
                                    } else {
                                        None
                                    }
                                }),
                            )
                        })
                        .map(|(_id, bet_pot)| {
                            self.sum_bet(bet_pot.unwrap(), ids_group.clone()).unwrap()
                        })
                        .max()
                        .unwrap();

                let mut total_chank: i32 = 0;
                let mut res: Vec<(String, i32)> = self
                    .players
                    .iter()
                    .filter(|u| ids_group.contains(&u.0))
                    .map(|u| {
                        let bank =
                            (group_max_pot as f32 / (group_sum_bet as f32 / u.1 as f32)) as f32;
                        total_chank += bank as i32;
                        pot -= bank as i32;
                        (u.0.to_owned(), bank as i32 + u.1)
                    })
                    .collect();

                let last: i32 = group_max_pot - total_chank;
                if last > 0 {
                    res[0].1 += last;
                    pot -= last;
                }
                for i in res.into_iter() {
                    result.push(i.clone());
                }

                self.players = self
                    .players
                    .iter()
                    .filter(|u| !ids_group.contains(&u.0) && (u.1 - group_sum_bet) > 0)
                    .map(|u| (u.0.to_owned(), u.1 - group_sum_bet))
                    .collect::<Vec<(String, i32)>>();

                pot -= group_sum_bet;
            }
        }
        if pot > 0 {
            let chank = pot / result.len() as i32;
            for i in result.iter_mut() {
                i.1 += chank;
            }
        }
        return Some(result);
        /* let wins = js_sys::Array::new_with_length(result.len() as u32);
        for (index, w) in result.into_iter().enumerate() {
            wins.set(
                index as u32,
                wasm_bindgen::JsValue::from_serde(&Win::new(w.0, w.1)).unwrap(),
            );
        }

        return Some(wins);*/
    }
    fn sum_bet(&self, bet_pot: i32, id_group: Vec<String>) -> Option<i32> {
        let mut sum = 0;
        for i in self.players.iter() {
            if !id_group.contains(&i.0) {
                sum += if bet_pot >= i.1 { i.1 } else { bet_pot };
            }
        }
        Some(sum)
    }
    fn check(&self) -> bool {
        {
            let current_pot = self.players.iter().map(|p| p.1).sum();
            if self.pot < current_pot {
                console_log!(
                    "{}",
                    format!(
                        "The total pot {} is less than the players bet {}",
                        self.pot, current_pot
                    )
                );
                return false;
            }
        }

        {
            for p in self.players.iter() {
                let mut is_find = false;
                for w in self.win_queue_group.iter() {
                    for u in w.iter() {
                        if u == &p.0 {
                            is_find = true;
                        }
                    }
                }
                if is_find == false {
                    console_log!("{}", format!("Not a complete list of players per pot"));
                    return is_find;
                }
            }
        }

        let max = self.players.iter().map(|u| u.1).max().unwrap();
        if self
            .players
            .iter()
            .filter(|u| u.1 == max)
            .map(|_| 1)
            .count()
            < 2
        {
            console_log!("{}", format!("Unmatched bet {}", max));
            return false;
        }

        true
    }
}

// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;


// cargo test it_partiql_eq_combination -- --nocapture
    #[test]
    fn it_partiql_eq_combination() {
        let mut hand = Hand::new(
            "1".to_owned(),
            1,
            Card::new(N::J, M::S),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::TEN, M::S),
            Card::new(N::A, M::D),
            Card::new(N::NINE, M::C),
        );
        let pair10: Total = hand.find().unwrap();

        let mut hand = Hand::new(
            "2".to_owned(),
            1,
            Card::new(N::NINE, M::S),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::NINE, M::D),
            Card::new(N::A, M::D),
            Card::new(N::FOUR, M::C),
        );
        let pair9s: Total = hand.find().unwrap();
        assert!(pair9s < pair10);

        let mut hand = Hand::new(
            "3".to_owned(),
            1,
            Card::new(N::NINE, M::H),
            Card::new(N::THREE, M::S),
            Card::new(N::TWO, M::S),
            Card::new(N::TEN, M::C),
            Card::new(N::NINE, M::D),
            Card::new(N::A, M::D),
            Card::new(N::FOUR, M::C),
        );

        let pair9h: Total = hand.find().unwrap();
        assert!(pair9s == pair9h);
    }

    #[test]
    fn it_partiql_eq_card() {
        assert!(Card::new(N::J, M::S) > Card::new(N::TEN, M::S));
        assert_ne!(Card::new(N::J, M::S), Card::new(N::J, M::D));
    }

    #[test]
    fn it_success_combination_high_card() {
        let c1 = Card::new(N::NINE, M::H);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::TWO, M::S);
        let c4 = Card::new(N::TEN, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::FOUR, M::C);
        let mut hand = Hand::new("HighCard Kd,Qd,10c,9h,4c".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c5, c6, c4, c1, c7], flash.cards.as_slice());
        assert!(flash.combination == Combination::HighCard);
    }

    #[test]
    fn it_success_combination_pair() {
        let c1 = Card::new(N::FOUR, M::H);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::TWO, M::S);
        let c4 = Card::new(N::TEN, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::FOUR, M::C);
        let mut hand = Hand::new("Pair 4h,4c,Kd,Qd,10c".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c1, c7, c5, c6, c4], flash.cards.as_slice());
        assert!(flash.combination == Combination::Pair);
    }

    #[test]
    fn it_success_combination_two_pair() {
        let c1 = Card::new(N::FOUR, M::H);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::K, M::S);
        let c4 = Card::new(N::TEN, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::FOUR, M::C);
        let mut hand = Hand::new("TwoPairs Ks,Kd,4h,4c,Qd".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c3, c5, c1, c7, c6], flash.cards.as_slice());
        assert!(flash.combination == Combination::TwoPairs);
    }

    #[test]
    fn it_success_combination_three_of_kind() {
        let c1 = Card::new(N::FOUR, M::H);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::FOUR, M::S);
        let c4 = Card::new(N::TEN, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::FOUR, M::C);
        let mut hand = Hand::new("ThreeOfKind 4h,4s,4c,Kd,Qd".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c1, c3, c7, c5, c6], flash.cards.as_slice());
        assert!(flash.combination == Combination::ThreeOfKind);
    }

    #[test]
    fn it_success_combination_straight() {
        let c1 = Card::new(N::NINE, M::H);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::J, M::S);
        let c4 = Card::new(N::TEN, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::FOUR, M::C);
        let mut hand = Hand::new("Straight Kd,Qd,Js,10c,9h".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c5, c6, c3, c4, c1], flash.cards.as_slice());
        assert!(flash.combination == Combination::Straight);
    }

    #[test]
    fn it_success_combination_flash() {
        {
            let c1 = Card::new(N::SIX, M::S);
            let c2 = Card::new(N::FIVE, M::C);
            let c3 = Card::new(N::TWO, M::S);
            let c4 = Card::new(N::TEN, M::C);
            let c5 = Card::new(N::TEN, M::S);
            let c6 = Card::new(N::A, M::S);
            let c7 = Card::new(N::NINE, M::S);
            let mut hand = Hand::new("№6 6s,5c,2s,10c,10s,As,9s".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
            let flash: Total = hand.find().unwrap();
            assert_eq!(&[c6, c5, c7, c1, c3], flash.cards.as_slice());
            assert!(flash.combination == Combination::Flush);
        }

        let c1 = Card::new(N::SIX, M::H);
        let c2 = Card::new(N::FIVE, M::H);
        let c3 = Card::new(N::TWO, M::S);
        let c4 = Card::new(N::TEN, M::H);
        let c5 = Card::new(N::TEN, M::S);
        let c6 = Card::new(N::A, M::H);
        let c7 = Card::new(N::NINE, M::H);
        let mut hand = Hand::new("№6 6h,5h,2s,10h,10s,Ah,9h".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c6, c4, c7, c1, c2], flash.cards.as_slice());
        assert!(flash.combination == Combination::Flush);
    }

    #[test]
    fn it_success_combination_full_house() {
        let c1 = Card::new(N::THREE, M::D);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::J, M::S);
        let c4 = Card::new(N::J, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::THREE, M::C);
        let mut hand = Hand::new("FullHouse 3d,3s,3c,Js,Jc".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c1, c2, c7, c3, c4], flash.cards.as_slice());
        assert!(flash.combination == Combination::FullHouse);
    }

    #[test]
    fn it_success_combination_four_of_kind() {
        let c1 = Card::new(N::THREE, M::D);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::J, M::S);
        let c4 = Card::new(N::J, M::C);
        let c5 = Card::new(N::THREE, M::H);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::THREE, M::C);
        let mut hand = Hand::new("FourOfKind 3s,3h,3d,3c,Qd".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c2, c5, c1, c7, c6], flash.cards.as_slice());
        assert!(flash.combination == Combination::FourOfKind);
    }

    #[test]
    fn it_success_combination_straight_flush() {
        let c1 = Card::new(N::TEN, M::D);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::J, M::D);
        let c4 = Card::new(N::J, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::NINE, M::D);
        let mut hand = Hand::new("StraightFlush Kd,Qd,Jd,10d,9d".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c5, c6, c3, c1, c7], flash.cards.as_slice());
        assert!(flash.combination == Combination::StraightFlush);
    }

    #[test]
    fn it_success_combination_royal_flush() {
        let c1 = Card::new(N::TEN, M::D);
        let c2 = Card::new(N::THREE, M::S);
        let c3 = Card::new(N::J, M::D);
        let c4 = Card::new(N::J, M::C);
        let c5 = Card::new(N::K, M::D);
        let c6 = Card::new(N::Q, M::D);
        let c7 = Card::new(N::A, M::D);
        let mut hand = Hand::new("RoyalFlush Ad,Kd,Qd,Jd,10d".to_owned(),1, c1, c2, c3, c4, c5, c6, c7);
        let flash: Total = hand.find().unwrap();
        assert_eq!(&[c7, c5, c6, c3, c1], flash.cards.as_slice());
        assert!(flash.combination == Combination::RoyalFlush);
    }
}
