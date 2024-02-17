fn main() {
    let message = "Hello, World!";
    let new_message = print_welcome(message);
    println!("{}", new_message);
}

fn print_welcome(text: &str) -> &str {
    print!("{}", text);
    "Hi There"
}
