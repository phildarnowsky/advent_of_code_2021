use std::fs;

fn main() {
    let contents = String::from(fs::read_to_string("input").expect("couldn't open input file"));
    let lines = contents.lines();
    let mut integers = lines.map(|x| x.parse::<u32>().expect("bad integer format"));

    let mut previous_value: u32 = integers.next().expect("empty list not valid");
    let mut count: u32 = 0;

    for value in integers {
        if value > previous_value {
            count += 1;
        }
        previous_value = value;
    }
    println!("{}", count);
}
