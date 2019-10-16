use std::path::{Path, PathBuf};
use std::iter::Iterator;
use walkdir::WalkDir;
use walkdir::DirEntry;

pub fn lookup_for_extensions(directory: &Path, extensions: Vec<&str>) -> Vec<PathBuf> {
    return WalkDir::new(directory).into_iter().filter_map(|e| {
        let entry = e.unwrap();

        return match extensions.iter().any(|extension| entry.file_name().to_str().unwrap().ends_with(extension)) {
            true => Some(entry.into_path()),
            false => None
        };
    }).collect();
}

pub fn lookup_for_extensions_and_predicate<F>(directory: &Path, extensions: Vec<&str>, filter: F) -> Vec<PathBuf>
    where F: Fn(&DirEntry) -> bool {
    return WalkDir::new(directory).into_iter().filter_map(|e| {
        let entry = e.unwrap();
        let entry_correct_extension = extensions.iter()
            .any(|extension| entry.file_name().to_str().unwrap().ends_with(extension));


        return match entry_correct_extension && filter(&entry) {
            true => Some(entry.into_path()),
            false => None
        };
    }).collect();
}

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
    fn lookup_for_extensions_works_for_js_and_hbs() -> io::Result<()> {
        setup()?;

        let online_shop_directory = Path::new("online-shop");
        let shoes_directory = Path::new("online-shop/shoes");
        let shoe_directory = Path::new("online-shop/shoes/shoe");
        let online_shop_js_files = lookup_for_extensions(online_shop_directory, vec!["js"]);
        let online_shop_hbs_files = lookup_for_extensions(online_shop_directory, vec!["hbs"]);
        let online_shop_files = lookup_for_extensions(online_shop_directory, vec!["hbs", "js"]);
        let shoes_js_files = lookup_for_extensions(shoes_directory, vec!["js"]);
        let shoes_hbs_files = lookup_for_extensions(shoes_directory, vec!["hbs"]);
        let shoes_files = lookup_for_extensions(shoes_directory, vec!["js", "hbs"]);
        let shoe_files = lookup_for_extensions(shoe_directory, vec!["js", "hbs"]);

        assert_eq!(
            online_shop_js_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec![
                "online-shop/shoes/shoe/brown.js", "online-shop/shoes/shoe.js", "online-shop/shoes/index.js",
                "online-shop/shoes/brown.js", "online-shop/index.js", "online-shop/details.js"
            ]
        );
        assert_eq!(
            online_shop_hbs_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec!["online-shop/shoes/brown.hbs", "online-shop/details.hbs"]
        );
        assert_eq!(
            online_shop_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec![
                "online-shop/shoes/shoe/brown.js", "online-shop/shoes/shoe.js", "online-shop/shoes/index.js",
                "online-shop/shoes/brown.js", "online-shop/shoes/brown.hbs", "online-shop/index.js",
                "online-shop/details.js", "online-shop/details.hbs"
            ]
        );
        assert_eq!(
            shoes_js_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec![
                "online-shop/shoes/shoe/brown.js", "online-shop/shoes/shoe.js", "online-shop/shoes/index.js",
                "online-shop/shoes/brown.js"
            ]
        );
        assert_eq!(
            shoes_hbs_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec!["online-shop/shoes/brown.hbs"]
        );
        assert_eq!(
            shoes_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec![
                "online-shop/shoes/shoe/brown.js", "online-shop/shoes/shoe.js", "online-shop/shoes/index.js",
                "online-shop/shoes/brown.js", "online-shop/shoes/brown.hbs"
            ]
        );
        assert_eq!(
            shoe_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec!["online-shop/shoes/shoe/brown.js"]
        );

        return fs::remove_dir_all("online-shop");
    }

    #[test]
    fn lookup_for_extensions_works_when_there_are_no_reference_files() -> io::Result<()> {
        setup()?;

        let shoe_directory = Path::new("online-shop/shoes/shoe");
        let shoe_hbs_files = lookup_for_extensions(shoe_directory, vec!["hbs"]);
        let online_shop_txt_files = lookup_for_extensions(shoe_directory, vec!["txt"]);
        let empty_array: Vec<&str> = Vec::new();

        assert_eq!(
            shoe_hbs_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            empty_array
        );
        assert_eq!(
            online_shop_txt_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            empty_array
        );

        return fs::remove_dir_all("online-shop");
    }

    #[test]
    fn lookup_for_extensions_and_predicate_works_for_js_and_hbs() -> io::Result<()> {
        setup()?;

        let online_shop_directory = Path::new("online-shop");
        let shoes_directory = Path::new("online-shop/shoes");
        let online_shop_js_files = lookup_for_extensions_and_predicate(online_shop_directory, vec!["js"], |e: &DirEntry| {
            return e.file_name().to_str().unwrap().ends_with("brown.js");
        });
        let online_shop_files = lookup_for_extensions_and_predicate(online_shop_directory, vec!["hbs", "js"], |e| {
            let file_name = e.file_name().to_str().unwrap();

            return file_name.ends_with("brown.js") || file_name.ends_with("details.hbs");
        });
        let shoes_js_files = lookup_for_extensions_and_predicate(shoes_directory, vec!["js"], |_| false);
        let empty_array: Vec<&str> = Vec::new();

        assert_eq!(
            online_shop_js_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec![
                "online-shop/shoes/shoe/brown.js", "online-shop/shoes/brown.js"
            ]
        );
        assert_eq!(
            online_shop_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            vec![
                "online-shop/shoes/shoe/brown.js", "online-shop/shoes/brown.js", "online-shop/details.hbs"
            ]
        );
        assert_eq!(
            shoes_js_files.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>(),
            empty_array
        );
        return fs::remove_dir_all("online-shop");
    }
}

