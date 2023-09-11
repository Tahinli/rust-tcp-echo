use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

enum EnvArg
    {
        Client,
        Server,
    }

fn take_arg() -> EnvArg
    {
        let args:Vec<String> = env::args().collect();
        match args.len()
            {
                1 => println!("Give an argument"),
                2 => 
                    {
                        if args[1] == "c"
                            {
                                return EnvArg::Client;
                            }
                        else if args[1] == "s"
                            {
                                return EnvArg::Server;
                            }
                        else 
                            {
                                println!("Give valid argument, c or s");
                            }
                    },
                _ => println!("Only one argument is allowed"),
            }
        panic!();
    }

fn client() 
    {
        println!("Client");
    }
fn server()
    {
        println!("Server");
    }

fn main() 
    {
        println!("Hello, world!");
        match take_arg() 
            {
                EnvArg::Client => client(),
                EnvArg::Server => server()
            }
    }
