use std::io;
use std::fs;
use std::path::PathBuf;
use inflector::cases::classcase::to_class_case;
use inflector::cases::snakecase::to_snake_case;
use super::super::utils;

const MIXIN_CODE : &'static str = "import Mixin from '@ember/object/mixin';

export default Mixin.create({
});";

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let file_name = to_snake_case(&input_name).replace("_", "-");
    let target_folder = format!("{}/src/utils/mixins", project_root.to_str().unwrap());
    let target_file_path = format!("{}/{}", target_folder, file_name);

    fs::create_dir_all(target_folder)?;
    utils::write_file_if_not_exists(format!("{}.js", target_file_path), MIXIN_CODE, &project_root)?;

    let test_code = get_test_code(file_name, application_name);

    return utils::write_file_if_not_exists(format!("{}-test.js", target_file_path), test_code.as_str(), &project_root);
}

fn get_test_code(file_name: String, application_name: &str) -> String {
    let class_name = to_class_case(&file_name.clone());

    return format!("import {{ module, test }} from 'qunit';
import EmberObject from '@ember/object';
import {{ setupTest }} from '{}/tests/helpers';
import {}Mixin from './{}';

module('Unit | Mixin | {}', function(hooks) {{
  setupTest(hooks);

  // Replace this with your real tests.
  test('it works', function (assert) {{
    let {}Object = EmberObject.extend(AuthMixin);
    let subject = {}Mixin.create();

    assert.ok(subject);
  }});
}});", application_name, class_name, file_name, file_name, class_name, class_name);
}
