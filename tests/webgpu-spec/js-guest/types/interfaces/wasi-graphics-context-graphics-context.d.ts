/** @module Interface wasi:graphics-context/graphics-context@0.0.1 **/

export class AbstractBuffer {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
}

export class Context {
  constructor()
  getCurrentBuffer(): AbstractBuffer;
  /**
  * TODO: might want to remove this.
  */
  present(): void;
}
