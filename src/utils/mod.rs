use std::path::PathBuf;
use std::io;
use std::fs;
use std::env;
use std::process;
use yansi::Paint;

pub mod application;
pub mod console;
pub mod file;
pub mod html_file;
pub mod recursive_file_lookup;
pub mod say;
pub mod walk_injection;

pub fn find_project_root() -> PathBuf {
    let mut path = application::in_parent_directories(&env::current_dir().unwrap(), "package.json").unwrap_or_else(|| {
        console::error("you are not on a frontend project! Change your directory");

        process::exit(1);
    });

    path.pop();

    return path;
}

pub fn write_file_if_not_exists(file_path: String, content: &str, project_root: &PathBuf) -> io::Result<()> { // TODO: add Future
    if fs::metadata(&file_path).is_ok() {
        console::log(format!("{} {}", Paint::yellow("not changed"), humanize_path(file_path, project_root)));

        return Ok(());
    }

    let result = fs::write(&file_path, content);

    console::log(format!("{} {}", Paint::green("created"), humanize_path(file_path, project_root)));

    return result;
}

fn humanize_path(file_path: String, project_root: &PathBuf) -> String {
    return file_path.replace(project_root.to_str().unwrap(), "");
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::env;
    use super::*;

    #[test]
    fn find_project_root_works_for_current_directory_of_a_project() -> io::Result<()> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        env::set_current_dir(&project_directory)?;

        assert_eq!(find_project_root(), PathBuf::from(project_directory));

        env::set_current_dir(&current_directory)?;

        Ok(())
    }

    #[test]
    fn find_project_root_works_for_parent_directory_of_a_project() -> io::Result<()> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let mocked_directory = format!("{}/ember-app-boilerplate/src", current_directory.to_string_lossy());

        env::set_current_dir(&mocked_directory)?;

        assert_eq!(find_project_root(), PathBuf::from(project_directory));

        env::set_current_dir(&current_directory)?;

        Ok(())
    }

    #[test]
    fn find_project_root_works_for_two_level_deep_parent_directory_of_a_project() -> io::Result<()> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let mocked_directory = format!("{}/ember-app-boilerplate/src/ui", current_directory.to_string_lossy());

        env::set_current_dir(&mocked_directory)?;

        assert_eq!(find_project_root(), PathBuf::from(project_directory));

        env::set_current_dir(&current_directory)?;

        Ok(())
    }
}
