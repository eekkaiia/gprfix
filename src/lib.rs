use std::fs::File;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::prelude::*;
use std::error::Error;

pub fn fix_dzg (raw_file: File, dzgname: String) -> std::io::Result<()> {
    
    let filename = format!("Clean_DZGs/{}", &dzgname);
    match File::create(&filename) {
        Err(why) => {
            eprintln!("Error creating file or directory {}", why.description());
            std::process::exit(1);
        },
        Ok(fix_file) => {
            let mut fix_file = LineWriter::new(fix_file);
            let mut repeat = false;
            let eol: [u8; 1] = [10];
            let reader = BufReader::new(raw_file); 
            for line in reader.lines() {
                match line {
                    Err(_) => (),
                    Ok(line) => {
                        if line.len() > 5 {
                            match &line[..6] {
                                "$GSSIS" => {
                                    if repeat == false {
                                        fix_file.write_all(line.as_bytes())?;
                                        fix_file.write_all(&eol)?;
                                        repeat = true;
                                    }
                                },
                                "$GPGGA" => {
                                    fix_file.write_all(line.as_bytes())?;
                                    fix_file.write_all(&eol)?;
                                    repeat = false;
                                },
                                _ => (),
                            }
                        }
                    },
                }
            }
        }
    }
    Ok(())
}