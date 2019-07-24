/* tslint:disable */
/**
* @param {number} concurrency 
* @param {WorkerPool} pool 
*/
export function run_test(concurrency: number, pool: WorkerPool): void;
/**
* Entry point invoked by `worker.js`, a bit of a hack but see the \"TODO\" above
* about `worker.js` in general.
* @param {number} ptr 
*/
export function child_entry_point(ptr: number): void;
/**
*/
export class BBBB {
  free(): void;
/**
* @param {any} object 
* @returns {BBBB} 
*/
  constructor(object: any);
/**
*/
  static something(): void;
}
/**
*/
export class WorkerPool {
  free(): void;
/**
* Creates a new `WorkerPool` which immediately creates `initial` workers.
*
* The pool created here can be used over a long period of time, and it
* will be initially primed with `initial` workers. Currently workers are
* never released or gc\'d until the whole pool is destroyed.
*
* # Errors
*
* Returns any error that may happen while a JS web worker is created and a
* message is sent to it.
* @param {number} initial 
* @returns {WorkerPool} 
*/
  constructor(initial: number);
}

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<any>}
*/
export default function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module, maybe_memory: WebAssembly.Memory): Promise<any>;
        