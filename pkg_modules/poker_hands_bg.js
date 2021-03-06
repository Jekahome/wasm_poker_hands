import * as wasm from './poker_hands_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
export const Combination = Object.freeze({ RoyalFlush:9,"9":"RoyalFlush",StraightFlush:8,"8":"StraightFlush",FourOfKind:7,"7":"FourOfKind",FullHouse:6,"6":"FullHouse",Flush:5,"5":"Flush",Straight:4,"4":"Straight",ThreeOfKind:3,"3":"ThreeOfKind",TwoPairs:2,"2":"TwoPairs",Pair:1,"1":"Pair",HighCard:0,"0":"HighCard", });
/**
*/
export const M = Object.freeze({ S:1,"1":"S",H:10,"10":"H",D:100,"100":"D",C:1000,"1000":"C", });
/**
*/
export const N = Object.freeze({ TWO:1,"1":"TWO",THREE:2,"2":"THREE",FOUR:4,"4":"FOUR",FIVE:8,"8":"FIVE",SIX:16,"16":"SIX",SEVEN:32,"32":"SEVEN",EIGHT:64,"64":"EIGHT",NINE:128,"128":"NINE",TEN:256,"256":"TEN",J:512,"512":"J",Q:1024,"1024":"Q",K:2048,"2048":"K",A:4096,"4096":"A", });
/**
*/
export class Card {

    static __wrap(ptr) {
        const obj = Object.create(Card.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_card_free(ptr);
    }
    /**
    */
    get n() {
        const ret = wasm.__wbg_get_card_n(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set n(arg0) {
        wasm.__wbg_set_card_n(this.ptr, arg0);
    }
    /**
    */
    get m() {
        const ret = wasm.__wbg_get_card_m(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set m(arg0) {
        wasm.__wbg_set_card_m(this.ptr, arg0);
    }
    /**
    * @param {number} n
    * @param {number} m
    */
    constructor(n, m) {
        const ret = wasm.card_new(n, m);
        return Card.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    get() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.card_get(ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string}
    */
    show_card() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.card_show_card(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
*/
export class Hand {

    static __wrap(ptr) {
        const obj = Object.create(Hand.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hand_free(ptr);
    }
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
    constructor(player_id, total_bet, c1, c2, c3, c4, c5, c6, c7) {
        const ptr0 = passStringToWasm0(player_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(c1, Card);
        var ptr1 = c1.ptr;
        c1.ptr = 0;
        _assertClass(c2, Card);
        var ptr2 = c2.ptr;
        c2.ptr = 0;
        _assertClass(c3, Card);
        var ptr3 = c3.ptr;
        c3.ptr = 0;
        _assertClass(c4, Card);
        var ptr4 = c4.ptr;
        c4.ptr = 0;
        _assertClass(c5, Card);
        var ptr5 = c5.ptr;
        c5.ptr = 0;
        _assertClass(c6, Card);
        var ptr6 = c6.ptr;
        c6.ptr = 0;
        _assertClass(c7, Card);
        var ptr7 = c7.ptr;
        c7.ptr = 0;
        const ret = wasm.hand_new(ptr0, len0, total_bet, ptr1, ptr2, ptr3, ptr4, ptr5, ptr6, ptr7);
        return Hand.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    show_hand() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.hand_show_hand(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
*/
export class Menager {

    static __wrap(ptr) {
        const obj = Object.create(Menager.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_menager_free(ptr);
    }
    /**
    * @param {number} pot
    */
    constructor(pot) {
        const ret = wasm.menager_new(pot);
        return Menager.__wrap(ret);
    }
    /**
    * @param {Hand} hand
    */
    add_hand(hand) {
        _assertClass(hand, Hand);
        var ptr0 = hand.ptr;
        hand.ptr = 0;
        wasm.menager_add_hand(this.ptr, ptr0);
    }
    /**
    * @returns {Array<any> | undefined}
    */
    calculate() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.menager_calculate(ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any> | undefined}
    */
    calculate_test() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.menager_calculate_test(ptr);
        return takeObject(ret);
    }
}
/**
*/
export class Pot {

    static __wrap(ptr) {
        const obj = Object.create(Pot.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pot_free(ptr);
    }
    /**
    * @param {number} pot
    */
    constructor(pot) {
        const ret = wasm.pot_new(pot);
        return Pot.__wrap(ret);
    }
}
/**
*/
export class Total {

    static __wrap(ptr) {
        const obj = Object.create(Total.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_total_free(ptr);
    }
    /**
    */
    get combination() {
        const ret = wasm.__wbg_get_total_combination(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set combination(arg0) {
        wasm.__wbg_set_total_combination(this.ptr, arg0);
    }
    /**
    */
    get key_range_group() {
        const ret = wasm.__wbg_get_total_key_range_group(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set key_range_group(arg0) {
        wasm.__wbg_set_total_key_range_group(this.ptr, arg0);
    }
    /**
    * @returns {Array<any>}
    */
    get_cards() {
        const ret = wasm.total_get_cards(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    show_cards() {
        const ret = wasm.total_show_cards(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string}
    */
    show_combination() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.total_show_combination(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get_player_id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.total_get_player_id(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {number}
    */
    get_win_pot() {
        const ret = wasm.total_get_win_pot(this.ptr);
        return ret;
    }
}

export function __wbg_log_437df00577fb3353(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

export function __wbg_card_new(arg0) {
    const ret = Card.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbg_total_new(arg0) {
    const ret = Total.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_json_parse(arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_newwithlength_e80fb11cf19c1628(arg0) {
    const ret = new Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_set_561aac756158708c(arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
};

export function __wbg_new_693216e109162396() {
    const ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_error_09919627ac0992f5(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

