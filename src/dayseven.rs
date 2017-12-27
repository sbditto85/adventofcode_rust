use std::collections::HashMap;

//static TESTINFO: &'static str = include_str!("dayseven.txt");
static TESTINFO: &'static str = include_str!("dayseventest.txt");

struct Program {
    name: String,
    parent: Option<String>
}

//named!(take4, take!(4));

pub fn day() -> String {
    let mut programs: HashMap<String, Program> = HashMap::new();

    TESTINFO.lines();
    "".to_string()
}
