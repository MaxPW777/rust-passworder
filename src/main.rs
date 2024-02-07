fn intro() {
    println!(
        "
RUST PASSWORD MANAGER
USAGE: 
    list : list passwords
    add 'service name' : add password
    get 'service name' : gets password
    help : shows this screen
    "
    )
}

fn main() {
    intro();
}
