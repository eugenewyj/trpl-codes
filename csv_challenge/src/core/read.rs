use super::{Error, PathBuf, File, Read, Write};

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}

pub fn write_csv(csv_data: &str, file_name: &Option<String>) -> Result<(), Error> {
    write(csv_data, file_name)?;
    Ok(())
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file: File = open(path)?;
    file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        return Err("input file missing")?
    }
    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn write(data: &str, out_file_name: &Option<String>) -> Result<(), Error> {
    match out_file_name {
        Some(file_name) =>  { 
            let mut buffer = File::create(file_name.to_string())?;
            buffer.write_all(data.as_bytes())?;
        },
        None => {
            std::io::stdout().write_all(data.as_bytes())?;
        },
    };
    
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::load_csv;

    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
}