use std::io;
use std::fs;
use std::path::PathBuf;
use super::super::utils;

const SERVICE_CODE: &'static str = "import Service from '@ember/service';

export default Service.extend({
});";

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let target_folder = format!("{}/src/services", project_root.to_str().unwrap());
    let target_file_path = format!("{}/{}", target_folder, input_name);

    fs::create_dir_all(&target_folder)?;
    utils::write_file_if_not_exists(format!("{}.js", &target_file_path), SERVICE_CODE, &project_root)?;

    let test_code = get_test_code(input_name, application_name);

    return utils::write_file_if_not_exists(format!("{}-test.js", target_file_path), test_code.as_str(), &project_root);
}

fn get_test_code(input_name: String, application_name: &str) -> String {
  return format!("import {{ module, test }} from 'qunit';
import {{ setupTest }} from '{}/tests/helpers';

module('Unit | Service | {}', function(hooks) {{
  setupTest(hooks);

  // Replace this with your real tests.
  test('it exists', function(assert) {{
    let service = this.owner.lookup('service:{}');

    assert.ok(service);
  }});
}});", application_name, input_name, input_name);
}
