export const StringView = {
  new(...args) {
    this._construct(...args)
    return Object.create(this);
  },
  _construct (input) {
    this._enc = new TextEncoder();
    this._dec = new TextDecoder();
    this._uint8Array = this._enc.encode(input);
    this._stringLength = input.length;
    this._dataView = new DataView(this._uint8Array);
  },
  substr(from = 0, too = this._stringLength) {
    return this._dec.decode(this._dataView.buffer.slice(from, too));
  },
  toString() {
    return this.substr(0, this._stringLength);
  },
  toUint8Array() {
    return this._uint8Array
  },
  getCharAt(index) {
    return String.fromCharCode(this._dataView.getInt8(index));
  },
  get length() {
    return this._stringLength;
  }
}