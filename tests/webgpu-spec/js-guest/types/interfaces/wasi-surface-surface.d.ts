/** @module Interface wasi:surface/surface@0.0.1 **/
export type Context = import('./wasi-graphics-context-graphics-context.js').Context;
export type Pollable = import('./wasi-io-poll.js').Pollable;
export interface CreateDesc {
  height?: number,
  width?: number,
}
export interface ResizeEvent {
  height: number,
  width: number,
}
export interface FrameEvent {
  /**
   * TODO: This field doesn't mean anything.
   * Can't have empty record. Would like to have a way around this.
   */
  nothing: boolean,
}
export interface PointerEvent {
  x: number,
  y: number,
}
/**
 * corresponds with https://w3c.github.io/uievents-code/#code-value-tables
 * `Unidentified` is not included, use `option<key>` instead.
 * # Variants
 * 
 * ## `"backquote"`
 * 
 * ## `"backslash"`
 * 
 * ## `"bracket-left"`
 * 
 * ## `"bracket-right"`
 * 
 * ## `"comma"`
 * 
 * ## `"digit0"`
 * 
 * ## `"digit1"`
 * 
 * ## `"digit2"`
 * 
 * ## `"digit3"`
 * 
 * ## `"digit4"`
 * 
 * ## `"digit5"`
 * 
 * ## `"digit6"`
 * 
 * ## `"digit7"`
 * 
 * ## `"digit8"`
 * 
 * ## `"digit9"`
 * 
 * ## `"equal"`
 * 
 * ## `"intl-backslash"`
 * 
 * ## `"intl-ro"`
 * 
 * ## `"intl-yen"`
 * 
 * ## `"key-a"`
 * 
 * ## `"key-b"`
 * 
 * ## `"key-c"`
 * 
 * ## `"key-d"`
 * 
 * ## `"key-e"`
 * 
 * ## `"key-f"`
 * 
 * ## `"key-g"`
 * 
 * ## `"key-h"`
 * 
 * ## `"key-i"`
 * 
 * ## `"key-j"`
 * 
 * ## `"key-k"`
 * 
 * ## `"key-l"`
 * 
 * ## `"key-m"`
 * 
 * ## `"key-n"`
 * 
 * ## `"key-o"`
 * 
 * ## `"key-p"`
 * 
 * ## `"key-q"`
 * 
 * ## `"key-r"`
 * 
 * ## `"key-s"`
 * 
 * ## `"key-t"`
 * 
 * ## `"key-u"`
 * 
 * ## `"key-v"`
 * 
 * ## `"key-w"`
 * 
 * ## `"key-x"`
 * 
 * ## `"key-y"`
 * 
 * ## `"key-z"`
 * 
 * ## `"minus"`
 * 
 * ## `"period"`
 * 
 * ## `"quote"`
 * 
 * ## `"semicolon"`
 * 
 * ## `"slash"`
 * 
 * ## `"alt-left"`
 * 
 * ## `"alt-right"`
 * 
 * ## `"backspace"`
 * 
 * ## `"caps-lock"`
 * 
 * ## `"context-menu"`
 * 
 * ## `"control-left"`
 * 
 * ## `"control-right"`
 * 
 * ## `"enter"`
 * 
 * ## `"meta-left"`
 * 
 * ## `"meta-right"`
 * 
 * ## `"shift-left"`
 * 
 * ## `"shift-right"`
 * 
 * ## `"space"`
 * 
 * ## `"tab"`
 * 
 * ## `"convert"`
 * 
 * ## `"kana-mode"`
 * 
 * ## `"lang1"`
 * 
 * ## `"lang2"`
 * 
 * ## `"lang3"`
 * 
 * ## `"lang4"`
 * 
 * ## `"lang5"`
 * 
 * ## `"non-convert"`
 * 
 * ## `"delete"`
 * 
 * ## `"end"`
 * 
 * ## `"help"`
 * 
 * ## `"home"`
 * 
 * ## `"insert"`
 * 
 * ## `"page-down"`
 * 
 * ## `"page-up"`
 * 
 * ## `"arrow-down"`
 * 
 * ## `"arrow-left"`
 * 
 * ## `"arrow-right"`
 * 
 * ## `"arrow-up"`
 * 
 * ## `"num-lock"`
 * 
 * ## `"numpad0"`
 * 
 * ## `"numpad1"`
 * 
 * ## `"numpad2"`
 * 
 * ## `"numpad3"`
 * 
 * ## `"numpad4"`
 * 
 * ## `"numpad5"`
 * 
 * ## `"numpad6"`
 * 
 * ## `"numpad7"`
 * 
 * ## `"numpad8"`
 * 
 * ## `"numpad9"`
 * 
 * ## `"numpad-add"`
 * 
 * ## `"numpad-backspace"`
 * 
 * ## `"numpad-clear"`
 * 
 * ## `"numpad-clear-entry"`
 * 
 * ## `"numpad-comma"`
 * 
 * ## `"numpad-decimal"`
 * 
 * ## `"numpad-divide"`
 * 
 * ## `"numpad-enter"`
 * 
 * ## `"numpad-equal"`
 * 
 * ## `"numpad-hash"`
 * 
 * ## `"numpad-memory-add"`
 * 
 * ## `"numpad-memory-clear"`
 * 
 * ## `"numpad-memory-recall"`
 * 
 * ## `"numpad-memory-store"`
 * 
 * ## `"numpad-memory-subtract"`
 * 
 * ## `"numpad-multiply"`
 * 
 * ## `"numpad-paren-left"`
 * 
 * ## `"numpad-paren-right"`
 * 
 * ## `"numpad-star"`
 * 
 * ## `"numpad-subtract"`
 * 
 * ## `"escape"`
 * 
 * ## `"f1"`
 * 
 * ## `"f2"`
 * 
 * ## `"f3"`
 * 
 * ## `"f4"`
 * 
 * ## `"f5"`
 * 
 * ## `"f6"`
 * 
 * ## `"f7"`
 * 
 * ## `"f8"`
 * 
 * ## `"f9"`
 * 
 * ## `"f10"`
 * 
 * ## `"f11"`
 * 
 * ## `"f12"`
 * 
 * ## `"fn"`
 * 
 * ## `"fn-lock"`
 * 
 * ## `"print-screen"`
 * 
 * ## `"scroll-lock"`
 * 
 * ## `"pause"`
 * 
 * ## `"browser-back"`
 * 
 * ## `"browser-favorites"`
 * 
 * ## `"browser-forward"`
 * 
 * ## `"browser-home"`
 * 
 * ## `"browser-refresh"`
 * 
 * ## `"browser-search"`
 * 
 * ## `"browser-stop"`
 * 
 * ## `"eject"`
 * 
 * ## `"launch-app1"`
 * 
 * ## `"launch-app2"`
 * 
 * ## `"launch-mail"`
 * 
 * ## `"media-play-pause"`
 * 
 * ## `"media-select"`
 * 
 * ## `"media-stop"`
 * 
 * ## `"media-track-next"`
 * 
 * ## `"media-track-previous"`
 * 
 * ## `"power"`
 * 
 * ## `"sleep"`
 * 
 * ## `"audio-volume-down"`
 * 
 * ## `"audio-volume-mute"`
 * 
 * ## `"audio-volume-up"`
 * 
 * ## `"wake-up"`
 * 
 * ## `"hyper"`
 * 
 * ## `"super"`
 * 
 * ## `"turbo"`
 * 
 * ## `"abort"`
 * 
 * ## `"resume"`
 * 
 * ## `"suspend"`
 * 
 * ## `"again"`
 * 
 * ## `"copy"`
 * 
 * ## `"cut"`
 * 
 * ## `"find"`
 * 
 * ## `"open"`
 * 
 * ## `"paste"`
 * 
 * ## `"props"`
 * 
 * ## `"select"`
 * 
 * ## `"undo"`
 * 
 * ## `"hiragana"`
 * 
 * ## `"katakana"`
 */
