/** @module Interface wasi:frame-buffer/frame-buffer@0.0.1 **/
export type Context = import('./wasi-graphics-context-graphics-context.js').Context;
export type AbstractBuffer = import('./wasi-graphics-context-graphics-context.js').AbstractBuffer;

export class Buffer {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static fromGraphicsBuffer(buffer: AbstractBuffer): Buffer;
  /**
  * TODO: This should be replcated with something that doesn't require a copy.
  */
  get(): Uint8Array;
  set(val: Uint8Array): void;
}

export class Device {
  constructor()
  connectGraphicsContext(context: Context): void;
}
