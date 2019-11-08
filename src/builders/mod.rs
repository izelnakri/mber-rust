pub mod application;
pub mod css;
pub mod dist_folder;
pub mod documentation_css;
pub mod documentation_js;
pub mod fastboot_package_json;
pub mod index_html;
pub mod memserver;
pub mod tests;
pub mod vendor;

// pub fn build_all_assets(config: Config) -> Result<(Vec<String>, Vec<fs::Metadata>), Box<dyn Error>> {
//     let project_root = &config.project_root.display();
//     // let tmp_path

//     // if !config.cli_arguments.testing {
//     //     fs::remove_dir_all("tmp")?;
//     // }

//     await fs.mkdirp(`${projectRoot}/tmp/assets`);

//     //   buildIndexHTML(`${projectRoot}/index.html`, buildConfig),
//     //   buildCSS(buildConfig),
//     //   buildVendor(buildConfig),
//     //   buildApplication(buildConfig, lint),
//     //   memserverIsEnabled ? buildFastbootPackageJSON(Object.assign(defaultAssetMap, {
//     //     "assets/memserver.js": "assets/memserver.js"
//     //   }), buildConfig, 'tmp') : buildFastbootPackageJSON(defaultAssetMap, buildConfig, 'tmp'),
//     //   memserverIsEnabled ? buildMemServer(buildConfig, lint) : null,
//     //   documentationIsEnabled ? buildDocumentation(buildConfig, lint) : null,
//     //   documentationIsEnabled ? buildDocumentationCSS(buildConfig) : null,
//     //   documentationIsEnabled ? buildDocumentationHTML(`${projectRoot}/index.html`, buildConfig) : null,
//     //   cliArguments.testing ? buildIndexHTML(`${projectRoot}/tests/index.html`, buildConfig) : null,
//     //   cliArguments.testing ? buildTests(buildConfig, lint) : null,
//     //   cliArguments.testing ? fs.copy(`${VENDOR_PATH}/test-support.css`, `${projectRoot}/tmp/assets/test-support.css`) : null,
//     //   cliArguments.testing ? fs.copy(`${VENDOR_PATH}/test-support.js`, `${projectRoot}/tmp/assets/test-support.js`) : null

// }

// fn reset_dist_directory() {
//     remove existing and create with /assets
// }
