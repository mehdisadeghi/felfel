# felfel
A Rust library and CLI tool for Farsi compound noun generation.

This is a little piece of code that generates compound names in Farsi.
Useful for naming things or anonymous visitors, or perhaps just for fun.

## Usage
_felfel_ is available both as a library as well as a little CLI tool.

### Use as a CLI tool

```bash
# Install it
$ cargo install felfel

# Use it in command line
$ felfel --> hopefully will make you smile
```

### Use as a library
Add this to your `Cargo.toml`:

```toml
[dependencies]
felfel = 0.1
```

Then invoke the `gen` or `gen_id` function:

```rust
use felfel;

fn main() {
	println!(felfel::gen());
}
```

### Use as a WebAssembly package
_felfel_ is compiled to Wasm and published to [npm](https://www.npmjs.com/package/felfel) as a
JavaScript package. First add it to your `package.json`:

```bash
# Add it to your program
$ npm install felfel
```

Then invoke the `gen` or `gen_id` function:

```JavaScript
import * as felfel from "felfel";
console.log(felfel.gen_id()); // -> nahange-shaer-612
````

You can also use the output of `wasm-pack build --target web` and upload it somewhere. See [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html) book for more info.

## Development
The only notable command worth mentioning is the generation of npm package:

```bash
# install wasm-pack
$ cargo install wasm-pack

# build the npm package
$ wasm-pack build --target nodejs

# publish the package
$ wasm-pack publish
```

## License
_felfel_ is distributed under MIT license.
