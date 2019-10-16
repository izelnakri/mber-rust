use std::io;
use std::fs;
use std::path::PathBuf;
use inflector::cases::snakecase::to_snake_case;
use inflector::string::singularize::to_singular;
use super::super::utils;

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let component_code = "import Component from '@ember/component';

    export default Component.extend({
    });";
    let folder_name = to_snake_case(&to_singular(&input_name)).replace("_", "-");
    let target_folder = format!("{}/src/ui/components/{}", project_root.to_string_lossy(), folder_name);

    fs::create_dir_all(&target_folder)?;
    utils::write_file_if_not_exists(format!("{}/component.js", target_folder), component_code, &project_root)?;
    utils::write_file_if_not_exists(format!("{}/template.hbs", target_folder), "{{yield}}", &project_root)?;
    utils::write_file_if_not_exists(format!("{}/styles.scss", target_folder), "", &project_root)?;
    utils::write_file_if_not_exists(
        format!("{}/integration-test.js", target_folder),
        get_integration_test_code(folder_name, application_name).as_str(),
        &project_root
    )
}

fn get_integration_test_code(folder_name: String, application_name: &str) -> String {
    return format!("import {{ module, test }} from 'qunit';
import {{ render }} from '@ember/test-helpers';
import hbs from 'htmlbars-inline-precompile';
import {{ setupRenderingTest }} from '{}/tests/helpers';

module('Integration | Component | {}', function(hooks) {{
  setupRenderingTest(hooks);

  test('it renders', async function(assert) {{
    // Set any properties with this.set('myProperty', 'value');
    // Handle any actions with this.set('myAction', function(val) {{ ... }});

    await render(hbs`<{}/>`);

    assert.equal(this.element.textContent.trim(), '');

    // Template block usage:
    await render(hbs`
      <{}>
        template block text
      </{}>
    `);

    assert.equal(this.element.textContent.trim(), 'template block text');
  }});
}});", folder_name, application_name, folder_name, folder_name, folder_name);
}
