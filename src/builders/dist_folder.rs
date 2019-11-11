use std::time::Instant;
// use std::path::PathBuf;
// use std::str::FromStr;
use std::fs;
use std::collections::HashMap;
use yansi::Paint;
use std::error::Error;
use super::{index_html}; // fastboot_package_json
use select::document::Document;
use super::super::utils::{console, file, html_file};
use super::super::types::Config;

// asset_map{ file_name: file_content }, hashed_file_names{file_name: hashed_file_name} (comes
// later after needed assets defined), then changes original html to hashed references and write
// hash filenames to the dist folder(this allows uniq file caching for clients)
pub fn build(config: &Config) -> Result<(String, Vec<HashMap<String, String>>), Box<dyn Error>> {
    console::log(format!("{} {}...", Paint::yellow("BUNDLING:"), config.application_name));

    let bundle_start = Instant::now();
    let environment = config.env["environment"].as_str().unwrap_or("development");
    let project_root = &config.project_root.display();
    let output_directory = format!("{}/dist", &project_root);
    let should_build_tests = (environment != "production") && config.cli_arguments.testing;
    let should_build_documentation = config.env["documentation"]["enabled"].as_bool().unwrap_or(false);

    reset_output_folder(&output_directory.as_str())?;

    let mut build_files: Vec<String> = Vec::new();
    let index_html_path = format!("{}/index.html", &project_root);

    build_files.extend(get_build_files_from_html(&index_html_path.as_str(), &config, false)?);

    let mut build_html_paths = vec![index_html_path];

    if should_build_tests {
        let tests_html_path = format!("{}/tests/index.html", &project_root);

        build_files.extend(get_build_files_from_html(&tests_html_path.as_str(), &config, false)?);
        build_html_paths.push(tests_html_path);
    }

    if should_build_documentation {
        let documentation_html_path = format!("{}/index.html", &project_root);

        build_files.extend(get_build_files_from_html(&documentation_html_path.as_str(), &config, true)?);
        build_html_paths.push(documentation_html_path);
    }

    build_files.sort();
    build_files.dedup();

    let target_asset_map = build_files.iter().fold(HashMap::new(), |mut result, file_name| {
        // let smt = format!("{}/tmp{}", &project_root, &file_name);
        // println!("{}", smt.as_str());
        let content = fs::read_to_string(format!("{}/tmp{}", &project_root, &file_name)).unwrap();

        result.insert(file_name, content);

        return result;
    });
    println!("{:?}", build_files);
    // println!("{:?}", target_asset_map);

    // let hashed_file_names = get_hashed_filename_map(&target_asset_map);

    // safe_write_html_and_assets(build_html_paths, target_asset_map, &hashed_file_names)?;
    // write_asset_map(&project_root, hashed_file_names)?;
    // copy_public_folder(&project_root)?;

    // if config.cli_arguments.fastboot {
    //     fastboot_package_json::build(hashed_file_names, config, Some("dist"))?;
    // }

    // TODO: in future create a thread global build error to say/stop tts on error

    let message = format!(
        "{} {} in {}",
        Paint::green("BUNDLED:"),
        &config.application_name,
        Paint::yellow(file::format_time_passed(bundle_start.elapsed().as_millis())),
    );

    console::log(message);
    console::log(Paint::green("Built project successfully. Stored in \"./dist\":"));

    // let output_metadata = fs::read_dir(output_directory)?;

    // output_metadata.iter_mut().filter().for_each(|file_path| {
    // file::report_file(file_path);
    // });
    // const fileObject = {
    //   fileName: stripProcessCWD(filePath),
    //   size: fileBuffer.length,
    //   gzipSize: gzipBuffer.length
    // }

    // console.log(
    //   chalk.blue(` - ${fileObject.fileName}:`),
    //   chalk.yellow(formatSize(fileObject.size)),
    //   chalk.green(`[${formatSize(fileObject.gzipSize)} gzipped]`)
    // );

    // return Ok((message, output_metadata));
    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert(String::from("a"), String::from("b"));

    return Ok((String::from(""), vec![hashmap]));
}

