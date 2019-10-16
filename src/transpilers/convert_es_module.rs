use std::path::Path;

// TODO: turn return types to Result
pub fn from_file<'a>(file: &Path, minify: bool) -> &'a str {
    return "function(){{ }};";
}

pub fn from_string<'a>(code_string: &'a str, file_name: &'a str, minify: bool) -> &'a str {
    return "function(){{ }};";
}

#[cfg(test)]
mod tests {
    use std::io;
    // use std::fs;
    use super::*;

    #[test]
    fn convert_es_module_from_string_works() -> io::Result<()> {
        let code = "import EmberRouter from '@ember/routing/router';
import DocumentationRouter from 'mber-documentation';
import ENV from '../config/environment';

const Router = EmberRouter.extend({{
  location: ENV.locationType,
  rootURL: ENV.rootURL
}});

Router.map(function() {{
  this.route('index', {{ path: '/' }});

  if (ENV.documentation && ENV.documentation.enabled) {{
    DocumentationRouter.apply(this, [ENV]);
  }}

  this.route('not-found', {{ path: '/*path' }});
}});

export default Router;";
        let expected_output = "";

        // assert_eq!(from_string(code, "src/router"), expected_output);

        Ok(())
    }

    // #[test]
    // fn convert_es_module_from_file_works() -> io::Result<()> {
    // }
}
