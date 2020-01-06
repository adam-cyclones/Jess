// @ts-ignore
import * as wasm32 from '../target/wasm32-bindgen/jess';
import { JESS_EXTNAME, TESS_EXTNAME } from './constants';
import { promises } from 'fs';
const { readFile } = promises;

import {
  iJessMethodsFrontend, iOpts
} from './bindings/ts/types/methods.interface';

export const compiler =  (async () => {
  // expose to public
  return <iJessMethodsFrontend> {
    /**
     * Compiles a Jess sourse file and returns the output/s
     * @param {path} - path to file.
    */
    async compileFile(path, opts: iOpts = {}) {
      // assign default options
      Object.assign(opts, {
        tess: false
      } as iOpts);

      const file = await readFile(path, 'utf8');
      return wasm32.compile(file);
    }
  }
})();

export {
  JESS_EXTNAME,
  TESS_EXTNAME
}