use std::path::PathBuf;
use std::env;

pub mod console;
pub mod ember_app_boilerplate;
pub mod search;

pub fn find_project_root() -> PathBuf {
    let mut path = search::in_parent_directories(&env::current_dir().unwrap(), "package.json").unwrap();

    path.pop();

    return path;
}

// pub fn write_file_if_not_exists() { // TODO: check if there is std alternative

// }

#[cfg(test)]
mod tests {
    use std::io;
    use std::env;
    use super::*;

    #[test]
    fn find_project_root_works_for_current_directory_of_a_project() -> io::Result<()> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        env::set_current_dir(&project_directory);

        assert_eq!(find_project_root(), PathBuf::from(project_directory));

        env::set_current_dir(&current_directory);

        Ok(())
    }

    #[test]
    fn find_project_root_works_for_parent_directory_of_a_project() -> io::Result<()> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let mocked_directory = format!("{}/ember-app-boilerplate/src", current_directory.to_string_lossy());

        env::set_current_dir(&mocked_directory);

        assert_eq!(find_project_root(), PathBuf::from(project_directory));

        env::set_current_dir(&current_directory);

        Ok(())
    }

    #[test]
    fn find_project_root_works_for_two_level_deep_parent_directory_of_a_project() -> io::Result<()> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let mocked_directory = format!("{}/ember-app-boilerplate/src/ui", current_directory.to_string_lossy());

        env::set_current_dir(&mocked_directory);

        assert_eq!(find_project_root(), PathBuf::from(project_directory));

        env::set_current_dir(&current_directory);

        Ok(())
    }
}
