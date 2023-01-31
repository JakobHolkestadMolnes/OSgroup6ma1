mod commons;
mod multi_threaded_server;
mod single_threaded_server;
use commons::Server;
use multi_threaded_server::MultiThreadedServer;

enum ServerType {
    SingleThreaded,
    MultiThreaded,
}
fn main() {
    // start a thread
    let handle = std::thread::spawn(|| {
        // create a single threaded server or a multi threaded server based on the command line argument
        let server = match std::env::args().nth(1).expect("Missing argument").as_str() {
            "single" => ServerType::SingleThreaded,
            "multi" => ServerType::MultiThreaded,
            _ => panic!("Invalid argument, expected 'single' or 'multi'"),
        };

        match server {
            ServerType::SingleThreaded => {
                let server = single_threaded_server::SingleThreadedServer::new();
                server.run();
            }
            ServerType::MultiThreaded => {
                let server = MultiThreadedServer::new();
                server.run();
            }
        }
    });

    //wait for the thread to finish
    let _ = handle.join();
}
