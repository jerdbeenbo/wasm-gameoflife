/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const wasm_bridge_init: () => void;
export const get_current_state: () => [number, number, number];
export const add_cell: (a: number, b: number) => [number, number];
export const wasm_bridge_update: () => [number, number, number];
export const __wbindgen_export_0: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_start: () => void;
