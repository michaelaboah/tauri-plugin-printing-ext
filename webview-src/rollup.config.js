import { nodeResolve } from '@rollup/plugin-node-resolve'
import { terser } from 'rollup-plugin-terser'
import typescript from '@rollup/plugin-typescript'
import commonjs from "@rollup/plugin-commonjs"
export default {
  input: './webview-src/index.ts',
  output: {
    dir: './webview-dist',
    entryFileNames: '[name].js',
    format: 'es',
    exports: 'auto'
  },
  plugins: [
    commonjs(),
    // nodeResolve(),
    terser(),
    typescript({
      tsconfig: './webview-src/tsconfig.json',
      moduleResolution: 'node'
    })
  ]
}
