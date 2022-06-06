let wasm_bindgen;
(function() {
    const __exports = {};
    let wasm;

    const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().slice(ptr, ptr + len));
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

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
};

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
__exports.Combination = Object.freeze({ RoyalFlush:9,"9":"RoyalFlush",StraightFlush:8,"8":"StraightFlush",FourOfKind:7,"7":"FourOfKind",FullHouse:6,"6":"FullHouse",Flush:5,"5":"Flush",Straight:4,"4":"Straight",ThreeOfKind:3,"3":"ThreeOfKind",TwoPairs:2,"2":"TwoPairs",Pair:1,"1":"Pair",HighCard:0,"0":"HighCard", });
/**
*/
__exports.M = Object.freeze({ S:1,"1":"S",H:10,"10":"H",D:100,"100":"D",C:1000,"1000":"C", });
/**
*/
__exports.N = Object.freeze({ TWO:1,"1":"TWO",THREE:2,"2":"THREE",FOUR:4,"4":"FOUR",FIVE:8,"8":"FIVE",SIX:16,"16":"SIX",SEVEN:32,"32":"SEVEN",EIGHT:64,"64":"EIGHT",NINE:128,"128":"NINE",TEN:256,"256":"TEN",J:512,"512":"J",Q:1024,"1024":"Q",K:2048,"2048":"K",A:4096,"4096":"A", });
/**
*/
class Card {

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
}
__exports.Card = Card;
/**
*/
class FullCombination {

    static __wrap(ptr) {
        const obj = Object.create(FullCombination.prototype);
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
        wasm.__wbg_fullcombination_free(ptr);
    }
    /**
    */
    get combination() {
        const ret = wasm.__wbg_get_fullcombination_combination(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set combination(arg0) {
        wasm.__wbg_set_fullcombination_combination(this.ptr, arg0);
    }
    /**
    */
    get key_range() {
        const ret = wasm.__wbg_get_fullcombination_key_range(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set key_range(arg0) {
        wasm.__wbg_set_fullcombination_key_range(this.ptr, arg0);
    }
    /**
    * @returns {Array<any>}
    */
    get_cards() {
        const ret = wasm.fullcombination_get_cards(this.ptr);
        return takeObject(ret);
    }
}
__exports.FullCombination = FullCombination;
/**
*/
class Hand {

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
    * @param {string} key
    * @param {Card} c1
    * @param {Card} c2
    * @param {Card} c3
    * @param {Card} c4
    * @param {Card} c5
    * @param {Card} c6
    * @param {Card} c7
    */
    constructor(key, c1, c2, c3, c4, c5, c6, c7) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
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
        const ret = wasm.hand_new(ptr0, len0, ptr1, ptr2, ptr3, ptr4, ptr5, ptr6, ptr7);
        return Hand.__wrap(ret);
    }
}
__exports.Hand = Hand;
/**
*/
class Menager {

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
    */
    constructor() {
        const ret = wasm.menager_new();
        return Menager.__wrap(ret);
    }
    /**
    * @param {Hand} hand
    */
    add(hand) {
        _assertClass(hand, Hand);
        var ptr0 = hand.ptr;
        hand.ptr = 0;
        wasm.menager_add(this.ptr, ptr0);
    }
    /**
    * @returns {Array<any> | undefined}
    */
    calculate_wasm() {
        const ret = wasm.menager_calculate_wasm(this.ptr);
        return takeObject(ret);
    }
}
__exports.Menager = Menager;

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input, maybe_memory) {
    if (typeof input === 'undefined') {
        let src;
        if (typeof document === 'undefined') {
            src = location.href;
        } else {
            src = document.currentScript.src;
        }
        input = src.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_card_new = function(arg0) {
        const ret = Card.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_fullcombination_new = function(arg0) {
        const ret = FullCombination.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithlength_e80fb11cf19c1628 = function(arg0) {
        const ret = new Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_561aac756158708c = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    imports.wbg.memory = maybe_memory || new WebAssembly.Memory({initial:18,maximum:16384,shared:true});

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

wasm_bindgen = Object.assign(init, __exports);

})();
