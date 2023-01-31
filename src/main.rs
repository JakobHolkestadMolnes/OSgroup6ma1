mod commons;
mod multi_threaded_server;
mod single_threaded_server;
use commons::Server;
use multi_threaded_server::MultiThreadedServer;
use single_threaded_server::SingleThreadedServer;

enum ServerType {
    SingleThreaded,
    MultiThreaded,
}

fn create_server(server_type: ServerType) -> Box<dyn Server> {
    match server_type {
        ServerType::SingleThreaded => Box::new(SingleThreadedServer::new()),
        ServerType::MultiThreaded => Box::new(MultiThreadedServer::new()),
    }
}
fn main() {
    // get type of server from command line argument
    let server_type = match std::env::args().nth(1).expect("Missing argument").as_str() {
        "single" => ServerType::SingleThreaded,
        "multi" => ServerType::MultiThreaded,
        "-h" | "--help" => {
            println!("Usage: ./osgroup6ma1 [single|multi]");
            std::process::exit(0);
        }
        _ => {
            println!("Invalid argument. Use -h or --help for help.");
            std::process::exit(1);
        },
    };

    // create a thread and run the server
    let handle = std::thread::spawn(move || {
        let server = create_server(server_type);
        server.run();
    });

    //wait for the thread to finish
    let _ = handle.join();
}
