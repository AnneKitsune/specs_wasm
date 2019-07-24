# Specs Wasm Usage Example

This simple example shows the (almost) minimum amount of code currently required to run specs on wasm (in the browser!).

### Prerequisites

* Running linux with the standard development tools available for your distribution.
* Rust installed

### Getting Started

Clone this repository:

```
git clone https://github.com/jojolepro/specs_wasm
```

Make sure that rust nightly is installed and up to date:

```
rustup install nightly
rustup update
```

Make nightly the default toolchain:

```
rustup default nightly
```

Add the rust-src component:
```
rustup component add rust-src
```

Install the wasm-bindgen cli tools:

```
cargo install -f wasm-bindgen-cli
```

Install Xargo

```
cargo install -f xargo
```

And run the build.sh script!
```
./build.sh
```

Open up firefox **nightly** and navigate to about:config

In there, you want to enable the `javascript.options.shared_memory` flag.

Once this is done, navigate to http://127.0.0.1:8000 , open the browser console and click the "Run!" button at the top of the page.

