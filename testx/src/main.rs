#![feature(duration_consts_float)]
use std::{thread::sleep, time::Duration};
use llml::{Matf2};

const WAIT : Duration = Duration::from_secs_f32(0.5);

fn main () {
    let mut alpha = Matf2::of_rot(1.);
    let beta = Matf2::of_rot(2.);

    loop {
        alpha = alpha * beta;
        let angle = alpha.x.x.acos();
        
        println!("{} => {:?}", angle, alpha);
        sleep(WAIT);
    }
}