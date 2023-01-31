mod commons;
mod multi_threaded_server;
mod single_threaded_server;
use multi_threaded_server::MultiThreadedServer;

fn main() {
    // start a thread
    let handle = std::thread::spawn(|| {
        let server = MultiThreadedServer::new();
        server.run();
    });

    //wait for the thread to finish
    let _ = handle.join();
}
