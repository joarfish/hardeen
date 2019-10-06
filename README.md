# 🎩 Hardeen

Hardeen is a library intended to allow for node-based procedural modelling and animation of 2d vector graphics. It's written in 🦀 Rust. There is a web-frontend that provides basic editing capabilities. This is powered by a WebAssembly wrapper around the Hardeen Core library.

## The Idea

Procdural modelling techniques, non-destructive/node-based editing are getting more and more attention in recent years. This has been limited to 3d software packets. The idea behind Hardeen is to explore whether similar approaches can lead to interesting results for the creation and animation of 2d vector graphics.

## Give it a try

You can use the web-frontend at https://jonasklein.dev/hardeen or build it yourself: Clone the whole repository and run `npm run dev` in the hardeen_webeditor folder to spin up a local server.

## Contributing

The project is at a very early stage. If you'd like to contribute or if you have ideas / criticism / feedback, don't hesitate to contact me.

## Built with

- [im-rs](https://docs.rs/crate/im/13.0.0)
- [Storm React-Diagrams](https://github.com/projectstorm/react-diagrams)
- [wasm-bingen](https://github.com/rustwasm/wasm-bindgen)

## License

This project is licensed under the GPLv3 License - see the LICENSE file for details
