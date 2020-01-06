"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const path_1 = require("path");
function capitalize(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}
exports.capitalize = capitalize;
function getLangCode() {
    let lang;
    const LANG = process.env.LANG;
    if (LANG) {
        lang = LANG
            .split('.')[0]
            .replace('_', '-');
    }
    else {
        lang = 'en-US';
    }
    return lang;
}
exports.getLangCode = getLangCode;
function isRelativePath(p) {
    return !(path_1.resolve(p) === path_1.normalize(p).replace(RegExp(path_1.sep + '$'), ''));
}
exports.isRelativePath = isRelativePath;
