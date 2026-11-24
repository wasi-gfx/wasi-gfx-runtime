import typescript from '@rollup/plugin-typescript';
import nodeResolve from '@rollup/plugin-node-resolve';

export default function() {
    return {
        input: `./tests.ts`,
        external: [
            'wasi:webgpu/webgpu@0.3.0-rc.2',
            'wasi:clocks/monotonic-clock@0.3.0',
        ],
        output: {
            file: `temp/tests.js`,
            format: 'esm'
        },
        plugins: [
            nodeResolve(),
            typescript(),
        ]
    };
};
