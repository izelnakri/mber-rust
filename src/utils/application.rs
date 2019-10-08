// NOTE: benchmark compare this one with tokio/mio!!
// TODO: rename this to project
use std::str::FromStr;
use std::path::{Path, PathBuf};

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

#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
    use super::*;

    fn setup() -> io::Result<()> {
        if fs::metadata("online-shop").is_ok() {
            fs::remove_dir_all("online-shop");
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
        setup();

        let target_dir = Path::new("online-shop/shoes");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "shoe.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/shoes/shoe.js"));

        fs::remove_dir_all("online-shop");

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_works_for_parent_directory() -> io::Result<()> {
        setup();

        let target_dir = Path::new("online-shop/shoes");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "details.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/details.js"));

        fs::remove_dir_all("online-shop");

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_works_for_two_level_parent_directory() -> io::Result<()> {
        setup();

        let target_dir = Path::new("online-shop/shoes/shoe");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "details.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/details.js"));

        fs::remove_dir_all("online-shop");

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_gets_right_files_when_it_has_duplicate_in_parents() -> io::Result<()> {
        setup();

        let target_dir = Path::new("online-shop/shoes");
        let content = String::from_utf8(fs::read(in_parent_directories(target_dir, "index.js").unwrap())?).unwrap();

        assert_eq!(content, String::from("// find me in online-shop/shoes/index.js"));

        fs::remove_dir_all("online-shop");

        Ok(())
    }

    #[test]
    fn search_in_parent_directories_return_none_when_nothing_is_found() -> io::Result<()> {
        setup();

        let target_dir = Path::new("online-shop/shoes");

        assert_eq!(in_parent_directories(target_dir, "lol.js").is_none(), true);

        fs::remove_dir_all("online-shop");

        Ok(())
    }
}
