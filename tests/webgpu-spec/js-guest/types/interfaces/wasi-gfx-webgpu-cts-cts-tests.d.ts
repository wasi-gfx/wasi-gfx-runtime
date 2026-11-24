declare module 'wasi-gfx:webgpu-cts/cts-tests' {
  export function listSpecs(): Array<string>;
  /**
   * TODO: consider changing error from `string` to `error-context`
   * Returns a list of tests. Each test can contain multiple cases
   *
   * @throws {string} To return the error of the result, throw it, or an `Error` that has it as its `payload`.
   */
  export function runSpecTests(name: string): Promise<SpecResult>;
  export interface CaseLog {
    message: string,
    stack?: string,
  }
  /**
   * # Variants
   * 
   * ## `"passed"`
   * 
   * ## `"warned"`
   * 
   * ## `"failed"`
   * 
   * ## `"skipped"`
   */
  export type CaseResultStatus = "passed" | "warned" | "failed" | "skipped";
  export interface CaseResult {
    name: string,
    status: CaseResultStatus,
    log: CaseLog,
  }
  export interface TestResult {
    name: string,
    cases: Array<CaseResult>,
  }
  export interface SpecResult {
    tests: Array<TestResult>,
  }
}
