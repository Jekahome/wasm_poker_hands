 

const { Menager,Hand,Card,N,M} = wasm_bindgen;

// First up, but try to do feature detection to provide better error messages
function loadWasm() {
  /*
  let msg = 'This demo requires a current version of Firefox (e.g., 79.0)';
  if (typeof SharedArrayBuffer !== 'function') {
    alert('this browser does not have SharedArrayBuffer support enabled' + '\n\n' + msg);
    return
  }
  // Test for bulk memory operations with passive data segments
  //  (module (memory 1) (data passive ""))
  const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
  if (!WebAssembly.validate(buf)) {
    alert('this browser does not support passive wasm memory, demo does not work' + '\n\n' + msg);
    return
  }
*/
  wasm_bindgen('./poker_hands_bg.wasm')
    .then(run)
    .catch(console.error);
}

loadWasm();

function run(){
     let c1 = new Card(1,100);
     let c2 = new Card(2,100);
     let c3 = new Card(4,100);
     let c4 = new Card(8,100);
     let c5 = new Card(16,100);
     let c6 = new Card(2048,10);
     let c7 = new Card(4096,10);
     let hand = new Hand("flash1",c1,c2,c3,c4,c5,c6,c7);
    // let arr = new Array();
     //arr.push(hand);
     let manager = new Menager();
     manager.add(hand);
     let res = manager.calculate_wasm();
     for(var i=0; i<res.length; i++){
      console.log(res[i].key_range,res[i].combination);
      for (let c of res[i].get_cards()){
        let card = c.get();
        console.log("Card:",card.n,card.m);
      }
     }
}
 