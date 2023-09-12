use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::{net::SocketAddr, collections::VecDeque};
use std::vec::Vec;
use std::io;
use volo_gen::volo::example::{
	PingRequest,
	PingResponse,
	SetRequest,
	SetResponse,
	GetRequest,
	GetResponse,
	DelRequest,
	DelResponse,
    PubRequest,
    PubResponse,
    SubRequest,
    SubResponse,
};
use pilota::FastStr;
use mini_redis::FilterLayer;
use async_channel::{Sender, Receiver, bounded};

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::RedisServiceClientBuilder::new("volo-example")
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}
#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut id = 0;
    loop {
        eprint!("mini-redis> ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取失败"); //TODO
        
        let args: Vec<&str> = input.trim().split(" ").collect();

        match args[0].to_lowercase().as_str() {
            "set" => {
                let req = SetRequest{id: id.clone(), key: FastStr::new(args[1]), value: FastStr::new(args[2])};
                let resp = CLIENT.set(req).await;
                match resp {
                    Ok(SetResponse{res: true}) => println!("OK"),
                    _ => println!("False")
                }
            }
            "ping" => {
                let req = PingRequest{id: id.clone()};
                let resp = CLIENT.ping(req).await;
                match resp {
                    Ok(PingResponse{}) => println!("PONG"),
                    _ => println!("Could not connect to Redis")
                }
            }
            "get" => {
                let req = GetRequest{id: id.clone(), key: FastStr::new(args[1])};
                let resp = CLIENT.get(req).await;
                match resp {
                    Ok(_resp) => {
                        if let Some(val) = _resp.value {
                            println!("{}", val);
                        } else {
                            println!("(nil)");
                        }
                    },
                    _ => println!("Error")
                }
            }
            "del" => {
                let mut ks: VecDeque<pilota::FastStr> = args.into_iter().map(|x| pilota::FastStr::new(x)).clone().collect();
                ks.pop_front();
                let req = DelRequest{id: id.clone(), keys: ks.into()};
                let resp = CLIENT.del(req).await;
                match resp {
                    Ok(_resp) => {
                        println!("{}", _resp.deleted);
                    },
                    Err(error) => println!("{:?}", error),
                    _ => println!("Unknown Error")
                }
            }
            "publish" => {
                let req = PubRequest{id: id.clone(), channel: FastStr::new(args[1]), msg: FastStr::new(args[2])};
                let resp = CLIENT.publish(req).await;
                match resp {
                    Ok(_resp) => {
                        println!("1");
                    },
                    _ => {
                        println!("0");
                    }
                }
            }
            "subscribe" => {
                println!("message");
                println!("{:?}", args[1]);
                println!("1\n");

                loop {
                    let subreq = SubRequest{id: id.clone(), channel: FastStr::new(args[1])};
                    let resp = CLIENT.sub(subreq).await;
                    let msg = resp.unwrap().msg;
                    println!("message");
                    println!("{:?}", args[1]);
                    println!("{:?}\n", msg);
                }
            }
            _ => {
                println!("Wrong Input.")
            }
        }
        id = id + 1;
    }
}
