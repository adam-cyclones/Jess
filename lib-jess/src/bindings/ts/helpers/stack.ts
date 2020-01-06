interface iStack {
  _stack: any[];
  _stackSize: number;
  construct(size: number): void;
  create(stackLimit: number): iStack;
  push(value: any): number;
  pop(): number;
  clear(): void;
  length: number;
  value: any[]
}

const Stack: iStack = {
  _stack: [] as any[],
  _stackSize: 1 as number,
  construct(size: number) {
    this._stackSize = size;
  },
  create(stackLimit: number) {
    this.construct(stackLimit);
    const proto = Object.create(this);
    return proto;
  },
  push(value: any) {
    if (this._stack.length === this._stackSize) {
      this.pop();
    }
    this._stack.push(value);
    return this._stack.length;
  },
  pop() {
    delete this._stack[0];
    this._stack = this._stack.filter(Boolean);
    return this._stack.length;
  },
  clear() {
    this._stack.length = 0;
  },
  get length(): number {
    return this._stack.length;
  },
  get value(): any[] {
    return this._stack;
  }
}

export { Stack, iStack };