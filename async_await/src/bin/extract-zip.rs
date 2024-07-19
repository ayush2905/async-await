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
            if !comment.is_empty() {
                println!("File {} comment {}", i, comment);
            }
        }
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size
            );
            if let Some(p) = outpath.parent(){
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}