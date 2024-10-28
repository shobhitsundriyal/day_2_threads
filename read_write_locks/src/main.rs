use std::sync::RwLock;
use once_cell::sync::Lazy;

static Users: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().to_string();
}
fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current user");
            let users = Users.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a new user and q to quit");
        let user = read_line();
        if user == "q" {
            break;
        }
        let mut lock = Users.write().unwrap();
        lock.push(user);
    }
}
