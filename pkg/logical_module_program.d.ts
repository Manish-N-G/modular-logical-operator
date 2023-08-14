/* tslint:disable */
/* eslint-disable */
/**
*/
export function main_logic_function(): void;
/**
* @param {string} input
* @returns {string}
*/
export function enter_input(input: string): string;
/**
* @param {string} input
* @returns {boolean}
*/
export function input_expression_client(input: string): boolean;
/**
* @param {string} input
* @returns {number}
*/
export function count_my_input_param_js(input: string): number;
/**
* @param {string} input_str
* @param {Float64Array} array
* @returns {Array<any>}
*/
export function assign_and_create_param_hash_js(input_str: string, array: Float64Array): Array<any>;
/**
* @returns {Array<any>}
*/
export function test_new(): Array<any>;
/**
* @param {string} input
* @returns {string}
*/
export function remove_whitespace(input: string): string;
/**
* @param {boolean} confirm
* @param {string} expression
* @returns {string}
*/
export function check_expression(confirm: boolean, expression: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main_logic_function: () => void;
  readonly enter_input: (a: number, b: number, c: number) => void;
  readonly input_expression_client: (a: number, b: number) => number;
  readonly count_my_input_param_js: (a: number, b: number) => number;
  readonly assign_and_create_param_hash_js: (a: number, b: number, c: number, d: number) => number;
  readonly test_new: () => number;
  readonly remove_whitespace: (a: number, b: number, c: number) => void;
  readonly check_expression: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
