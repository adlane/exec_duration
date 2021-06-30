extern crate exec_duration;

use exec_duration::ExecProbe;
use std::thread::sleep;
use std::time;

fn main() {
    let mut i = 0;
    while i < 10 {
        let mut o = ExecProbe::new("main");
        func1();
        o.add_point("func1");
        func2();
        o.add_point("func2");
        i += 1;
    }

    let list = exec_duration::fetch_results();
    for r in list.iter() {
        println!("{}", r);
    }
}

fn func1() {
    sleep(time::Duration::from_millis(100));
}

fn func2() {
    sleep(time::Duration::from_millis(50));
}
