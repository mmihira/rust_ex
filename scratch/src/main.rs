extern crate rand;
extern crate threadpool;
extern crate curl;
extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::io::{stdout, Write};
use curl::easy::Easy;
use std::thread;
use std::str;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Free,
    Set,
    Dead
}

#[derive(Debug)]
pub enum Material {
    Sand,
    Water,
    Stone,
    Background
}

fn main(){
    let mut n_workers = 10;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.refer(&mut n_workers)
            .add_option(&["-w"], Store,
            "Name for the greeting");
        ap.parse_args_or_exit();
    }
    println!("Using {} workers", n_workers);
    let pool = ThreadPool::new(n_workers);

    let urls = vec![
        "www.cnn.com.au"
    ];

    let mut arcs = Vec::new();
    for i in urls {
        arcs.push((i, Arc::new(Mutex::new(vec![]))));
    }

    // let (tx, rx) = channel();
    for ns in &arcs {
        // let tx = tx.clone();
        let _x = ns.1.clone();
        let url = ns.0.clone();
        pool.execute(move|| {
            let n = _x.clone();
            let mut easy = Easy::new();
            easy.url(&url).unwrap();
            let mut transfer = easy.transfer();
            transfer.write_function(move |data| {
                (*n.lock().unwrap()).extend_from_slice(data);
                println!("{}", data.len());
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
            // println!("{}", sdf);
            // println!("{}", str::from_utf8(&sdf).unwrap());
            // tx.send(1).unwrap();
        });
    }

    println!("Waiting..");
    // for x in rx.iter().take(arcs.len()) { println!("{}", pool.active_count()); println!("{:?}", x); }
    for ns in &arcs {
        while Arc::strong_count(&ns.1) > 1 { }
    }
    println!("strong count 0");

    for ns in arcs {
        let mut guard = Arc::try_unwrap(ns.1).unwrap();
        let vec_box = Box::new(guard.get_mut().unwrap());
        let z = str::from_utf8(*vec_box).unwrap();
        // println!("{:?}", z);
    }

}
