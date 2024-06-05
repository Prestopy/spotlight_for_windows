use walkdir::WalkDir;

fn search_files(user: &str, query: &str) {
    for entry in WalkDir::new("/Users/".to_string() + user + "/") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_str = path.to_str().unwrap();
            if path_str.contains(query) {
                println!("{}", path_str);
            }
        }
    }
}

fn main() {
    let mut user = String::new();

    println!("What is your username?");
    std::io::stdin().read_line(&mut user).unwrap();

    loop {
        println!("What is your query?");
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();

        match query.trim() {
            "###modifyUsername" => {
                println!("What is your new username?");
                std::io::stdin().read_line(&mut user).unwrap();
            },
            "###exit" => break,
            _ => ()
        }

        // Search for files
        search_files(&user.trim(), &query.trim());
    }
}
