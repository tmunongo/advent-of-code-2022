use color_eyre::eyre::Context;

struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for file {:?}: {}", self.path, self.inner)
    }
}

fn main() -> color_eyre::Result<()> {
    // equivalent to writing, but now unwrap handles the two cases for us
    color_eyre::install()?;
    let mut max = 0;

    for group in include_str!("input.txt")
    .replace("\r\n", "\n")
    .split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            sum += value;
            // println!(" - {value}");
        }
        if sum > max {
            max = sum;
        }
    }
    println!("Group has sum {max}");

    Ok(())
}

fn read_input_file() -> color_eyre::Result<String> {
    // let path = "src/inpt.txt";
    // match std::fs::read_to_string(path) {
    //     Ok(s) => Ok(s),
    //     Err(e) => Err(PathedIoError { path: path.into(), inner: e })
    // }
    // expect("Couldn't read input file src/input.txt")

    // with fs-err
    // fs_err::read_to_string("src/inpt.txt")
    let input = std::fs::read_to_string("src/inpt.txt").wrap_err("reading src/input.txt failed")?;
    Ok(input)
}