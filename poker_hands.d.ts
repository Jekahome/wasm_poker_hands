declare namespace wasm_bindgen {
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
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly __wbg_fullcombination_free: (a: number) => void;
  readonly __wbg_get_fullcombination_combination: (a: number) => number;
  readonly __wbg_set_fullcombination_combination: (a: number, b: number) => void;
  readonly __wbg_get_fullcombination_key_range: (a: number) => number;
  readonly __wbg_set_fullcombination_key_range: (a: number, b: number) => void;
  readonly fullcombination_get_cards: (a: number) => number;
  readonly __wbg_card_free: (a: number) => void;
  readonly __wbg_get_card_n: (a: number) => number;
  readonly __wbg_set_card_n: (a: number, b: number) => void;
  readonly __wbg_get_card_m: (a: number) => number;
  readonly __wbg_set_card_m: (a: number, b: number) => void;
  readonly card_new: (a: number, b: number) => number;
  readonly card_get: (a: number) => number;
  readonly __wbg_hand_free: (a: number) => void;
  readonly hand_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly __wbg_menager_free: (a: number) => void;
  readonly menager_new: () => number;
  readonly menager_add: (a: number, b: number) => void;
  readonly menager_calculate_wasm: (a: number) => number;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_thread_destroy: () => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>, maybe_memory?: WebAssembly.Memory): Promise<InitOutput>;
