// QuickJS doesn't provide timers or `performance`, but the CTS relies on both.
// Must be imported before anything from the CTS, since it captures `setTimeout` at module evaluation.
import { now, waitFor } from "wasi:clocks/monotonic-clock@0.3.0";

type TimerHandle = number;

let nextTimerHandle: TimerHandle = 1;
const activeTimers = new Set<TimerHandle>();

function setTimeout(callback: (...args: any[]) => void, ms: number = 0, ...args: any[]): TimerHandle {
    const handle = nextTimerHandle++;
    activeTimers.add(handle);
    const nanos = BigInt(Math.max(0, Math.round(ms * 1_000_000)));
    waitFor(nanos).then(() => {
        if (activeTimers.delete(handle)) {
            callback(...args);
        }
    });
    return handle;
}

function clearTimeout(handle: TimerHandle | undefined) {
    if (handle !== undefined) {
        activeTimers.delete(handle);
    }
}

// Captured lazily: top-level code runs at build time (Wizer snapshot), where the clock has a different origin.
let startMark: bigint | undefined;
const performance = {
    now(): number {
        startMark ??= now();
        return Number(now() - startMark) / 1_000_000;
    },
};

Object.assign(globalThis, { setTimeout, clearTimeout, performance });
