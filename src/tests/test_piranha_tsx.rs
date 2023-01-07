/*
Copyright (c) 2022 Uber Technologies, Inc.

 <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 except in compliance with the License. You may obtain a copy of the License at
 <p>http://www.apache.org/licenses/LICENSE-2.0

 <p>Unless required by applicable law or agreed to in writing, software distributed under the
 License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 express or implied. See the License for the specific language governing permissions and
 limitations under the License.
*/

use crate::models::default_configs::TSX;

use super::{get_piranha_arguments_for_test, initialize, run_match_test};

#[test]
fn test_ts_match_only_find_fors() {
  initialize();
  let relative_path_to_tests = &format!("{}/{}/{}", TSX, "structural_find", "find_jsx_elements");
  run_match_test(
    get_piranha_arguments_for_test(relative_path_to_tests, TSX),
    4,
  );
}

#[test]
fn test_ts_match_only_find_fors_within_functions() {
  initialize();
  let relative_path_to_tests = &format!(
    "{}/{}/{}",
    TSX, "structural_find", "find_props_identifiers_within_b_jsx_elements"
  );
  run_match_test(
    get_piranha_arguments_for_test(relative_path_to_tests, TSX),
    2,
  );
}

#[test]
fn test_ts_match_only_find_fors_within_functions_not_within_whiles() {
  initialize();
  let relative_path_to_tests = &format!(
    "{}/{}/{}",
    TSX, "structural_find", "find_props_identifiers_within_variable_declarators_not_within_divs"
  );
  run_match_test(
    get_piranha_arguments_for_test(relative_path_to_tests, TSX),
    2,
  );
}
