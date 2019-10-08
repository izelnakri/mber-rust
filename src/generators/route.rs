use std::io;
use std::fs;
use std::path::PathBuf;
use inflector::cases::snakecase::to_snake_case;
use super::super::utils;

const ROUTE_CODE: &'static str = "import Route from '@ember/routing/route';

export default Route.extend({
});";

pub fn generate(input_name: String, application_name: &str, project_root: PathBuf) -> io::Result<()> {
    let folder_name = to_snake_case(&input_name).replace("_", "-");
    let target_folder = format!("{}/src/ui/routes/{}", project_root.to_str().unwrap(), folder_name);

    fs::create_dir_all(&target_folder)?;
    utils::write_file_if_not_exists(format!("{}/route.js", &target_folder), ROUTE_CODE, &project_root)?;
    utils::write_file_if_not_exists(format!("{}/template.hbs", &target_folder), "{{{{outlet}}}}", &project_root)?;
    utils::write_file_if_not_exists(format!("{}/styles.scss", &target_folder), "", &project_root)?;
    utils::write_file_if_not_exists(
        format!("{}/acceptance-test.js", &target_folder),
        get_acceptance_test_code(folder_name.clone(), application_name).as_str(),
        &project_root
    )?;

    return utils::write_file_if_not_exists(
        format!("{}/unit-test.js", &target_folder),
        get_unit_test_code(folder_name, application_name).as_str(),
        &project_root
    );
}

fn get_acceptance_test_code(folder_name: String, application_name: &str) -> String {
  return format!("import {{ module, test }} from 'qunit';
import {{ visit, currentURL }} from '@ember/test-helpers';
import {{ setupApplicationTest }} from '{}/tests/helpers';

module('Acceptance | {}', function(hooks) {{
  setupApplicationTest(hooks);

  // test('visiting /route', async function(assert) {{
  //   await visit('/route');
  //
  //   assert.equal(currentURL(), '/route');
  // }});
}});", application_name, folder_name);
}

fn get_unit_test_code(folder_name: String, application_name: &str) -> String {
  return format!("import {{ module, test }} from 'qunit';
import {{ setupTest }} from '{}/tests/helpers';

module('Unit | Route | {}', function(hooks) {{
  setupTest(hooks);

  test('it exists', function(assert) {{
    let route = this.owner.lookup('route:{}');

    assert.ok(route);
  }});
}});", application_name, folder_name, folder_name);
}
