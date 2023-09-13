use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::{env, thread};
use std::str::from_utf8;

enum EnvArg
    {
        Client,
        Server,
    }
impl EnvArg 
    {
        fn c_connect() -> bool
            {
                match TcpStream::connect("localhost:2121")
                    {
                        Ok(mut socket) =>
                            {
                                println!("Connected");
                                let socket_clone = socket.try_clone().expect("Cloning Failed");
                                thread::spawn(move || {Self::c_send(&socket_clone)});
                                let stay = true;
                                while stay
                                    {
                                        let mut data = [0 as u8; 1024];
                                        match socket.read(&mut data)
                                            {
                                                Ok(_) => 
                                                    {
                                                        if data[0] != 0
                                                            {
                                                                println!("{}", from_utf8(&data).unwrap());
                                                            }
                                                    }
                                                Err(e) =>
                                                    {
                                                        println!("Failed to receive data: {}", e);
                                                        return false;
                                                    }
                                            }
                                    }
                            }
                        Err(e) =>
                            {
                                println!("Failed to connect: {}", e);
                                return false;
                            }
                    }
                return true;
            }
        fn c_send(mut socket:&TcpStream)
            {
                let stay = true;
                while stay
                    {
                        socket.write(take_string().as_bytes()).unwrap();
                    }
            }

        fn s_connect() -> bool
            {
                let socket = TcpListener::bind("localhost:2121");
                for stream in socket.expect("Panic").incoming()
                    {
                        match stream
                            {
                                Ok(mut stream) =>
                                    {
                                        let stay = true;
                                        while stay
                                            {
                                                let mut data = [0 as u8; 1024];
                                                match stream.read(&mut data)
                                                    {
                                                        Ok(a) => 
                                                            {
                                                                if a == 0
                                                                    {
                                                                        println!("Connection Closed");
                                                                        return false;
                                                                    }
                                                                println!("{}", stream.write(&data).unwrap());
                                                            }
                                                        Err(e) =>
                                                            {
                                                                println!("Failed to Read: {}", e);
                                                                return false;
                                                            }
                                                    }
                                            }
                                    }
                                Err(e) =>
                                    {
                                        println!("Something Happened: {}", e);
                                    }
                            }
                    }
                
                return true;
            }
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

fn take_string() -> String
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to Read from Console");
        input
    }

fn client() 
    {
        println!("Client");
        if EnvArg::c_connect() != true
            {
                panic!();
            }
    }
fn server()
    {
        println!("Server");
        if EnvArg::s_connect() != true
            {
                panic!();
            }
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
