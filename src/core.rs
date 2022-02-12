pub mod file {
    

    pub fn read_lines(path: &str) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
        use std::io::BufRead;

        let reader = {
            let file = std::fs::File::open(path)?;
            std::io::BufReader::new(file)
        };

        Ok(reader.lines())
    }

}