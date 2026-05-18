/** @module Interface wasi-gfx:webgpu-cts/cts-tests@0.1.0 **/
export function listSpecs(): Array<string>;
/**
 * TODO: consider changing error from `string` to `error-context`
 * Returns a list of tests. Each test can contain multiple cases
 */
export function runSpecTests(name: string): Array<TestResult>;
/**
 * # Variants
 * 
 * ## `"pass"`
 * 
 * ## `"skip"`
 * 
 * ## `"fail"`
 */
export type CaseStatus = 'pass' | 'skip' | 'fail';
export interface CaseLog {
  message: string,
  stack?: string,
}
export interface CaseResult {
  name: string,
  status: CaseStatus,
  logs: Array<CaseLog>,
}
export interface TestResult {
  name: string,
  cases: Array<CaseResult>,
}
