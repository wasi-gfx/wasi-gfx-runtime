import typescript from '@rollup/plugin-typescript';
import nodeResolve from '@rollup/plugin-node-resolve';

export default function() {
    return {
        input: `./tests.ts`,
        external: [
            'wasi:webgpu/webgpu',
            'wasi:webgpu/surface',
            'wasi:webgpu/graphics-context',
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
