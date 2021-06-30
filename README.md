# exec_duration
Get execution duration of a function or a bloc of code

Stupid and easy to use Rust code instrumentalization library.

This module provides a simple API to measure the execution duration of a function or a block code.

## Install

```toml
[dependencies]
exec_duration = "0.1.0"
```

## Examples

```
use exec_duration::ExecProbe;
//!
fn function_1() {
    // Create a new execution probe object
    // exec duration will be computed from this point
    let mut ep = ExecProbe::new("function_1");
//!
    // some code
//!
    // add a new point
    ep.add_point("part 1");
//!
    // some code
//!
    // add a new point
    ep.add_point("part 2");
//!
    // some code
//!
    // add a new point
    ep.add_point("part 3");
}
//!
fn function_2() {
    // Create a new execution probe object
    // exec duration will be computed from this point
    let mut ep = ExecProbe::new("function_2");
//!
    // some code
//!
    // add a new point
    ep.add_point("part 1");
//!
    // some code
//!
    // add a new point
    ep.add_point("part 2");
//!
    // some code
//!
    // optionally call the stop function
    ep.stop();
}
//!
fn main() {
    function_1();
    function_2();
//!
    // fetch results
    if let Ok(list) = exec_duration::fetch_results() {
        for r in list.iter() {
            println!("{}", r);
        }
    }
}
```
