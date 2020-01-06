import { resolve, normalize, sep } from "path";

export function capitalize(str: string) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

export function getLangCode() {
  let lang!: string;
  const LANG = process.env.LANG;
  if (LANG) {
    lang = LANG
      .split('.')[0]
      .replace('_', '-')
  } else {
    lang = 'en-US';
  }
  return lang;
}

export function isRelativePath(p: string) {
  return !(resolve(p) === normalize(p).replace( RegExp(sep+'$'), '' ));
}