#[macro_export]
macro_rules! read_input {
    ($path:literal) => {{
        use std::io::BufRead;
        std::io::BufReader::new(std::fs::File::open("src/data/".to_owned() + $path).unwrap())
            .lines()
            .map(|x| x.unwrap())
    }};
}
