export interface iOpts {
  tess?: boolean;
}

export interface iJessMethodsFrontend {
  compileFile(p: string, opts?: iOpts): void;
}

export interface iJessMethodsWasm {
  compile(s: Uint8Array, opts?: iOpts): void;
}