export type Key = 'backquote' | 'backslash' | 'bracket-left' | 'bracket-right' | 'comma' | 'digit0' | 'digit1' | 'digit2' | 'digit3' | 'digit4' | 'digit5' | 'digit6' | 'digit7' | 'digit8' | 'digit9' | 'equal' | 'intl-backslash' | 'intl-ro' | 'intl-yen' | 'key-a' | 'key-b' | 'key-c' | 'key-d' | 'key-e' | 'key-f' | 'key-g' | 'key-h' | 'key-i' | 'key-j' | 'key-k' | 'key-l' | 'key-m' | 'key-n' | 'key-o' | 'key-p' | 'key-q' | 'key-r' | 'key-s' | 'key-t' | 'key-u' | 'key-v' | 'key-w' | 'key-x' | 'key-y' | 'key-z' | 'minus' | 'period' | 'quote' | 'semicolon' | 'slash' | 'alt-left' | 'alt-right' | 'backspace' | 'caps-lock' | 'context-menu' | 'control-left' | 'control-right' | 'enter' | 'meta-left' | 'meta-right' | 'shift-left' | 'shift-right' | 'space' | 'tab' | 'convert' | 'kana-mode' | 'lang1' | 'lang2' | 'lang3' | 'lang4' | 'lang5' | 'non-convert' | 'delete' | 'end' | 'help' | 'home' | 'insert' | 'page-down' | 'page-up' | 'arrow-down' | 'arrow-left' | 'arrow-right' | 'arrow-up' | 'num-lock' | 'numpad0' | 'numpad1' | 'numpad2' | 'numpad3' | 'numpad4' | 'numpad5' | 'numpad6' | 'numpad7' | 'numpad8' | 'numpad9' | 'numpad-add' | 'numpad-backspace' | 'numpad-clear' | 'numpad-clear-entry' | 'numpad-comma' | 'numpad-decimal' | 'numpad-divide' | 'numpad-enter' | 'numpad-equal' | 'numpad-hash' | 'numpad-memory-add' | 'numpad-memory-clear' | 'numpad-memory-recall' | 'numpad-memory-store' | 'numpad-memory-subtract' | 'numpad-multiply' | 'numpad-paren-left' | 'numpad-paren-right' | 'numpad-star' | 'numpad-subtract' | 'escape' | 'f1' | 'f2' | 'f3' | 'f4' | 'f5' | 'f6' | 'f7' | 'f8' | 'f9' | 'f10' | 'f11' | 'f12' | 'fn' | 'fn-lock' | 'print-screen' | 'scroll-lock' | 'pause' | 'browser-back' | 'browser-favorites' | 'browser-forward' | 'browser-home' | 'browser-refresh' | 'browser-search' | 'browser-stop' | 'eject' | 'launch-app1' | 'launch-app2' | 'launch-mail' | 'media-play-pause' | 'media-select' | 'media-stop' | 'media-track-next' | 'media-track-previous' | 'power' | 'sleep' | 'audio-volume-down' | 'audio-volume-mute' | 'audio-volume-up' | 'wake-up' | 'hyper' | 'super' | 'turbo' | 'abort' | 'resume' | 'suspend' | 'again' | 'copy' | 'cut' | 'find' | 'open' | 'paste' | 'props' | 'select' | 'undo' | 'hiragana' | 'katakana';
export interface KeyEvent {
  key?: Key,
  text?: string,
  altKey: boolean,
  ctrlKey: boolean,
  metaKey: boolean,
  shiftKey: boolean,
}

export class Surface {
  constructor(desc: CreateDesc)
  connectGraphicsContext(context: Context): void;
  height(): number;
  width(): number;
  requestSetSize(height: number | undefined, width: number | undefined): void;
  subscribeResize(): Pollable;
  getResize(): ResizeEvent | undefined;
  subscribeFrame(): Pollable;
  getFrame(): FrameEvent | undefined;
  subscribePointerUp(): Pollable;
  getPointerUp(): PointerEvent | undefined;
  subscribePointerDown(): Pollable;
  getPointerDown(): PointerEvent | undefined;
  subscribePointerMove(): Pollable;
  getPointerMove(): PointerEvent | undefined;
  subscribeKeyUp(): Pollable;
  getKeyUp(): KeyEvent | undefined;
  subscribeKeyDown(): Pollable;
  getKeyDown(): KeyEvent | undefined;
}
