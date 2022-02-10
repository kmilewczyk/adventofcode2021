pub mod file {
    pub fn read_lines(path: &str) -> Result<std::io::Lines<std::io::BufReader<std::fs::File>>, String> {
        use std::io::BufRead;

        let reader = {
            let file = std::fs::File::open(path).map_err(|err| format!("Couldn't open file '{}'. Reason: {}", path, err))?;
            std::io::BufReader::new(file)
        };

        Ok(reader.lines())
    }
}