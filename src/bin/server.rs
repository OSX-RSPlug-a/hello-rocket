use std::net::TcpListener;

use exchanger::{response::handle_connection, thread_pool::ThreadPool, configuration::{WEBSERVER_PORT, get_env_var, MAX_WORKERS_ENV, DEFAULT_MAX_WORKERS}};
use log::info;

fn main() {
    env_logger::init();
    let listener = TcpListener::bind(format!("127.0.0.1:{WEBSERVER_PORT}")).unwrap();
    info!("Server listening at port {}", WEBSERVER_PORT);
    let max_workers: usize = get_env_var(MAX_WORKERS_ENV, DEFAULT_MAX_WORKERS);
    let pool = ThreadPool::new(max_workers);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}
