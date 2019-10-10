#[macro_use]
extern crate neon;
extern crate mber;

use mber::utils::console;
use neon::prelude::*;

fn import(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(""))
}

fn import_addon(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(""))
}

fn import_as_amd_module(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(""))
}

fn inject_inline_content(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(""))
}

fn build(mut cx: FunctionContext) -> JsResult<JsString> {
    console::log("This is from Rust!: calling from index.js neon binding....");

    Ok(cx.string("returning from build bro"))
}

register_module!(mut m, {
    m.export_function("import", import)?;
    m.export_function("importAddon", import_addon)?;
    m.export_function("importAsAMDModule", import_as_amd_module)?;
    m.export_function("injectInlineContent", inject_inline_content)?;
    m.export_function("build", build)
});

    // return new Promise(async (resolve) => {
    //   global.MBER_THREAD_POOL = WorkerPool.start(os.cpus().length);

    //   const projectRoot = await findProjectRoot();
    //   const ENV = serializeRegExp((await import(`${projectRoot}/config/environment.js`)).default(environment));
    //   const applicationName = ENV.modulePrefix || 'frontend';
    //   const buildMeta = [
    //     'vendorPrepends', 'vendorAppends', 'applicationPrepends', 'applicationAppends',
    //     'testPrepends', 'testAppends'
    //   ].reduce((result, key) => {
    //     if (this[key].length > 0) {
    //       return Object.assign(result, {
    //         [key]: transpileAddonToES5(projectRoot, this[key], applicationName)
    //       });
    //     }

    //     return result;
    //   }, {});

    //   Promise.all(Object.keys(buildMeta).map((metaKey) => buildMeta[metaKey]))
    //     .then(async (finishedBuild) => {
    //       const cliArguments = Object.assign({}, {
    //         fastboot: true,
    //         port: 1234,
    //         socketPort: (global.MBER_DISABLE_SOCKETS|| ENV.environment === 'production') ? null : 65511,
    //         talk: true,
    //         testing: ENV.environment !== 'production'
    //       }, parseCLIArguments());
    //       const { socketPort, port } = cliArguments;
    //       const targetPort = await resolvePortNumberFor('Web server', port);
    //       const targetSocketPort = socketPort ?
    //         (await resolvePortNumberFor('Websocket server', socketPort)) : null;
    //       const result = await buildAssets({
    //         applicationName: ENV.modulePrefix || 'frontend',
    //         ENV: ENV,
    //         cliArguments: Object.assign({}, cliArguments, {
    //           port: targetPort,
    //           socketPort: targetSocketPort,
    //         }),
    //         projectRoot: projectRoot,
    //         buildCache: finishedBuild.reduce((result, code, index) => {
    //           return Object.assign(result, { [`${Object.keys(buildMeta)[index]}`]: code });
    //         }, {}),
    //         indexHTMLInjections: this.indexHTMLInjections,
    //       });

    //       resolve(result);
    //     }).catch((error) => reportErrorAndExit(error));

