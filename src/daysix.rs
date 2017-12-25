use std::collections::HashSet;
use std::str::FromStr;
use std::iter::FromIterator;

//static TESTBLOCKS: &'static str = include_str!("daysix.txt");
static TESTBLOCKS: &'static str = include_str!("daysixtest.txt");

pub fn day() -> String {
    let mut so_far: HashSet<Vec<i64>> = HashSet::new();
    let mut prev_len = so_far.len();
    let mut v: Vec<i64> = Vec::from_iter(TESTBLOCKS.split_whitespace().filter_map(|j| { i64::from_str(j).ok() }));

    so_far.insert(v.clone());

    while prev_len != so_far.len() {
        prev_len = so_far.len();
        if let Some((cur_index, _)) = get_max_with_index(&v) {
            distribute(&mut v, cur_index);
        }
        println!("v: {:?}", v);
        so_far.insert(v.clone());
    }
    so_far.len().to_string()
}

fn get_max_with_index(v: &Vec<i64>) -> Option<(usize, i64)> {
    v.iter().enumerate().fold(None, |o_max, (idx, &val)|{
        if let Some((_, cur_max)) = o_max {
            if cur_max > val {
                o_max
            } else {
                Some((idx, val))
            }
        } else {
            Some((idx, val))
        }
    })
}

fn distribute(v: &mut Vec<i64>, idx: usize) {
    let mut cur_idx = idx;
    let mut cur_val = v[idx];
    let len = v.len();

    //set current idx to 0v
    v[cur_idx] = 0;
    while cur_val > 0 {
        cur_idx += 1;
        if cur_idx >= len {
            cur_idx = 0;
        }
        v[cur_idx] += 1;
        cur_val -= 1;
    }
}
