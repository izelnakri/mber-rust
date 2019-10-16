use std::io;
use std::fs;
use std::path::PathBuf;
use inflector::cases::snakecase::to_snake_case;
use inflector::string::singularize::to_singular;
use super::super::utils;

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let model_code = "import DS from 'ember-data';

    export default DS.Model.extend({

    });";
    let folder_name = to_snake_case(&to_singular(&input_name)).replace("_", "-");
    let target_folder = format!("{}/src/data/models/{}", project_root.to_str().unwrap(), folder_name);

    fs::create_dir_all(&target_folder)?;
    utils::write_file_if_not_exists(format!("{}/model.js", &target_folder), model_code, &project_root)?;

    let test_code = get_test_code(folder_name, application_name);

    return utils::write_file_if_not_exists(format!("{}/unit-test.js", &target_folder), test_code.as_str(), &project_root);
}

fn get_test_code(folder_name: String, application_name: &str) -> String {
  return format!("import {{ module, test }} from 'qunit';
import {{ setupTest }} from '{}/tests/helpers';

module('Unit | Model | {}', function(hooks) {{
  setupTest(hooks);

  // Replace this with your real tests.
  test('it exists', function(assert) {{
    let store = this.owner.lookup('service:store');
    let model = store.createRecord('{}', {{/}});

    assert.ok(model);
  }});
}});", application_name, folder_name, folder_name);
}
