const util = require("./util");

function ponyfill() {
  if (typeof WebAssembly === "undefined") {
    return function instantiateStreaming() {
      return Promise.reject(new Error("WebAssembly is not supported"));
    };
  }
  if (WebAssembly.instantiateStreaming) {
    return WebAssembly.instantiateStreaming.bind(WebAssembly);
  }

  /**
   * Ponyfill of WebAssembly.instantiateStreaming
   *
   * @param Promise<Response> source
   * @param Object [importObject={}]
   * @return Promise<{ module: WebAssembly.Module, instance: WebAssembly.Instance }> A Promise that resolves to a ResultObject which contains two fields
   * @see https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiateStreaming
   */
  function instantiateStreaming(source, importObject) {
    importObject = importObject || {};
    return source
      .then(util.toArrayBuffer)
      .then(util.preferCompile)
      .then(mod => {
        // eslint-disable-next-line promise/no-nesting
        return util.preferInstantiate(mod, importObject).then(instance => {
          return {
            module: mod,
            instance: instance
          };
        });
      });
  }

  return instantiateStreaming;
}

module.exports = {
  instantiateStreaming: ponyfill()
};
