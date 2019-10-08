use std::io;
use std::fs;
use std::path::PathBuf;
use super::super::utils;

const MODEL_CODE: &'static str = "import DS from 'ember-data';

export default DS.Model.extend({

});";

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let target_folder = format!("{}/src/data/models/{}", project_root.to_str().unwrap(), input_name);

    fs::create_dir_all(&target_folder)?;
    utils::write_file_if_not_exists(format!("{}/model.js", &target_folder), MODEL_CODE, &project_root)?;

    let test_code = get_test_code(input_name, application_name);

    return utils::write_file_if_not_exists(format!("{}/unit-test.js", &target_folder), test_code.as_str(), &project_root);
}

fn get_test_code(input_name: String, application_name: &str) -> String {
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
}});", application_name, input_name, input_name);
}
