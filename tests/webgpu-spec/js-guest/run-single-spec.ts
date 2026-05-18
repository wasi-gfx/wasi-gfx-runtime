import { Fixture } from "./cts/src/common/framework/fixture";
import { Logger } from "./cts/src/common/internal/logging/logger";
import { LiveTestCaseResult } from "./cts/src/common/internal/logging/result";
import { parseQuery } from "./cts/src/common/internal/query/parseQuery";
import { TestQuerySingleCase } from "./cts/src/common/internal/query/query";
import { stringifyPublicParamsUniquely } from "./cts/src/common/internal/query/stringify_params";
import { TestGroup, RunCase, IterableTest } from "./cts/src/common/internal/test_group";

import { declareGlobals } from "@wasi-gfx/js-webgpu/globals";
import type { CaseLog, CaseResult, CaseStatus, TestResult } from "./types/interfaces/wasi-gfx-webgpu-cts-cts-tests";

const log = new Logger();
const query = "webgpu:placeholder:placeholder:*";
const webgpuQuery = parseQuery(query);

let initialized = false;
function ensureInit() {
    if (!initialized) {
        declareGlobals();
        initialized = true;
    }
}

function testName(c: IterableTest): string {
    return c.testPath.join(',');
}

function caseName(test: RunCase): string {
    return `${test.id.test.join(',')}:${stringifyPublicParamsUniquely(test.id.params)}`;
}

export async function runSpecTests(spec: TestGroup<Fixture>): Promise<TestResult[]> {
    ensureInit();
    const testResults: TestResult[] = [];

    for (const test of spec.iterate()) {
        const name = testName(test);
        const casesResults: CaseResult[] = [];
        for(const case_ of test.iterate(null)) {
            const name = caseName(case_);
            const [rec, res] = log.record(name);
            await case_.run(rec, webgpuQuery as TestQuerySingleCase, []);
            casesResults.push(buildCaseResult(name, res));
        }
        testResults.push({
            name,
            cases: casesResults,
        });
    }

    return testResults;
}

function buildCaseResult(name: string, result: LiveTestCaseResult): CaseResult {
    const logs: CaseLog[] = (result.logs ?? []).map(l => {
        const rawData = l.toRawData();
        let message = `${rawData.name}\n\n${rawData.message}`;
        if (rawData.extra) {
            message += `\n\n${rawData.extra}`;
        }
        return {
            message,
            stack: rawData.stack,
        }
    });
    let status: CaseStatus;
    switch (result.status) {
        case 'fail':
            status = "fail";
            break;
        case 'skip':
            let reason = (result.logs ?? []).reduce((ac, l) => `${ac}\n${l}`, "");
            status = "skip";
            break;
        case 'pass':
            status = "pass";
            break;
        default:
            throw new Error(name + "\n" + JSON.stringify(result));
    }
    return { name, status, logs };
}
