use std::io;
use std::fs;
use std::path::PathBuf;
use inflector::cases::camelcase::to_camel_case;
use inflector::cases::snakecase::to_snake_case;
use super::super::utils;

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let util_code = "export default function() {
      return true;
    }";
    let file_name = to_snake_case(&input_name).replace("_", "-");
    let target_folder = format!("{}/src/utils", project_root.to_str().unwrap());
    let target_file_path = format!("{}/{}", target_folder, file_name);

    fs::create_dir_all(&target_folder)?;
    utils::write_file_if_not_exists(format!("{}.js", &target_file_path), util_code, &project_root)?;

    let test_code = get_test_code(file_name, application_name);

    return utils::write_file_if_not_exists(format!("{}-test.js", target_file_path), test_code.as_str(), &project_root);
}

fn get_test_code(file_name: String, application_name: &str) -> String {
    let import_name = to_camel_case(&file_name);

    return format!("import {{ module, test }} from 'qunit';
import {{ setupTest }} from '{}/tests/helpers';
import {} from './{}';

module('Unit | Util | {}', function(hooks) {{
  setupTest(hooks);

  // Replace this with your real tests.
  test('it works', function (assert) {{
    let result = {}();

    assert.ok(result);
  }});
}});", application_name, import_name, file_name, file_name, import_name);
}
