use std::env;
use std::fs;
use std::process;
use std::io;
use inflector::cases::snakecase::to_snake_case;
use inflector::string::singularize::to_singular;
use yansi::Paint;
use super::super::utils;
use super::super::utils::console;

pub fn run() -> std::io::Result<()> {
    let abstraction = std::env::args()
        .nth(2)
        .unwrap_or_else(|| {
            console::error("mber d missing an ember abstraction to delete!");

            process::exit(1);
        });
    let ember_abstractions = vec![
        "component", "helper", "initializer", "instance-initializer", "mixin", "model", "route", "service", "util"
    ];

    if !ember_abstractions.contains(&abstraction.as_str()) {
        console::log(format!("{} is not a valid ember abstraction to delete. Choose one of these abstractions:", abstraction));
        println!("{:?}", ember_abstractions);

        process::exit(1);
    }

    let remaining_args = env::args().skip(3).collect::<Vec<_>>();

    if remaining_args.len() == 0 {
        console::error(format!("mber d {} missing a name to delete!", abstraction));

        process::exit(1);
    }

    let name = remaining_args.join(" ");
    let project_root = utils::find_project_root().to_string_lossy().to_string();

    match abstraction.as_str() {
        "component" => {
            let folder_name = to_snake_case(&to_singular(&name)).replace("_", "-");
            let target_folder = format!("{}/src/ui/components/{}", project_root, folder_name);

            check_if_path_exists(&target_folder, &project_root);
            remove_folder(target_folder, project_root)?;
        },
        "helper" => {
            let file_name = to_snake_case(&name).replace("_", "-");
            let file_path = format!("{}/src/ui/components/{}.js", project_root, file_name);

            check_if_path_exists(&file_path, &project_root);
            remove_file_if_exists(&file_path, &project_root)?;
            remove_file_if_exists(&format!("{}/src/ui/components/{}-test.js", project_root, file_name), &project_root)?;
        },
        "initializer" => {
            let file_name = to_snake_case(&name).replace("_", "-");
            let file_path = format!("{}/src/init/initializers/{}.js", project_root, file_name);

            check_if_path_exists(&file_path, &project_root);
            remove_file_if_exists(&file_path, &project_root)?;
            remove_file_if_exists(&format!("{}/src/init/initializers/{}-test.js", project_root, file_name), &project_root)?;
        },
        "instance_initializer" => {
            let file_name = to_snake_case(&name).replace("_", "-");
            let file_path = format!("{}/src/init/instance-initializers/{}.js", project_root, file_name);

            check_if_path_exists(&file_path, &project_root);
            remove_file_if_exists(&file_path, &project_root)?;
            remove_file_if_exists(&format!("{}/src/init/instance-initializers/{}-test.js", project_root, file_name), &project_root)?;
        },
        "mixin" => {
            let file_name = to_snake_case(&name).replace("_", "-");
            let file_path = format!("{}/src/utils/mixins/{}.js", project_root, file_name);

            check_if_path_exists(&file_path, &project_root);
            remove_file_if_exists(&file_path, &project_root)?;
            remove_file_if_exists(&format!("{}/src/utils/mixins/{}-test.js", project_root, file_name), &project_root)?;
        },
        "model" => {
            let folder_name = to_snake_case(&to_singular(&name)).replace("_", "-");
            let target_folder = format!("{}/src/data/models/{}", project_root, folder_name);

            check_if_path_exists(&target_folder, &project_root);
            remove_folder(target_folder, project_root)?;
        },
        "route" => {
            let folder_name = to_snake_case(&name).replace("_", "-");
            let target_folder = format!("{}/src/ui/routes/{}", project_root, folder_name);

            check_if_path_exists(&target_folder, &project_root);
            remove_folder(target_folder, project_root)?;
        },
        "service" => {
            let file_name = to_snake_case(&name).replace("_", "-");
            let file_path = format!("{}/src/services/{}.js", project_root, file_name);

            check_if_path_exists(&file_path, &project_root);
            remove_file_if_exists(&file_path, &project_root)?;
            remove_file_if_exists(&format!("{}/src/services/{}-test.js", project_root, file_name), &project_root)?;
        }
        "util" => {
            let file_name = to_snake_case(&name).replace("_", "-");
            let file_path = format!("{}/src/utils/{}.js", project_root, file_name);

            check_if_path_exists(&file_path, &project_root);
            remove_file_if_exists(&file_path, &project_root)?;
            remove_file_if_exists(&format!("{}/src/utils/{}-test.js", project_root, file_name), &project_root)?;
        }
        _ => ()
    }

    Ok(())
}

fn check_if_path_exists(path: &String, project_root: &String) {
    if fs::metadata(&path).is_err() {
        let target_path = path.replace(format!("{}/", project_root).as_str(), "");

        console::error(format!("{} does not exist!", target_path));
        process::exit(1);
    }
}

fn remove_file_if_exists(path: &String, project_root: &String) -> io::Result<()> {
    if fs::metadata(&path).is_ok() {
        fs::remove_file(&path)?;
        console::log(format!("{} {}", Paint::red("deleted"), path.replace(format!("{}/", project_root).as_str(), "")));
    }

    return Ok(());
}

fn remove_folder(path: String, project_root: String) -> io::Result<()> {
    fs::remove_dir_all(&path)?;
    console::log(format!("{} {}", Paint::red("deleted"), path.replace(format!("{}/", project_root).as_str(), "")));

    return Ok(());
}
