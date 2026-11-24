/// <reference path="./interfaces/wasi-clocks-monotonic-clock.d.ts" />
/// <reference path="./interfaces/wasi-clocks-types.d.ts" />
/// <reference path="./interfaces/wasi-gfx-webgpu-cts-cts-tests.d.ts" />
/// <reference path="./interfaces/wasi-webgpu-webgpu.d.ts" />
declare module 'wasi-gfx:webgpu-cts/imports' {
  export type * as WasiClocksMonotonicClock030 from 'wasi:clocks/monotonic-clock@0.3.0'; // import wasi:clocks/monotonic-clock@0.3.0
  export type * as WasiClocksTypes030 from 'wasi:clocks/types@0.3.0'; // import wasi:clocks/types@0.3.0
  export type * as WasiWebgpuWebgpu030Rc2 from 'wasi:webgpu/webgpu@0.3.0-rc.2'; // import wasi:webgpu/webgpu@0.3.0-rc.2
  export * as ctsTests from 'wasi-gfx:webgpu-cts/cts-tests'; // export wasi-gfx:webgpu-cts/cts-tests
}
