use std::net::TcpListener;


fn main() {

   let port = mini_redis::DEFAULT_PORT;
   let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

   mini_redis::server::run(listener);

}