fn reset_output_folder(output_directory: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all(output_directory).unwrap_or_else(|_| {});
    // fs::create_dir_ahtml_pathll(format!("{}/assets", output_directory).as_str())?;

    // return Ok(fs::crhtml_patheate_dir_all(format!("{}/dist/assets", folder_path).as_str())?); // NOTE: very important breaks other tests otherwise
    return Ok(fs::create_dir_all(format!("{}/assets", output_directory).as_str())?);
}

fn get_build_files_from_html(html_path: &str, config: &Config, is_documentation: bool)
    -> Result<Vec<String>, Box<dyn Error>> {
    let html = match is_documentation {
        true => index_html::build_documentation_html(&html_path, &config)?,
        false => index_html::build(&html_path, &config)?
    };
    let html_document = Document::from(html.as_str());
    let (mut html_js_files, mut html_css_files) = html_file::find_internal_assets_from_html(&html_document);

    return Ok(html_js_files.into_iter().chain(html_css_files.into_iter()).collect());
}

fn get_hashed_filename_map(asset_map: &HashMap<String, String>) -> HashMap<String, String> {
    return asset_map.clone();
}

fn safe_write_html_and_assets(
    build_html_paths: Vec<&str>, target_asset_map: HashMap<String, String>, hashed_file_names: &HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
    return Ok(());
}

fn write_asset_map(project_root: &str, hashed_file_names: HashMap<String, String>)
    -> Result<(), Box<dyn Error>> {
    return Ok(());
}

