use std::fs;

fn main() {
    let contents = String::from(fs::read_to_string("input").expect("couldn't open input file"));
    let lines = contents.lines();
    let mut integers = lines.map(|x| x.parse::<u32>().expect("bad integer format"));

    let mut v1: u32 = integers.next().expect("list too short");
    let mut v2: u32 = integers.next().expect("list too short");
    let mut v3: u32 = integers.next().expect("list too short");
    let mut count: u32 = 0;

    for v4 in integers {
        let previous_sum = v1 + v2 + v3;
        let sum = v2 + v3 + v4;
        if sum > previous_sum {
            count += 1;
        }
        v1 = v2;
        v2 = v3;
        v3 = v4;
    }
    println!("{}", count);
}

