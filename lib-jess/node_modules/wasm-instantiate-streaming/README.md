# wasm-instantiate-streaming
![package-version](https://img.shields.io/npm/v/wasm-instantiate-streaming.svg)
![node-version](https://img.shields.io/node/v/wasm-instantiate-streaming.svg)
![CircleCI](https://circleci.com/gh/Leko/wasm-instantiate-streaming.svg?style=svg)
![codecov](https://codecov.io/gh/Leko/wasm-instantiate-streaming/branch/master/graph/badge.svg)
![license](https://img.shields.io/github/license/Leko/wasm-instantiate-streaming.svg)

Polyfill of WebAssembly.instantiateStreaming
`instantiateStreaming` loading WebAssembly modules efficiently

> The optimizations we applied can be summarized as follows:
>
> * Use asynchronous APIs to avoid blocking the main thread
> * Use streaming APIs to compile and instantiate WebAssembly modules more quickly
> * Don’t write code you don’t need
>
> &mdash; [Loading WebAssembly modules efficiently  |  Web  |  Google Developers](https://developers.google.com/web/updates/2018/04/loading-wasm)

## Install
```
npm install wasm-instantiate-streaming
```

## Usage
### Using import/require
```js
import { instantiateStreaming } from 'wasm-instantiate-streaming'

// For browser
instantiateStreaming(fetch('some-module.wasm'))

// Use with node-fetch
const fetch = require('node-fetch')
instantiateStreaming(fetch('some-module.wasm'))

// Use with fs module
const fs = require('fs')
const { promisify } = require('util')
const readFile = promisify(fs.readFile)
instantiateStreaming(readFile('./some-module.wasm'))
```

### Using without import/require
Please use `build/browser.min.js`

```html
<script src="path/to/build/browser.min.js">
```

```js
instantiateStreaming(fetch('some-module.wasm'))
```

### Pass importsObject
```js
const importObject = {
  imports: {
    imported_func (arg) {
      console.log(arg)
    }
  }
}

instantiateStreaming(fetch('some-module.wasm'), importObject)
```

## API
See MDN document

> &mdash; [WebAssembly.instantiateStreaming() - JavaScript | MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiateStreaming)

## Development
```sh
git clone git@github.com:Leko/wasm-instantiate-streaming.git
cd wasm-instantiate-streaming

git submodule update --init --recursive
cd deps/wabt
make
cd ../..

npm install
npm test
```

## License
[MIT](https://opensource.org/licenses/MIT)
