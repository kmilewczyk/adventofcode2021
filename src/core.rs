pub mod file {
    use anyhow::{ Context, Result };

    pub fn read_lines(path: &str) -> Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
        use std::io::BufRead;

        let reader = {
            let file = std::fs::File::open(path).with_context(|| format!("Couldn't open file '{}'.", path))?;
            std::io::BufReader::new(file)
        };

        Ok(reader.lines())
    }

}