enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_command(line: &str) -> Result<Command, ()> {
    let mut words = line.split_whitespace();
    let raw_command: String = words.next().expect("missing raw command").to_string();
    let n: u32 = words
        .next()
        .expect("missing distance")
        .parse()
        .expect("bad integer format");

    if raw_command.eq("forward") {
        return Ok(Command::Forward(n));
    }
    if raw_command.eq("down") {
        return Ok(Command::Down(n));
    }
    if raw_command.eq("up") {
        return Ok(Command::Up(n));
    }
    Err(())
}

fn main() {
    let text = std::fs::read_to_string("input").expect("couldn't read file");
    let lines = text.lines();
    let commands: Vec<Command> = lines
        .map(|line| parse_command(line).expect("bad command"))
        .collect();

    let mut aim = 0;
    let mut y = 0;
    let mut z = 0;

    for command in commands {
        match command {
            Command::Forward(n) => {
                y += n;
                z += n * aim
            }
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }

    println!("{}", y * z);
}
