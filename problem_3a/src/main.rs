fn main() {
    let mut counts: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut n_lines = 0;
    let text = std::fs::read_to_string("input").expect("couldn't read file");
    let lines = text.lines();
    for line in lines {
        update_counts(line, &mut counts);
        n_lines += 1;
    }

    let gamma = calculate_gamma(counts, n_lines);
    let epsilon = 2_u32.pow(12) - 1 - gamma;

    println!("{}", gamma * epsilon);
}

fn update_counts(line: &str, counts: &mut Vec<u32>) {
    for (i, char) in line.chars().enumerate() {
        if char == '1' {
            counts[i] += 1
        }
    }
}

fn calculate_gamma(mut counts: Vec<u32>, n_lines: u32) -> u32 {
    let mut power_of_two = 1;
    let mut total = 0;
    let threshold = n_lines / 2;

    counts.reverse();
    for count in counts {
        if count > threshold {
            total += power_of_two;
        }
        power_of_two *= 2;
    }

    total
}
