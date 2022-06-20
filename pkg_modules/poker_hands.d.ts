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
export class Hand {
  free(): void;
/**
* @param {string} player_id
* @param {number} total_bet
* @param {Card} c1
* @param {Card} c2
* @param {Card} c3
* @param {Card} c4
* @param {Card} c5
* @param {Card} c6
* @param {Card} c7
*/
  constructor(player_id: string, total_bet: number, c1: Card, c2: Card, c3: Card, c4: Card, c5: Card, c6: Card, c7: Card);
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
* @param {number} pot
*/
  constructor(pot: number);
/**
* @param {Hand} hand
*/
  add_hand(hand: Hand): void;
/**
* @returns {Array<any> | undefined}
*/
  calculate(): Array<any> | undefined;
}
/**
*/
export class Pot {
  free(): void;
/**
* @param {number} pot
*/
  constructor(pot: number);
}
/**
*/
export class Total {
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
  get_player_id(): string;
/**
* @returns {number}
*/
  get_win_pot(): number;
/**
*/
  combination: number;
/**
*/
  key_range_group: number;
}
