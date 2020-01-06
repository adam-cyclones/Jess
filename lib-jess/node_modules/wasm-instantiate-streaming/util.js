function toUint8Array(buff) {
  var u = new Uint8Array(buff.length);
  for (var i = 0; i < buff.length; ++i) {
    u[i] = buff[i];
  }
  return u;
}

function toArrayBuffer(response) {
  // browser, node-fetch
  if (typeof response.arrayBuffer === "function") {
    return response.arrayBuffer();

    // Node.js
  } else {
    return Promise.resolve(toUint8Array(response));
  }
}

function preferCompile(arrayBuffer) {
  if (WebAssembly.compile) {
    return WebAssembly.compile(arrayBuffer);
  } else {
    return Promise.resolve(new WebAssembly.Module(arrayBuffer));
  }
}

function preferInstantiate(wasmModule, importObject) {
  if (WebAssembly.compile) {
    return WebAssembly.instantiate(wasmModule, importObject);
  } else {
    return Promise.resolve(new WebAssembly.Module(wasmModule));
  }
}

module.exports = {
  toUint8Array: toUint8Array,
  toArrayBuffer: toArrayBuffer,
  preferCompile: preferCompile,
  preferInstantiate: preferInstantiate
};
