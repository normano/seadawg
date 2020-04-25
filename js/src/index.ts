import { SeaDawgCore } from "./v2/core";
import { SeaValueSinkNode } from "./v2/data";

export class SeaDawgMap<Value> implements Map<string, Value> {

  private _core: SeaDawgCore;

  [Symbol.toStringTag]: string;
  size: number = 0;

  constructor() {
    this._core = new SeaDawgCore();
  }

  get(key: string): Value {
    
    const existingSink = <SeaValueSinkNode>this._core.findExact(key);

    if(!existingSink) {
      return null;
    }

    return existingSink.data;
  }

  has(key: string): boolean {
    
    const existingSink = <SeaValueSinkNode>this._core.findExact(key);

    return !!existingSink;
  }

  set(key: string, value: Value): this {

    const existingSink = <SeaValueSinkNode>this._core.findExact(key);

    if(!existingSink) {

      this._core.add(key, new SeaValueSinkNode(value));
      this.size++;

    } else {
      existingSink.data = value;
    }

    return this;
  }

  clear(): void {
    this._core = new SeaDawgCore();
  }

  delete(key: string): boolean {
    throw new Error("Method not implemented.");
  }

  forEach(callbackfn: (value: Value, key: string, map: Map<string, Value>) => void, thisArg?: any): void {
    throw new Error("Method not implemented.");
  }

  [Symbol.iterator](): IterableIterator<[string, Value]> {
    throw new Error("Method not implemented.");
  }

  entries(): IterableIterator<[string, Value]> {
    throw new Error("Method not implemented.");
  }

  keys(): IterableIterator<string> {
    throw new Error("Method not implemented.");
  }

  values(): IterableIterator<Value> {
    throw new Error("Method not implemented.");
  }
}