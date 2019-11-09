use select::document::Document;
use select::predicate::{Name};
use regex::Regex;

pub fn find_internal_assets_from_html(document: &Document) -> (Vec<&str>, Vec<&str>) {
    let script_tags = document.find(Name("script")).fold(Vec::new(), |mut result, node| {
        let src = node.attr("src").unwrap_or("");

        if src != "" && !uri_is_external(src) {
            result.push(src);
        }

        return result;
    });
    let style_tags = document.find(Name("link")).fold(Vec::new(), |mut result, node| {
        let href = node.attr("href").unwrap_or("");

        if href != "" && !uri_is_external(href) {
            result.push(href);
        }

        return result;
    });

    return (script_tags, style_tags);
}

pub fn uri_is_external(reference: &str) -> bool {
    let reference = reference.to_lowercase();

    return ((reference.starts_with("http://") || reference.starts_with("https://")) || reference.starts_with("//")) &&
        !reference.starts_with("http://localhost") &&
        !reference.starts_with("https://localhost") &&
        Regex::new(r"(?i)^(?:[a-z]+:)?//").unwrap().is_match(&reference);
}

// pub fn rewrite_html_assets(document: &Document, asset_map: HashMap<String, String>) {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_internal_assets_from_html_works_for_only_js() {
        let html = r##"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta http-equiv="X-UA-Compatible" content="IE=edge">

            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">

            <meta name="description" content="">
            <!-- EMBER_CLI_FASTBOOT_TITLE --><!-- EMBER_CLI_FASTBOOT_HEAD -->
          </head>
          <body>
            <!-- EMBER_CLI_FASTBOOT_BODY -->

            <script src="https://markets.live/vendor.js"></script>
            <script src="/assets/vendor.js"></script>
            <script src="/assets/application.js"></script>
          </body>
        </html>
        "##;
        let document = Document::from(html);
        let (scripts, styles) = find_internal_assets_from_html(&document);
        let empty_vec: Vec<&str> = Vec::new();

        assert_eq!(scripts, vec!["/assets/vendor.js", "/assets/application.js"]);
        assert_eq!(styles, empty_vec);
    }

    #[test]
    fn find_internal_assets_from_html_works_for_only_css() {
        let html = r##"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta http-equiv="X-UA-Compatible" content="IE=edge">

            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">

            <meta name="description" content="">
            <link rel="stylesheet" href="https://markets.live/assets/vendor.css">
            <link rel="stylesheet" href="/assets/vendor-styles.css">
            <link rel="stylesheet" href="/assets/application-styles.css">
          </head>
          <body>
          </body>
        </html>
        "##;
        let document = Document::from(html);
        let (scripts, styles) = find_internal_assets_from_html(&document);
        let empty_vec: Vec<&str> = Vec::new();

        assert_eq!(scripts, empty_vec);
        assert_eq!(styles, vec!["/assets/vendor-styles.css", "/assets/application-styles.css"]);
    }

    #[test]
    fn find_internal_assets_from_html_works_for_html_files_with_bunch_of_different_js_and_css() {
        let html = r##"
        <!DOCTYPE html>
        <html>
          <head>
            <meta charset="utf-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="description" content="">
            <meta name="viewport" content="width=device-width, initial-scale=1">

            <link rel="stylesheet" href="https://markets.live/assets/application.css">
            <link rel="stylesheet" href="//izelnakri.com/assets/application.css">
            <link rel="stylesheet" href="/assets/application.css">
            <link rel="stylesheet" href="assets/test-support.css">

            <script src="/assets/init.js"></script>
          </head>
          <body style="margin: 0;">
            <div id="qunit"></div>
            <div id="qunit-fixture"></div>

            <div id="ember-testing-container">
              <div id="ember-testing"></div>
            </div>

            <script src="https://markets.live/assets/all.js"></script>
            <script src="//izelnakri.com/assets/all.js"></script>
            <script src="/assets/vendor.js"></script>
            <script type="text/javascript">
              document.addEventListener('keydown', function(event) {
                event.keyCode === 80 ? document.querySelector('#qunit-abort-tests-button').click() : null;
              });
            </script>
            <script src="/assets/test-support.js"></script>
            <script src="/assets/application.js"></script>
            <script src="/assets/tests.js"></script>
          </body>
        </html>
        "##;
        let document = Document::from(html);
        let (scripts, styles) = find_internal_assets_from_html(&document);

        assert_eq!(scripts, vec![
            "/assets/init.js", "/assets/vendor.js", "/assets/test-support.js", "/assets/application.js",
            "/assets/tests.js"
        ]);
        assert_eq!(styles, vec!["/assets/application.css", "assets/test-support.css"]);
    }

    #[test]
    fn str_is_absolute_url_returns_false_for_local_references() {
        assert_eq!(uri_is_external("http://localhost:8000/index.js"), false);
        assert_eq!(uri_is_external("https://localhost:8000/index.js"), false);
        assert_eq!(uri_is_external("/documentation.js"), false);
        assert_eq!(uri_is_external("memserver.js"), false);

        assert_eq!(uri_is_external("HTTP://LOCALHOST:8000/INDEX.JS"), false);
        assert_eq!(uri_is_external("HTTPS://LOCALHOST:8000/INDEX.JS"), false);
        assert_eq!(uri_is_external("/DOCUMENTATION.JS"), false);
        assert_eq!(uri_is_external("MEMSERVER.JS"), false);
    }

    #[test]
    fn str_is_absolute_url_returns_true_for_external_references() {
        assert_eq!(uri_is_external("https://markets.live/application.js"), true);
        assert_eq!(uri_is_external("https://www.markets.live/application.js"), true);
        assert_eq!(uri_is_external("http://izelnakri.com/vendor.js"), true);
        assert_eq!(uri_is_external("http://www.izelnakri.com/vendor.js"), true);
        assert_eq!(uri_is_external("//us1.cdnjs.com/underscore.js"), true);

        assert_eq!(uri_is_external("HTTPS://MARKETS.LIVE/APPLICATION.JS"), true);
        assert_eq!(uri_is_external("HTTPS://WWW.MARKETS.LIVE/APPLICATION.JS"), true);
        assert_eq!(uri_is_external("HTTP://IZELNAKRI.COM/VENDOR.JS"), true);
        assert_eq!(uri_is_external("HTTP://WWW.IZELNAKRI.COM/VENDOR.JS"), true);
        assert_eq!(uri_is_external("//US1.CDNJS.COM/UNDERSCORE.JS"), true);
    }
}
