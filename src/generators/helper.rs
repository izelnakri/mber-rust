use std::io;
use std::path::PathBuf;
use super::super::utils;

const HELPER_CODE: &'static str = "import Helper from '@ember/component/helper'

export let helper = Helper.helper((params/*, hash*/) => {
  return params;
});";

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let target_file_path = format!("{}/src/ui/components/{}", project_root.to_str().unwrap(), input_name);

    utils::write_file_if_not_exists(format!("{}.js", target_file_path), HELPER_CODE, &project_root)?;

    let test_code = get_test_code(input_name, &application_name);

    return utils::write_file_if_not_exists(format!("{}-test.js", target_file_path), test_code.as_str(), &project_root);
}

fn get_test_code(input_name: String, application_name: &str) -> String {
    return format!("import {{ module, test }} from 'qunit';
import {{ render }} from '@ember/test-helpers';
import hbs from 'htmlbars-inline-precompile';
import {{ setupRenderingTest }} from '{}/tests/helpers';

module('Integration | Helper | {}', function(hooks) {{
  setupRenderingTest(hooks);

  // Replace this with your real tests.
  test('it renders', async function(assert) {{
    this.set('inputValue', '1234');

    await render(hbs`{{{{{} inputValue}}}}`);

    assert.equal(this.element.textContent.trim(), '1234');
  }});
}});", application_name, input_name, input_name);
}
