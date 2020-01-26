### Building from source
Jess comprisies of two primary technologies, Rust and Typescript, although we try to provide a portable toolchain with less to install, there are one or two things you will need to download. These steps have been tested on a fresh MacBook (2019). It is unknown if windows or linux OS's will be able to compile, PR's welcome but when I am able I will try to test more dev environments.

Special notes, Rust is constantly moving, it is safe to asssume that at some point soon a new version in stable will release which will not require nightly and so the folowing Rust instructions are not set in stone.

#### Misc
- Connection to the internet.

#### Rust
- Rustup 1.21.1 (2020-01-08) or above.
- Rustc using [nightly chanel](https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml "O'reilly article about installing the rust nightly channel")
- target wasm32-unknown-unknown `rustup target add wasm32-unknown-unknown`
- wasm-bindgen `cargo install wasm-bindgen-cli`