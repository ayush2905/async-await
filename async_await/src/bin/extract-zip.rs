use std::io;
use std::fs;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32{
    lets args: Vec<_> = std::env::args().collect();

    if args.len()<2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            
        }
    }
}