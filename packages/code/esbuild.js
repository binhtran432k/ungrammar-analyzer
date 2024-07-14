// @ts-check
const esbuild = require("esbuild");
const glob = require("glob");
const path = require("path");
const polyfill = require("@esbuild-plugins/node-globals-polyfill");

const isProduction = process.argv.includes("--production");
const isWatch = process.argv.includes("--watch");
const isTest = process.argv.includes("--test");

/**
 * For web extension, all tests, including the test runner, need to be bundled into
 * a single module that has a exported `run` function .
 * This plugin bundles implements a virtual file extensionTests.ts that bundles all these together.
 * @type {import('esbuild').Plugin}
 */
const testBundlePlugin = {
  name: "testBundlePlugin",
  setup(build) {
    build.onResolve({ filter: /[\/\\]extensionTests\.ts$/ }, (args) => {
      if (args.kind === "entry-point") {
        return { path: path.resolve(args.path) };
      }
    });
    build.onLoad({ filter: /[\/\\]extensionTests\.ts$/ }, async (args) => {
      const testsRoot = path.join(__dirname, "src/test/suite");
      const files = await glob.glob("*.test.{ts,tsx}", { cwd: testsRoot, posix: true });
      return {
        contents:
          `export { run } from './mochaTestRunner.ts';` +
          files.map((f) => `import('./${f}');`).join(""),
        watchDirs: files.map((f) => path.dirname(path.resolve(testsRoot, f))),
        watchFiles: files.map((f) => path.resolve(testsRoot, f)),
      };
    });
  },
};

/**
 * @type {import('esbuild').Plugin}
 */
const esbuildProblemMatcherPlugin = {
  name: "esbuild-problem-matcher",

  setup(build) {
    build.onStart(() => {
      console.log("[watch] build started");
    });
    build.onEnd((result) => {
      result.errors.forEach(({ text, location }) => {
        console.error(`✘ [ERROR] ${text}`);
        if (location) {
          console.error(`    ${location.file}:${location.line}:${location.column}:`);
        }
      });
      console.log("[watch] build finished");
    });
  },
};

/**
 * @typedef {import('esbuild').BuildOptions} BuildOptions
 */

/** @type {BuildOptions} */
const sharedOptions = {
  bundle: true,
  external: ["vscode"],
  target: "es2020",
  format: "cjs",
  minify: isProduction,
  sourcemap: !isProduction,
  sourcesContent: false,
  plugins: [esbuildProblemMatcherPlugin],
};

/** @type {BuildOptions} */
const webOptions = {
  ...sharedOptions,
  entryPoints: ["src/extension.ts"],
  outdir: "dist/web",
  platform: "browser",
};

/** @type {BuildOptions} */
const desktopOptions = {
  ...sharedOptions,
  entryPoints: ["src/extension.ts"],
  outdir: "dist/desktop",
  platform: "node",
};

/** @type {BuildOptions} */
const testOptions = {
  ...sharedOptions,
  entryPoints: ["src/test/suite/extensionTests.ts"],
  outfile: "dist/extensionTests.js",
  define: {
    global: "globalThis",
  },
  plugins: [
    polyfill.NodeGlobalsPolyfillPlugin({
      process: true,
      buffer: true,
    }),
    testBundlePlugin,
    esbuildProblemMatcherPlugin,
  ],
  platform: "browser",
};

function createContexts() {
  if (isTest) {
    return Promise.all([esbuild.context(testOptions)]);
  }
  return Promise.all([esbuild.context(webOptions), esbuild.context(desktopOptions)]);
}

createContexts()
  .then(async (contexts) => {
    if (isWatch) {
      const promises = contexts.map((context) => context.watch());
      await Promise.all(promises);
    } else {
      const promises = contexts.map((context) => context.rebuild());
      await Promise.all(promises);
      for (const context of contexts) {
        await context.dispose();
      }
    }
  })
  .then(() => undefined)
  .catch(console.error);
