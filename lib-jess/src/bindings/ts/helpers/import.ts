import { parseCurlyBraces, curlyBraceStack } from './braceParser';
import { readFileSync } from 'fs';
import { JESS_EXTNAME, TESS_EXTNAME } from '../../../constants';
import { iOpts } from '../types/methods.interface';
import { resolve } from 'path';

const encoding = 'utf8';

/**
 * Appends the Tess file extname if not existing in path
*/
export function import_jess_sync(path: string) {
  return readFileSync(path.endsWith(JESS_EXTNAME, 0) ? path : `${path}${JESS_EXTNAME}`, { encoding });
}

/**
 * Appends the Tess file extname if not existing in path
*/
export function import_tess_sync(path: string) {
  return readFileSync(path.endsWith(TESS_EXTNAME, 0) ? path : `${path}${TESS_EXTNAME}`, { encoding });
}