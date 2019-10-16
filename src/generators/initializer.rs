use std::io;
use std::fs;
use std::path::PathBuf;
use inflector::cases::snakecase::to_snake_case;
use super::super::utils;

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let initializer_code = "export function initialize(/* application */) {
      // application.inject('route', 'foo', 'service:foo');
    }

    export default {
      initialize
    };";

    let file_name = to_snake_case(&input_name).replace("_", "-");
    let target_folder = format!("{}/src/init/initializers", project_root.to_str().unwrap());
    let target_file_path = format!("{}/{}", target_folder, file_name);

    fs::create_dir_all(target_folder)?;
    utils::write_file_if_not_exists(format!("{}.js", target_file_path), initializer_code, &project_root)?;

    let test_code = get_test_code(file_name, application_name);

    return utils::write_file_if_not_exists(format!("{}-test.js", target_file_path), test_code.as_str(), &project_root);
}

fn get_test_code(file_name: String, application_name: &str) -> String {
    return format!("import Application from '@ember/application';
import {{ module, test }} from 'qunit';
import {{ run }} from '@ember/runloop';
import {{ setupTest }} from '{}/tests/helpers';
import {{ initialize }} from './{}';

module('Unit | Initializer | {}', function(hooks) {{
  setupTest(hooks);

  hooks.beforeEach(function() {{
    this.TestApplication = Application.extend();
    this.TestApplication.initializer({{
      name: 'initializer under test',
      initialize
    }});
    this.application = this.TestApplication.create({{ autoboot: false }});
  }});

  hooks.afterEach(function() {{
    run(this.application, 'destroy');
  }});

  // Replace this with your real tests.
  test('it works', async function(assert) {{
    await this.application.boot();

    assert.ok(true);
  }});
}});", application_name, file_name, file_name);
}
