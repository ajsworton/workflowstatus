mod elasticsearch;
mod server;

fn main() {

    let addr = [0,0,0,0];
    let port = 5000;

    server::serve(addr, port);

}
