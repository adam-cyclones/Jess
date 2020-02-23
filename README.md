<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/adam-cyclones/Jess">
    We don't have a logo yet, can you help?
  </a>

  <h3 align="center">Jess</h3>

  <p align="center">
    What do you get if you smash JavaScript and CSS together and give CSS the spotlight? Jess is the answer to CSS in JS by reversing the paradigm.
    <br />
    <a href="https://github.com/adam-cyclones/Jess/blob/master/samples/simple.jss"><strong>Take a peek</strong></a>
    <br />
    <br />
    <a href="https://github.com/adam-cyclones/Jess/issues">Report Bug</a>
    |
    <a href="https://github.com/adam-cyclones/Jess/issues">Request Feature</a>
    |
    <a href="https://dev.to/adam_cyclones/jess-language-design-2mag">News @ Dev.to</a>
  </p>
</p>


### Building from source
Jess comprises of two primary technologies, Rust and Typescript, although we try to provide a portable tool-chain with less to install, there are one or two things you will need to download. These steps have been tested on a fresh MacBook (2019). It is unknown if windows or linux OS's will be able to compile, PR's welcome but when I am able I will try to test more dev environments.

Special notes, Rust is constantly moving, it is safe to assume that at some point soon a new version in stable will release which will not require nightly and so the following Rust instructions are not set in stone.

### Requirements

#### Misc
- Connection to the internet.
- install all npm sub-module dependencies

#### Rust
- Rustup 1.21.1 (2020-01-08) or above.
- Rustc using [nightly chanel](https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml "O'reilly article about installing the rust nightly channel")
- Target wasm32-unknown-unknown `rustup target add wasm32-unknown-unknown`
- wasm-bindgen `cargo install wasm-bindgen-cli`

#### Node
- v12.14.1 - Jess will always be developed on (LTS)




<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Your Name - [@your_twitter](https://twitter.com/your_username) - email@example.com

Project Link: [https://github.com/your_username/repo_name](https://github.com/your_username/repo_name)



<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Dev.to](https://dev.to/)