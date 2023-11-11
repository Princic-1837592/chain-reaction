/* tslint:disable */
/* eslint-disable */
/**
* @param {number} height
* @param {number} width
* @param {number} players
* @returns {boolean}
*/
export function newGame(height: number, width: number, players: number): boolean;
/**
* @param {number} row
* @param {number} column
* @returns {string | undefined}
*/
export function addAtom(row: number, column: number): string | undefined;
/**
* @returns {string}
*/
export function getState(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly newGame: (a: number, b: number, c: number) => number;
  readonly addAtom: (a: number, b: number, c: number) => void;
  readonly getState: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
