use std::str::FromStr;
use std::iter::FromIterator;

static TESTJUMPS: &'static str = include_str!("dayfive.txt");
//static TESTJUMPS: &'static str = include_str!("dayfivetest.txt");

pub fn day() -> String {
    let mut v: Vec<i64> = Vec::from_iter(TESTJUMPS.lines().filter_map(|j| { i64::from_str(j).ok() }));
    let vlen = v.len();
    let mut num_steps = 0;
    let mut cur_index = 0;
    while cur_index < vlen {
        let cur_val = v[cur_index];
        v[cur_index] = cur_val + 1;
        if cur_val > 0 {
            cur_index += cur_val as usize;
        } else {
            cur_index -= (cur_val * -1) as usize;
        }
        num_steps += 1;
    }
    num_steps.to_string()
}
