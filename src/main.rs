extern crate flate2;
extern crate serde_json;

use std::{fs};
use std::io::{Result, 
        BufReader, BufRead};
use std::time::{SystemTime};
use std::path::{Path};
use std::collections::HashMap;
use flate2::read::GzDecoder;


fn main() -> Result<()> {
    let mut args = std::env::args();

    let dir_str = args.nth(1).unwrap_or("./data".to_string());                
    let target_dir = Path::new(&dir_str);
    let target_name = target_dir.file_name().expect("Path Error").to_os_string().into_string().unwrap();
    println!("target_dir: {:?}", target_dir);
    let mut ent_freqs: HashMap<String, u32> = HashMap::new();
    
    for entry in fs::read_dir(&target_dir)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.into_os_string().into_string().unwrap();
        if path_str.ends_with(".gz"){
            print!("{:?} ... ", path_str);
            let tick = SystemTime::now();
            process_gz_file(&path_str, &mut ent_freqs)?;
            let elapsed = tick.elapsed().unwrap().as_millis() as f64;
            println!("{}sec", elapsed/1000.)
        }
    }

    save_ent_freqs(&ent_freqs, &format!("{}.json", &target_name))?;
    Ok(())
}

fn process_gz_file(gz_path: &impl AsRef<Path>,
        ent_freqs: &mut HashMap<String, u32>) -> Result<()> {
    let file = fs::File::open(gz_path)?;
    let gz_stream = GzDecoder::new(file);
    let reader = BufReader::new(gz_stream);
    
    for line in reader.lines() {
        let line = line?;
        let toks: Vec<&str> = line.split(' ').collect();
        if toks[0] == "zh"{
            // println!("{}", line);
            // io::stdout().flush().unwrap();
            let cur_freq = match toks[2].parse::<u32>(){
                Ok(f) => f,
                _ => 0
            };
            let item_title = toks[1].to_string();
            *ent_freqs.entry(item_title).or_insert(0) += cur_freq;
            
            // if n > 10 { break }
        }
        
    }

    Ok(())
}

fn save_ent_freqs(
        ent_freqs: &HashMap<String, u32>, 
        out_path: &impl AsRef<Path>) -> Result<()>{
    let file = fs::File::create(out_path)?;
    // let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(file, ent_freqs)?;

    Ok(())
}
