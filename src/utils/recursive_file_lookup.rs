use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn with_extensions(directory: &Path, extensions: Vec<&str>) -> Vec<PathBuf> {
    return WalkDir::new(directory).into_iter().filter_map(|e| {
        let entry = e.unwrap();

        return match extensions.iter().any(|extension| entry.file_name().to_str().unwrap().ends_with(extension)) {
            true => Some(entry.into_path()),
            false => None
        };
    }).collect();
}

// NOTE: maybe in future:
// pub fn with_extensions_and_predicate(directory: &Path, extensions: &Vec, filter: Fn) -> Vec<Path> {

// }

#[cfg(test)]
mod tests {
    use std::io;
    use std::env;
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
    fn with_extensions_works_for_js_and_hbs() -> io::Result<()> {
        setup()?;
      // const onlineShopJSFiles = await lookup(`${CWD}/online-shop`, 'js');
      // const onlineShopHBSFiles = await lookup(`${CWD}/online-shop`, 'hbs');
      // const onlineShopFiles = await lookup(`${CWD}/online-shop`, ['js', 'hbs']);
      // const shoesJSFiles = await lookup(`${CWD}/online-shop/shoes`, 'js');
      // const shoesHBSFiles = await lookup(`${CWD}/online-shop/shoes`, 'hbs');
      // const shoesFiles = await lookup(`${CWD}/online-shop/shoes`, ['js', 'hbs']);
      // const shoeFiles = await lookup(`${CWD}/online-shop/shoes/shoe`, ['js', 'hbs']);

        let current_dir = env::current_dir()?;
        let online_shop_directory = Path::new("online-shop");
        let shoes_directory = Path::new("online-shop/shoes");
        let shoe_directory = Path::new("online-shop/shoes/shoe");
        let online_shop_js_files = with_extensions(online_shop_directory, vec!["js"]);
        let online_shop_hbs_files = with_extensions(online_shop_directory, vec!["hbs"]);
        let online_shop_files = with_extensions(online_shop_directory, vec!["hbs", "js"]);
        let shoes_js_files = with_extensions(shoes_directory, vec!["js"]);
        let shoes_hbs_files = with_extensions(shoes_directory, vec!["hbs"]);
        let shoes_files = with_extensions(shoes_directory, vec!["js", "hbs"]);
        let shoe_files = with_extensions(shoe_directory, vec!["js", "hbs"]);

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
    fn with_extensions_works_when_there_are_no_reference_files() -> io::Result<()> {
        setup()?;

        let current_dir = env::current_dir()?;
        let online_shop_directory = Path::new("online-shop");
        let shoe_directory = Path::new("online-shop/shoes/shoe");
        let shoe_hbs_files = with_extensions(shoe_directory, vec!["hbs"]);
        let online_shop_txt_files = with_extensions(shoe_directory, vec!["txt"]);
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
}

