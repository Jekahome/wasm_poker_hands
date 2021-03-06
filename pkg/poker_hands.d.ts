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
*/
  combination: number;
/**
*/
  key_range: number;
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
  add(hand: Hand): void;
/**
* @returns {Array<any> | undefined}
*/
  calculate_wasm(): Array<any> | undefined;
}
