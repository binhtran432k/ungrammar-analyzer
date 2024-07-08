// @ts-check
const esbuild = require("esbuild");

const isProduction = process.argv.includes("--production");
const isWatch = process.argv.includes("--watch");

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
  outfile: "dist/web/extension.js",
  platform: "browser",
};

/** @type {BuildOptions} */
const desktopOptions = {
  ...sharedOptions,
  entryPoints: ["src/extension.ts"],
  outfile: "dist/desktop/extension.js",
  platform: "node",
};

function createContexts() {
  return Promise.all([esbuild.context(webOptions), esbuild.context(desktopOptions)]);
}

createContexts()
  .then((contexts) => {
    if (isWatch) {
      const promises = contexts.map((context) => context.watch());
      return Promise.all(promises).then(() => undefined);
    } else {
      const promises = contexts.map((context) => context.rebuild());
      Promise.all(promises)
        .then(async () => {
          for (const context of contexts) {
            await context.dispose();
          }
        })
        .then(() => undefined)
        .catch(console.error);
    }
  })
  .catch(console.error);
