fn main() {
    println!("Hello, world!");
    let name = "Read me. Or don't.txt";
    if let Some(ext) = file_ext(name) {
        println!("file extension: {}", ext)
    } else {
        println!("no file extension")
    }

    let ext = {
        let name = "Read me. Or don't.txt";
        file_ext(name).unwrap_or("") // borrowed value does not live long enough
    };
    println!("extension: {}", ext);

    
    let ext = {
        let name = String::from("Read me. Or don't.txt");
        file_ext(&name).unwrap_or("")
    };
    println!("extension: {}", ext);
}

fn file_ext(name: &str) -> Option<&str> {
    name.split(".").last()
}