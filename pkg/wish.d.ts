/* tslint:disable */
/* eslint-disable */
/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
  /**
   * Both vertically and horizontally subsampled.
   */
  Cs420 = 0,
  /**
   * Horizontally subsampled.
   */
  Cs422 = 1,
  /**
   * Not subsampled.
   */
  Cs444 = 2,
  /**
   * Monochrome.
   */
  Cs400 = 3,
}
export class WebHandle {
  free(): void;
  /**
   * @returns {boolean}
   */
  has_panicked(): boolean;
  /**
   * @returns {string | undefined}
   */
  panic_message(): string | undefined;
  /**
   * @returns {string | undefined}
   */
  panic_callstack(): string | undefined;
  constructor();
  /**
   * @param {HTMLCanvasElement} canvas
   * @returns {Promise<void>}
   */
  start(canvas: HTMLCanvasElement): Promise<void>;
  destroy(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_webhandle_free: (a: number, b: number) => void;
  readonly webhandle_destroy: (a: number) => void;
  readonly webhandle_has_panicked: (a: number) => number;
  readonly webhandle_new: () => number;
  readonly webhandle_panic_callstack: (a: number) => Array;
  readonly webhandle_panic_message: (a: number) => Array;
  readonly webhandle_start: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly closure1146_externref_shim: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__had1c479c4aa8aa1f_multivalue_shim: (a: number, b: number) => Array;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure1163_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly closure1502_externref_shim: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
