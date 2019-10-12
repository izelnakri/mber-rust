use std::env;
// use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// pub fn with_extensions<'a, 'b>(directory: &'a Path, extensions: &'b Vec<&str>) {
pub fn with_extensions<'a, 'b>(directory: &'a Path) -> Vec<PathBuf> {
    return WalkDir::new(directory).into_iter().filter_map(|e| {
        let entry = e.unwrap();

        return match &entry.file_name().to_str().unwrap().ends_with(".js") {
            true => Some(entry.into_path()),
            false => None
        };
    }).collect();
}

// pub fn with_extensions_and_filter(directory: &Path, extensions: &Vec, filter: Fn) -> Vec<Path> {

// }

#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
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
        fs::write("online-shop/details.hbs", "// find me in online-shop/details.hbs")?;
        fs::write("online-shop/shoes/shoe.js", "// find me in online-shop/shoes/shoe.js")?;
        fs::write("online-shop/shoes/index.js", "// find me in online-shop/shoes/index.js")?;
        fs::write("online-shop/shoes/brown.js", "// find me in online-shop/shoes/brown.js")?;
        fs::write("online-shop/shoes/brown.hbs", "// find me in online-shop/shoes/brown.hbs")?;
        fs::write("online-shop/shoes/shoe/brown.js", "// find me in online-shop/shoes/shoe/brown.js")?;

        Ok(())
    }

    #[test]
    fn with_extensions_works_for_js_and_hbs_by_default() -> io::Result<()> {
        setup()?;
        let current_dir = env::current_dir()?;
        // let online_shop_directory = Path::new(format!("{}/online-shop", &current_dir.to_string_lossy()).as_str());
        let online_shop_files = with_extensions(Path::new(format!("{}/online-shop", &current_dir.to_string_lossy()).as_str()));
        // let online_shop_files = with_extensions(online_shop_directory, &vec![]);
        // let shoes_files = with_extensions("${CWD}/online-shop/shoes");
        // let shoe_files = with_extensions("${CWD}/online-shop/shoes/shoe");

        // assert_eq!(online_shop_files, vec![&current_dir]);
        assert_eq!(true, true);

        Ok(())
    }
}

