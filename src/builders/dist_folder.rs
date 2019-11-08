use std::time::Instant;
use std::path::PathBuf;
// use std::str::FromStr;
use std::fs;
use yansi::Paint;
use std::error::Error;
use super::super::utils::{console, html_file};
use super::super::types::Config;

// TODO: moved internal_html_asset lookup and path locality check to a util module
// asset_map{ file_name: file_content }, hashed_file_names{file_name: hashed_file_name} (comes
// later after needed assets defined), then changes original html to hashed references and write
// hash filenames to the dist folder(this allows uniq file caching for clients)
pub fn build(config: Config) -> Result<(Vec<String>, Vec<HashMap<String, String>), Box<dyn Error>> {
    console::log(format!("{} {}...", Paint::yellow("BUNDLING:"), config.application_name));

    let bundle_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_directory = format!("{}/dist", &project_root).as_str();
    let should_build_tests = (config.env["environment"].to_string() != "production") &&
        config.cli_arguments.testing;
    let should_build_documentation = config.env["documentation"]["enabled"].as_bool().unwrap_or(false);

    reset_output_folder(&output_directory)?;

    // NOTE: simplify below in functions
    // TODO: also do the transpileIndexHTML one more time?
    // let index_html_path = PathBuf::from_str(format!("{}/index.html", &project_root).as_str())?;
    // let index_html = fs::read_to_string(index_html_path);
    // let index_html_document = Document::from(index_html);
    // let build_asset_file_names = html_file::find_internal_assets_from_html(&index_html_document);
    // // let build_html_paths = vec![index_html_path];

    // if should_build_tests {
    //     let index_html_path = PathBuf::from_str(format!("{}/index.html", &project_root).as_str())?;
    //     let index_html = fs::read_to_string(index_html_path);
    //     let index_html_document = Document::from(index_html);
    //     let build_asset_file_names = html_file::find_internal_assets_from_html(&index_html_document);
    // }

    // if should_build_documentation {
    //     let index_html_path = PathBuf::from_str(format!("{}/index.html", &project_root).as_str())?;
    //     let index_html = fs::read_to_string(index_html_path);
    //     let index_html_document = Document::from(index_html);
    //     let build_asset_file_names = html_file::find_internal_assets_from_html(&index_html_document);
    // }

    // let target_asset_map = build_asset_file_names.iter().fold(HashMap::new(), |result, file_name| {
    //     result.insert(file_name, fs::read_to_string(file_name)?);

    //     return result;
    // }).collect::<HashMap<String, String>>();
    // let hashed_file_names = get_hashed_filenames(&target_asset_map);

    // safe_write_html_and_assets(build_html_paths, target_asset_map, hashed_file_names)?;
    // write_asset_map(&project_root, hashed_file_names)?;
    // // TODO: copy $project_root/public to output_directory

    // // TODO: change asset_map keys(file_names to hashed ones)
    // if &config.cli_arguments.fastboot {
    //     fastboot_package_json::build(target_asset_map, config, Some("dist"))?;
    // }

    // // TODO: in future create a thread global build error to say/stop tts on error

    // let message = format!(
    //     "{} {} in {}ms",
    //     Paint::green("BUNDLED:"),
    //     &config.application_name,
    //     Paint::yellow(file::format_time_passed(bundle_start.elapsed().as_millis())),
    // );

    // console::log(message):
    // console::log(Paint::green("Built project successfully. Stored in \"./dist\":")));

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
    let hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("a", "b");

    return Ok(("", vec![hashmap]);
}

fn reset_output_folder(folder_path: &str) -> Result<(PathBuf, Value, Value), Box<dyn Error>> {
    fs::remove_dir_all(&folder_path)?;

    return fs::create_dir_all(format!("{}/assets", folder_path).as_str()); // NOTE: very important breaks other tests otherwise
}
