/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Combination {
  RoyalFlush,
  StraightFlush,
  FourOfKind,
  FullHouse,
  Flush,
  Straight,
  ThreeOfKind,
  TwoPairs,
  Pair,
  HighCard,
}
/**
*/
export enum M {
  S,
  H,
  D,
  C,
}
/**
*/
export enum N {
  TWO,
  THREE,
  FOUR,
  FIVE,
  SIX,
  SEVEN,
  EIGHT,
  NINE,
  TEN,
  J,
  Q,
  K,
  A,
}
/**
*/
export class Card {
  free(): void;
/**
* @param {number} n
* @param {number} m
*/
  constructor(n: number, m: number);
/**
* @returns {any}
*/
  get(): any;
/**
* @returns {string}
*/
  show_card(): string;
/**
*/
  m: number;
/**
*/
  n: number;
}
/**
*/
export class FullCombination {
  free(): void;
/**
* @returns {Array<any>}
*/
  get_cards(): Array<any>;
/**
* @returns {Array<any>}
*/
  show_cards(): Array<any>;
/**
* @returns {string}
*/
  show_combination(): string;
/**
* @returns {string}
*/
  get_key_hand(): string;
/**
*/
  combination: number;
/**
*/
  key_range_group: number;
}
/**
*/
export class Hand {
  free(): void;
/**
* @param {string} key
* @param {Card} c1
* @param {Card} c2
* @param {Card} c3
* @param {Card} c4
* @param {Card} c5
* @param {Card} c6
* @param {Card} c7
*/
  constructor(key: string, c1: Card, c2: Card, c3: Card, c4: Card, c5: Card, c6: Card, c7: Card);
/**
* @returns {string}
*/
  show_hand(): string;
}
/**
*/
export class Menager {
  free(): void;
/**
*/
  constructor();
/**
* @param {Hand} hand
*/
  add_hand(hand: Hand): void;
/**
* @param {Pot} pot
* @returns {boolean}
*/
  add_pot(pot: Pot): boolean;
/**
* @returns {Array<any> | undefined}
*/
  calculate_wasm(): Array<any> | undefined;
/**
* @returns {Array<any> | undefined}
*/
  get_win_combinations(): Array<any> | undefined;
/**
* @returns {Array<any> | undefined}
*/
  calculate_pot(): Array<any> | undefined;
}
/**
*/
export class Pot {
  free(): void;
/**
* @param {number} pot
*/
  constructor(pot: number);
/**
* @param {number} id
* @param {number} bet
*/
  add_player(id: number, bet: number): void;
/**
* @param {Int32Array} win_js
*/
  add_next_group_win(win_js: Int32Array): void;
/**
* @param {Int32Array} win
*/
  add_next_group_win_vec(win: Int32Array): void;
/**
* @returns {Array<any> | undefined}
*/
  calculate(): Array<any> | undefined;
}
/**
*/
export class Win {
  free(): void;
/**
*/
  id: number;
/**
*/
  pot: number;
}
