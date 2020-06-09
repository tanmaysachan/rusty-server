use rusty_server::server;

fn main() {
    let server = server::Server::new();

    if let Err(e) = server::Server::run(&server) {
        eprintln!("Server error: {}", e);
    }
}
