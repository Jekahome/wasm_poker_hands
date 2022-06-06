import { Menager,Hand,Card}  from "poker_hands";

 
function run(){
    let c1 = new Card(1,100);
    let c2 = new Card(2,100);
    let c3 = new Card(4,100);
    let c4 = new Card(8,100);
    let c5 = new Card(16,100);
    let c6 = new Card(2048,10);
    let c7 = new Card(4096,10);
    let hand = new Hand("flash1",c1,c2,c3,c4,c5,c6,c7);
   
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

run();run();