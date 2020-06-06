use std::{
    str,
    io::{
        BufRead,
        BufReader,
    },
    os::unix::net::{
        UnixStream,
        UnixListener,
    },
    thread,
    fs,
};
use futures::executor::block_on;
use rhai::Dynamic;

mod sfuncs;
mod engine;

struct Server<'a> {
    listener: UnixListener,
    script_engine: engine::ScriptEngine<'a>,
}
impl Server<'_> {
    fn new() -> Self {
        let mut script_engine = engine::ScriptEngine::new();
        //remove any old socket
        fs::remove_file("/tmp/zled.sock");
        //only one server can be run at a time
        let listener = UnixListener::bind("/tmp/zled.sock")
            .expect("Failed to bind to socket!");

        Server {
            listener: listener,
            script_engine: script_engine,
        }
    }
    fn run(&mut self) {
        block_on(self.get_clients());
    }
    async fn eval(&mut self, code: &str) -> Dynamic {
        self.script_engine.eval(code)
    }
    async fn get_clients(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream).await;
                }
                Err(err) => {
                    println!("Error getting client stream: {}", err);
                    break;
                }
            }
        }
    }
    async fn handle_client(&self, stream: UnixStream) {
        let stream = BufReader::new(stream);
        for line in stream.lines() {
            //println!("{}", line.unwrap());
            if line.unwrap() == "rWin_size" {
                let res = self.eval("eWin_size").await;
            }
        }
    }
}

fn main() {
    let mut server = Server::new();
    server.run();
}

