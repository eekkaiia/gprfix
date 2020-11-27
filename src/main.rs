use std::fs;
use std::fs::File;
use std::env::current_dir;
use gprfix::fix_dzg;

fn main() {   
    match fs::create_dir("Clean_DZGs") {
        Ok(_) => {
            if let Ok(here) = current_dir() {
                match fs::read_dir(here) {
                    Ok(result) => {
                        for target in result {
                            if let Ok(target) = target {
                                let filename = target.file_name().into_string().unwrap();
                                if filename.to_lowercase().contains(".dzg") {
                                    println!("{:?}", filename);
                                    let file_to_fix = File::open(&filename);
                                    match fix_dzg(file_to_fix.unwrap(), filename) {
                                        Ok(_) => (),
                                        Err(why) => eprintln!("fix_dzg error - {:?}", why),
                                    }
                                }
                            }
                        }
                    },
                    Err(why) => eprintln!("Read directory error - {:?}", why),
                }
            }
        },
        Err(why) => eprintln!("Create directory [Clean_DZGs] error - {:?}", why),
    }
}