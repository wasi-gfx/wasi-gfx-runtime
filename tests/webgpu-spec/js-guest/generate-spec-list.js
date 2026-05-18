import fs from "fs";
import path from "path";

main();

async function main() {
    const testFiles = await getTestFiles('./cts/src/webgpu');
    const result = generateTS(testFiles);
    await fs.promises.writeFile('./spec-list.ts', result);
}

async function getTestFiles(dir, out = []) {
    for await (const entry of await fs.promises.opendir(dir)) {
        const entryPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            await getTestFiles(entryPath, out);
        } else if (entry.isFile() && entry.name.endsWith(".spec.ts")) {
            // review:  this still needed??
            out.push(entryPath.replace("", ""));
        }
    }
    return out;
}

function generateTS(testFiles) {
    let imports = "";
    let exports = [];
    for (const file of testFiles) {
        if (file.endsWith("worker.spec.ts")) {
            // requires dynamic imports which are not yet supported in ComponentizeJS
            continue;
        }
        const name = file.replace("cts/src/webgpu/", "").replace(".spec.ts", "").replaceAll("/", "_");
        imports += `import { g as ${name} } from './${file.replace(".ts", "")}';\n`;
        exports.push(name);
    }
    const result = imports + "\n\n" + "export const specs = {\n    " + exports.join(",\n    ") + "\n};\n";
    return result;
}
