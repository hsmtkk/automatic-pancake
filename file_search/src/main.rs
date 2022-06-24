use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let key_word = &args[2];
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()){
        let name: &str = entry.file_name().to_str().unwrap();
        if name.contains(key_word){
            println!("{}", entry.path().display());
        }
    }
}
