import "./polyfills";
import { specs } from "./spec-list";
import { runSpecTests } from "./run-single-spec";
import type { SpecResult, listSpecs as listSpecsType, runSpecTests as runSpecTestsType } from "wasi-gfx:webgpu-cts/cts-tests";
import { TestGroup, TestGroupBuilder } from "./cts/src/common/internal/test_group";
import { Fixture } from "./cts/src/common/framework/fixture";

interface CtsInterface {
    listSpecs: typeof listSpecsType,
    runSpecTests: typeof runSpecTestsType,
}

export const ctsTests: CtsInterface = {
    listSpecs(): string[] {
        return Object.keys(specs);
    },

    async runSpecTests(specName: string): Promise<SpecResult> {
        try {
            const spec = (specs as Record<string, TestGroupBuilder<Fixture>>)[specName];
            if (!spec) {
                throw new Error(`Unknown spec: ${specName}`);
            }
            return await runSpecTests(spec as TestGroup<Fixture>);
        } catch (e) {
        if (e === undefined || e === null)
                throw "";
            throw (e as Error).toString();
        }
    },
};
