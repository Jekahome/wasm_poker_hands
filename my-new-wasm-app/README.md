<div align="center">

  <h1><code>create-wasm-app</code></h1>

  <strong>An <code>npm init</code> template for kick starting a project that uses NPM packages containing Rust-generated WebAssembly and bundles them with Webpack.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/create-wasm-app"><img src="https://img.shields.io/travis/rustwasm/create-wasm-app.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="#usage">Usage</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## Help

```
Run test
$ cargo test -- --nocapture

```

## About

This template is designed for depending on NPM packages that contain
Rust-generated WebAssembly and using them to create a Website.

* Want to create an NPM package with Rust and WebAssembly? [Check out
  `wasm-pack-template`.](https://github.com/rustwasm/wasm-pack-template)
* Want to make a monorepo-style Website without publishing to NPM? Check out
  [`rust-webpack-template`](https://github.com/rustwasm/rust-webpack-template)
  and/or
  [`rust-parcel-template`](https://github.com/rustwasm/rust-parcel-template).

## 🚴 Usage

Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

wasm-pack: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

```
npm init wasm-app

or

npm init wasm-app my-new-wasm-app
npm install
npm start
```

## 🔋 Batteries Included

- `.gitignore`: ignores `node_modules`
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you
- `README.md`: the file you are reading now!
- `index.html`: a bare bones html document that includes the webpack bundle
- `index.js`: example js file with a comment showing how to import and use a wasm pkg
- `package.json` and `package-lock.json`:
  - pulls in devDependencies for using webpack:
      - [`webpack`](https://www.npmjs.com/package/webpack)
      - [`webpack-cli`](https://www.npmjs.com/package/webpack-cli)
      - [`webpack-dev-server`](https://www.npmjs.com/package/webpack-dev-server)
  - defines a `start` script to run `webpack-dev-server`
- `webpack.config.js`: configuration file for bundling your js with webpack

## Use in NEXT.js

```
1. copy folder pkg in root project

2. Setting next.config.js

    module.exports = {
      experimental: { images: { layoutRaw: true } },
      webpack(config) {
        config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm'
        config.experiments = { asyncWebAssembly: true }
        return config
      },
    }

 3. Import in class React


 export default class YourComponent extends React.Component {
   mod_wasm;
   componentDidMount() {
      // после того, как компонент отрендерился в DOM 
      (async () => {  
         // Dynamically load
         this.mod_wasm = (await import('../../pkg/'))
       }).bind(this)();
    }
    ....
    handleChange(e){
         let c1 = new this.mod_wasm.Card(1,100);
         let c2 = new this.mod_wasm.Card(2,100);
         let c3 = new this.mod_wasm.Card(4,100);
         let c4 = new this.mod_wasm.Card(8,100);
         let c5 = new this.mod_wasm.Card(16,100);
         let c6 = new this.mod_wasm.Card(2048,10);
         let c7 = new this.mod_wasm.Card(4096,10);
         let hand = new this.mod_wasm.Hand("flash1",c1,c2,c3,c4,c5,c6,c7);
      
         let manager = new this.mod_wasm.Menager();
         manager.add(hand);
         let res = manager.calculate_wasm();
         for(var i=0; i<res.length; i++){
            console.log(res[i].key_range,res[i].combination);
            for (let c of res[i].get_cards()){
               let card = c.get();
               console.log(">>>Card<<<:",card.n,card.m);
            }
         }
    }
  }


```



## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
