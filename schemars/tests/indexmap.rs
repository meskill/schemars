mod util;
use std::hash::RandomState;

use indexmap2::{IndexMap, IndexSet};
use schemars::JsonSchema;
use util::*;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct IndexMapTypes {
    map: IndexMap<i32, bool, RandomState>,
    set: IndexSet<isize, RandomState>,
}

#[test]
fn indexmap_types() -> TestResult {
    test_default_generated_schema::<IndexMapTypes>("indexmap")
}
