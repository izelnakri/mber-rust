pub fn find_internal_assets_from_html(document: &Document) -> (Vec<&str>, Vec<&str>) {
    // NOTE: also add scripts and links that are not absolute urls: new RegExp('^(?:[a-z]+:)?//', 'i');
    let body_node = document.find(Name("body"));
    let script_tags = body_node.find(Name("script")).filter(|node| {
        true
    });
    let style_tags = body_node.find(Name("link")).filter(|node| {
        true
    });

    return (script_tags, style_tags);
}

// NOTE: also check hyper maybe instead?
pub fn uri_is_absolute_url(reference: &str) {
    // new RegExp('^(?:[a-z]+:)?//', 'i');
    return Regex::new(r"(?i)^(?:[a-z]+:)?//").is_match(reference);
}

// pub fn rewrite_html_assets(document: &Document, asset_map: HashMap<String, String>) {

// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn find_internal_assets_from_html_works_for_only_js() {
//     }
//     #[test]
//     fn find_internal_assets_from_html_works_for_only_css() {
//     }
//     #[test]
//     fn find_internal_assets_from_html_works_for_html_files_with_bunch_of_different_js_and_css() {
//     }
//     #[test]
//     fn str_is_absolute_url_returns_false_for_local_references() { // localhost or relative
//     }
//     #[test]
//     fn str_is_absolute_url_returns_true_for_external_references() { // http, https and ones start with //
//     }
// }
