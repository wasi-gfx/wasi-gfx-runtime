import { Fixture } from "./cts/src/common/framework/fixture";
import { Logger } from "./cts/src/common/internal/logging/logger";
import { LiveTestCaseResult } from "./cts/src/common/internal/logging/result";
import { parseQuery } from "./cts/src/common/internal/query/parseQuery";
import { TestQuerySingleCase } from "./cts/src/common/internal/query/query";
import { stringifyPublicParamsUniquely } from "./cts/src/common/internal/query/stringify_params";
import { TestGroup, RunCase, IterableTest } from "./cts/src/common/internal/test_group";

import { declareGlobals } from "@wasi-gfx/js-webgpu/globals";
import type { CaseLog, CaseResult, CaseResultStatus, SpecResult, TestResult } from "wasi-gfx:webgpu-cts/cts-tests";

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

export async function runSpecTests(spec: TestGroup<Fixture>): Promise<SpecResult> {
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

    return {
        tests: testResults
    };
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
    const log: CaseLog = {
        message: logs.map(l => l.message).join('\n\n'),
        stack: logs.map(l => l.stack).filter(s => s !== undefined).join('\n\n') || undefined,
    };
    let status: CaseResultStatus;
    switch (result.status) {
        case 'fail':
            status = "failed";
            break;
        case 'skip':
            status = "skipped";
            break;
        case 'pass':
            status = "passed";
            break;
        case 'warn':
            status = "warned";
            break;
        default:
            throw new Error(name + "\n" + JSON.stringify(result));
    }
    return { name, status, log };
}