fn copy_public_folder(project_root: &str) -> Result<(), Box<dyn Error>> {
    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use std::path::PathBuf;
    use serde_json::json;
    use std::collections::HashMap;
    use super::super::{build_all_assets};
    use super::super::super::types::BuildCache;

    const TIME_TO_BUILD_DIST_THRESHOLD: u128 = 4000;

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        env::set_current_dir(&project_directory)?;

        let output_directory = format!("{}/dist", &project_directory);

        fs::remove_dir_all(&output_directory).unwrap_or_else(|_| {});
        fs::remove_dir_all("tmp").unwrap_or_else(|_| {});

        return Ok((current_directory, output_directory, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all("tmp")?;
        fs::remove_dir_all("dist")?;
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_dist_folder_works() -> Result<(), Box<dyn Error>> {
        let (actual_current_directory, output_directory, project_directory) = setup_test()?;

        assert_eq!(fs::metadata(output_directory).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        ); // NOTE: testing: true must be there

        build_all_assets(&config)?;

        let build_start = Instant::now();
        let (message, created_files) = build(&config)?;
        let time_passed = build_start.elapsed().as_millis();
        let file_names: Vec<PathBuf> = fs::read_dir("tmp/assets")?.fold(Vec::new(), |mut result, entry| {
            let dir_entry = entry.unwrap();

            if dir_entry.metadata().unwrap().is_file() {
                result.push(dir_entry.path());
            }

            return result;
        });

        assert!(time_passed < TIME_TO_BUILD_DIST_THRESHOLD);
        assert!(file_names.len() == 6);

        let file_contents = [
            fs::read_to_string("tmp/assets/vendor.js")?,
            fs::read_to_string("tmp/assets/application.css")?,
            fs::read_to_string("tmp/assets/application.js")?,
            fs::read_to_string("tmp/assets/test-support.css")?,
            fs::read_to_string("tmp/assets/test-support.js")?,
            fs::read_to_string("tmp/assets/tests.js")?
        ];
        let target_index_html_assets = file_names.iter().filter(|file_name| {
            let target_file_name = file_name.to_str().unwrap().to_string();

            return !target_file_name.contains("tests") && !target_file_name.contains("test-support");
        });
        // let output_html = fs::read_to_string("dist/index.html")?;

        // target_index_html_assets.for_each(|file_name| {
        //     println!("{}", output_html);
        //     let target_reference = file_name.to_str().unwrap().to_string().replace("./", "").replace("dist/", "/");

        //     assert!(&output_html.contains(&target_reference));
        // });

        // let target_file_names =
        // let index_html_assets =
        // t.true(!(await fs.exists(`${PROJECT_ROOT}/dist`)));

        // const ENV = environmentFunc('development');

      // const files = await buildDistFolder({
      //   applicationName: 'some-app',
      //   ENV: ENV
      // });
      // const timePassed = timer.stop();

      // t.true(files.length === 8);
      // t.true(timePassed < TIME_TO_BUILD_DIST_THRESHOLD);

      // const fileNames = files.reduce((result, file) => {
      //   if (!file.fileName.includes('documentation')) {
      //     result.push(file.fileName);
      //   }

      //   return result;
      // }, []);
      // const outputHTML = (await fs.readFile(INDEX_HTML_OUTPUT_PATH)).toString();
      // const fileContents = await Promise.all([
      //   fs.readFile(`${PROJECT_ROOT}/tmp/assets/application.css`),
      //   fs.readFile(`${PROJECT_ROOT}/tmp/assets/test-support.css`),
      //   fs.readFile(`${PROJECT_ROOT}/tmp/assets/application.js`),
      //   fs.readFile(`${PROJECT_ROOT}/tmp/assets/vendor.js`),
      //   fs.readFile(`${PROJECT_ROOT}/tmp/assets/test-support.js`),
      //   fs.readFile(`${PROJECT_ROOT}/tmp/assets/tests.js`)
      // ]);
      // const targetIndexHTMLAssets = fileNames
      //   .filter((fileName) => {
      //     return !fileName.includes('tests') && !fileName.includes('test-support');
      //   });

      // await Promise.all(targetIndexHTMLAssets.map((fileName) => {
      //   const targetFileName = fileName.replace('./', '');

      //   t.true(outputHTML.includes(targetFileName.replace('dist/', '/')));
      // }));

      // const testHTML = (await fs.readFile(TEST_HTML_OUTPUT_PATH)).toString();
      // const testFileAssetContents = await Promise.all(fileNames.map((fileName) => {
      //   const targetFileName = fileName.replace('./', '');

      //   t.true(testHTML.includes(targetFileName.replace('dist/', '/')));

      //   return fs.readFile(`${PROJECT_ROOT}/${targetFileName}`);
      // }));

      // testFileAssetContents.forEach((hashedFileContent) => {
      //   t.truthy(fileContents.find((fileContent) => fileContent.length === hashedFileContent.length));
      // });
      // files.forEach((file) => {
      //   t.truthy(!INITIAL_BUILD_FILES.find((fileName) => file.fileName.endsWith(fileName)));
      //   t.true((file.gzipSize > 0) && (file.gzipSize < file.size));
      // });

      // t.true(await fs.exists(`${PROJECT_ROOT}/dist/package.json`));

      // const assetMap = JSON.parse(await fs.readFile(`${PROJECT_ROOT}/dist/assets/assetMap.json`));

      // t.true(assetMap.prepend === '');
      // t.true(Object.keys(assetMap.assets).length === 9);
      // t.true(assetMap.assets['assets/assetMap.json'] === 'assets/assetMap.json');

      // const targetFileNames = fileNames.map((fileName) => fileName.replace('./dist/', ''));

      // t.truthy(targetFileNames.find((fileName) => fileName === assetMap.assets['assets/application.css']))
      // t.truthy(targetFileNames.find((fileName) => fileName === assetMap.assets['assets/application.js']))
      // t.truthy(targetFileNames.find((fileName) => fileName === assetMap.assets['assets/test-support.js']))
      // t.truthy(targetFileNames.find((fileName) => fileName === assetMap.assets['assets/test-support.css']))
      // t.truthy(targetFileNames.find((fileName) => fileName === assetMap.assets['assets/tests.js']))
      return finalize_test(actual_current_directory);
    }

    // #[test]
    // fn build_works_for_different_application_with_memserver_mode() {
    // }

    // #[test]
    // fn build_works_for_production() {
    // }

    // #[test]
    // fn build_works_for_different_application_with_memserver_mode_and_fastboot_false() {
    // }

    // #[test]
    // fn build_resets_dist() {
    // }
}
