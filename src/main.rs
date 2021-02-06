use std::path::Path;
use std::io;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use messages_generator::Generator;


fn load_file(path: &Path, generator: &mut Generator) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        generator.train(&line?)
    }
    Ok(())
}


fn main() -> Result<(), io::Error>{
    let mut generator = Generator::new();
    loop {
        print!("> "); io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        match line.as_str().trim() {
            "clear" => generator = Generator::new(),
            "gen" => println!("{}", generator.generate(20)),
            line if line.starts_with("load") => {
                println!("{}", &line);
                match line.splitn(2, ' ').nth(1) {
                    Some(path) => {
                        load_file(&Path::new(path), &mut generator)?;
                    },
                    None => println!("No file name specified")
                }
            }
            _ => println!("WTF?")
        }
        
    }
}
