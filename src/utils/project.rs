// NOTE: benchmark compare this one with tokio/mio!!
use std::error::Error;
use std::fs;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn in_parent_directories(starting_directory: &Path, target_file_with_extension: &str) -> Option<PathBuf> {
    let target_path = PathBuf::from_str(format!("{}/{}", starting_directory.to_str().unwrap(), target_file_with_extension).as_str()).unwrap();

    return search_in_directory(target_path, &target_file_with_extension);
}

fn search_in_directory(mut path: PathBuf, target_file_with_extension: &str) -> Option<PathBuf> {
    if path.exists() {
        return Some(path);
    } else if path.pop() && path.pop() {
        path.push(target_file_with_extension);

        return search_in_directory(path, &target_file_with_extension);
    }

    return None;
}

pub fn recursively_copy_folder(folder_path: String, target_path: &String) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&target_path).unwrap_or_else(|_| {});

    for entry in WalkDir::new(&folder_path).into_iter() {
        let entry = entry.expect("ENTRY NOT FOUND");
        let target_path = entry.path().to_str().unwrap().replace(&folder_path, &target_path);

        match entry.file_type().is_dir() {
            true => { fs::create_dir_all(target_path)?; },
            false => { fs::copy(entry.path(), target_path)?; }
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
    use std::ffi::OsString;
    use super::*;

    fn setup() -> io::Result<()> {
        if fs::metadata("online-shop").is_ok() {
            fs::remove_dir_all("online-shop")?;
        }

        fs::create_dir("online-shop")?;
        fs::create_dir("online-shop/shoes")?;
        fs::create_dir("online-shop/shirts")?;
        fs::create_dir("online-shop/shoes/shoe")?;
        fs::write("online-shop/index.js", "// find me in online-shop/index.js")?;
        fs::write("online-shop/details.js", "// find me in online-shop/details.js")?;
        fs::write("online-shop/shoes/shoe.js", "// find me in online-shop/shoes/shoe.js")?;
        fs::write("online-shop/shoes/index.js", "// find me in online-shop/shoes/index.js")?;
        fs::write("online-shop/shoes/brown.js", "// find me in online-shop/shoes/brown.js")?;
        fs::write("online-shop/shoes/shoe/brown.js", "// find me in online-shop/shoes/shoe/brown.js")?;

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_works_for_current_directory() -> io::Result<()> {
        setup()?;

        let target_dir = Path::new("online-shop/shoes");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "shoe.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/shoes/shoe.js"));

        fs::remove_dir_all("online-shop")?;

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_works_for_parent_directory() -> io::Result<()> {
        setup()?;

        let target_dir = Path::new("online-shop/shoes");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "details.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/details.js"));

        fs::remove_dir_all("online-shop")?;

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_works_for_two_level_parent_directory() -> io::Result<()> {
        setup()?;

        let target_dir = Path::new("online-shop/shoes/shoe");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "details.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/details.js"));

        fs::remove_dir_all("online-shop")?;

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_gets_right_files_when_it_has_duplicate_in_parents() -> io::Result<()> {
        setup()?;

        let target_dir = Path::new("online-shop/shoes");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "index.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/shoes/index.js"));

        fs::remove_dir_all("online-shop")?;

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_return_none_when_nothing_is_found() -> io::Result<()> {
        setup()?;

        let target_dir = Path::new("online-shop/shoes");

        assert_eq!(in_parent_directories(target_dir, "lol.js").is_none(), true);

        fs::remove_dir_all("online-shop")?;

        Ok(())
    }

    #[test]
    fn recursively_copy_folder_works() -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all("test-tmp").unwrap_or_else(|_| {});

        recursively_copy_folder("ember-app-boilerplate/public".to_string(), &"test-tmp".to_string())?;

        let test_tmp_dir_entries = fs::read_dir("test-tmp")?.into_iter().map(|entry| -> String {
            let target_path = entry.unwrap().path();

            return target_path.to_str().unwrap().to_string();
        }).collect::<Vec<String>>();

        vec![
            "test-tmp/crossdomain.xml", "test-tmp/favicon.ico", "test-tmp/images", "test-tmp/robots.txt"
        ].into_iter().for_each(|path| assert_eq!(test_tmp_dir_entries.contains(&path.to_string()), true));

        let mut folder_entries = WalkDir::new("ember-app-boilerplate/public").into_iter();
        let tmp_entries = WalkDir::new("test-tmp").into_iter()
            .map(|entry| entry.unwrap().file_name().to_os_string())
            .collect::<Vec<OsString>>();

        assert!(folder_entries.all(|entry| {
            let file_name = entry.unwrap().file_name().to_os_string();

            if file_name == "public" {
                return true;
            }

            return tmp_entries.contains(&file_name);
        }));

        return Ok(fs::remove_dir_all("test-tmp")?);
    }
}
