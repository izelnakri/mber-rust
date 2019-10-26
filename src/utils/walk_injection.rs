use serde::{Deserialize, Serialize};
// use serde_json;
use std::collections::HashMap;
// use std::borrow::BorrowMut;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum KeyValue {
    Vec(Vec<u8>),
    RefCell(HashMap<String, KeyValue>),
}

pub fn lookup_for_extensions(hashmap: HashMap<String, String>, extensions: Vec<&str>) -> HashMap<String, String> {
    return hashmap.into_iter()
        .filter(|(key, _value)| extensions.iter().any(|extension| key.ends_with(extension)))
        .collect(); // as hashmap
}

// pub fn lookup_for_extensions_and_predicate<F>(injection_str: &str, extensions: Vec<&str>, predicate: F) -> HashMap<String, String> // TODO: add predicate? starts_with?
//     where F: Fn(&str) -> bool {
//     // return WalkDir::new(directory).into_iter().filter_map(|e| {
//     //     let entry = e.unwrap();
//     //     let entry_correct_extension = extensions.iter()
//     //         .any(|extension| entry.file_name().to_str().unwrap().ends_with(extension));


//     //     return match entry_correct_extension && predicate(&entry) {
//     //         true => Some(entry.into_path()),
//     //         false => None
//     //     };
//     // }).collect();
// }

pub fn flatten_fs_hashmap(fs_hashmap: HashMap<String, KeyValue>, parent_folders: Vec<String>) -> HashMap<String, String> {
    return fs_hashmap.into_iter().fold(HashMap::new(), |mut result, (key, value)| {
        let mut new_parent_folders = parent_folders.to_vec();

        match value {
            KeyValue::RefCell(nested_hashmap) => {
                new_parent_folders.push(key.to_string());

                return result.into_iter().chain(flatten_fs_hashmap(nested_hashmap, new_parent_folders)).collect();
            },
            _ => {
                new_parent_folders.append(&mut vec![key]);

                if let KeyValue::Vec(value_in_string) = value {
                    result.insert(new_parent_folders.join("/"), String::from_utf8(value_in_string.to_vec()).unwrap());
                }

                return result;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
    use super::*;
    use super::super::super::injections::documentation;

    #[test]
    fn lookup_for_extensions_works_for_js_and_hbs() -> io::Result<()> {
        let documentation_hashmap: HashMap<String, KeyValue> = serde_json::from_str(documentation::as_str()).unwrap();
        let flat_documentation_hashmap = flatten_fs_hashmap(documentation_hashmap, vec![]);
        let result = lookup_for_extensions(flat_documentation_hashmap.clone(), vec!["js", "hbs"]); // TODO: maybe scope it to /src only?
        let result_keys = result.keys().map(|k| k.as_str()).collect::<Vec<&str>>();

        assert_eq!(result_keys.len(), 35);
        assert!(result_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-demo/example/component.js"));
        assert!(result_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/template.hbs"));

        let sub_directory: HashMap<String, String> = flat_documentation_hashmap.into_iter()
            .filter(|(key, _)| key.starts_with("_vendor/mber-documentation/src/ui/components/docs-viewer/navigation"))
            .collect();
        let sub_directory_result = lookup_for_extensions(sub_directory.clone(), vec!["js", "hbs"]);
        let sub_directory_keys = sub_directory_result.keys().map(|k| k.as_str()).collect::<Vec<&str>>();

        assert_eq!(sub_directory_keys.len(), 6);
        assert!(sub_directory_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/component.js"));
        assert!(sub_directory_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/template.hbs"));
        assert!(sub_directory_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/title/template.hbs"));
        assert!(sub_directory_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/link/component.js"));
        assert!(sub_directory_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/link/template.hbs"));
        assert!(sub_directory_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/title/component.js"));

        let sub_directory_hbs_result = lookup_for_extensions(sub_directory, vec!["hbs"]);
        let sub_directory_hbs_keys = sub_directory_hbs_result.keys().map(|k| k.as_str()).collect::<Vec<&str>>();

        assert_eq!(sub_directory_hbs_keys.len(), 3);
        assert!(sub_directory_hbs_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/template.hbs"));
        assert!(sub_directory_hbs_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/title/template.hbs"));
        assert!(sub_directory_hbs_keys.contains(&"_vendor/mber-documentation/src/ui/components/docs-viewer/navigation/link/template.hbs"));

        Ok(())
    }

    // #[test]
    // fn lookup_for_extensions_works_when_there_are_no_reference_files() -> io::Result<()> {
    // } // NOTE: run on .hbs on some sub directory

    // #[test]
    // fn lookup_for_extensions_and_predicate_works_for_js_and_hbs() -> io::Result<()> {
    // }

    // #[test]
    // fn lookup_for_extensions_and_predicate_works_when_there_are_no_reference_files() -> io::Result<()> {
    // }
}
