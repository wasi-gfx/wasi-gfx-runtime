import { specs } from "./spec-list";
import { runSpecTests } from "./run-single-spec";
import type { TestResult } from "../../types/interfaces/wasi-gfx-js-webgpu-cts-tests";
import { TestGroup, TestGroupBuilder } from "./cts/src/common/internal/test_group";
import { Fixture } from "./cts/src/common/framework/fixture";

export const ctsTests = {
    listSpecs(): string[] {
        return Object.keys(specs);
    },

    async runSpecTests(specName: string): Promise<TestResult[]> {
        try {
            const spec = (specs as Record<string, TestGroupBuilder<Fixture>>)[specName];
            if (!spec) {
                throw new Error(`Unknown spec: ${specName}`);
            }
            return runSpecTests(spec as TestGroup<Fixture>);
        } catch (e) {
            if (e === undefined || e === null)
                throw "";
            throw (e as Error).toString();
        }
    },
};
