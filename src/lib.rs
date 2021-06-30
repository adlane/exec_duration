//! Stupid and easy to use Rust code instrumentalization library.
//!
//! This module provides a simple API to measure the execution duration of a function or a block code.
//!
//! # Examples
//!
//! ```
//! use exec_duration::ExecProbe;
//!
//! fn function_1() {
//!     // Create a new execution probe object
//!     // exec duration will be computed from this point
//!     let mut ep = ExecProbe::new("function_1");
//!
//!     // some code
//!
//!     // add a new point
//!     ep.add_point("part 1");
//!
//!     // some code
//!
//!     // add a new point
//!     ep.add_point("part 2");
//!
//!     // some code
//!
//!     // add a new point
//!     ep.add_point("part 3");
//! }
//!
//! fn function_2() {
//!     // Create a new execution probe object
//!     // exec duration will be computed from this point
//!     let mut ep = ExecProbe::new("function_2");
//!
//!     // some code
//!
//!     // add a new point
//!     ep.add_point("part 1");
//!
//!     // some code
//!
//!     // add a new point
//!     ep.add_point("part 2");
//!
//!     // some code
//!
//!     // optionally call the stop function
//!     ep.stop();
//! }
//!
//! fn main() {
//!     function_1();
//!     function_2();
//!
//!     // fetch results
//!     let list = exec_duration::fetch_results();
//!     for r in list.iter() {
//!         println!("{}", r);
//!     }
//! }
//! ```

#![doc(issue_tracker_base_url = "https://github.com/adlane/exec_duration/issues/")]
#![doc(html_root_url = "https://docs.rs/exec_duration/0.1.1")]
#![doc(html_no_source)]
#![deny(missing_docs)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate more_asserts;

mod manager;
/// output module exposes the results (metrics)
pub mod output;

/// Fetch execution metrics.
/// Typically, this function needs to be called once the execution of all measured blocks is done.
///
/// # Examples
/// ```
/// use exec_duration;
/// use exec_duration::ExecProbe;
///
/// let mut ep = ExecProbe::new("main");
///
/// // code
///
/// // fetch results
/// let list = exec_duration::fetch_results();
/// for r in list.iter() {
///     println!("{}", r);
/// }
/// ```
pub fn fetch_results() -> Vec<output::ExecDuration> {
    let ctx = manager::get_instance();
    unsafe {
        let ctx: &mut manager::ExecProbeManager = &mut *ctx;
        ctx.fetch_results()
    }
}

impl ExecProbe {
    /// Create a new instance
    ///
    /// # Examples
    /// ```
    /// use exec_duration;
    /// use exec_duration::ExecProbe;
    ///
    /// let ep = ExecProbe::new("main");
    /// ```
    pub fn new(name: &str) -> Self {
        ExecProbe {
            data: manager::ExecData::new(name),
            stop_done: false,
        }
    }

    /// Add a new point
    ///
    /// # Examples
    /// ```
    /// use exec_duration;
    /// use exec_duration::ExecProbe;
    ///
    /// let mut ep = ExecProbe::new("main");
    /// ep.add_point("line 1");
    /// ```
    pub fn add_point(&mut self, name: &str) {
        self.data.add_point(name);
    }

    /// Stop metrics and commit
    ///
    /// In most cases a call to this function is optional because ExecProbe implements the Drop trait and when an ExecProbe instance goes out of scope, a call to `stop` function will be performed
    /// automatically.
    ///
    /// # Examples
    /// ```
    /// use exec_duration;
    /// use exec_duration::ExecProbe;
    ///
    /// let mut ep = ExecProbe::new("main");
    /// ep.add_point("line 1");
    /// ep.stop();
    /// ```
    pub fn stop(&mut self) {
        if !self.stop_done {
            self.data.stop();
            self.stop_done = true;
        }
    }
}

impl Drop for ExecProbe {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Execution probe structure.
/// Instances are created using `ExecProbe::new` function.
///
/// # Examples
/// ```
/// use exec_duration;
/// use exec_duration::ExecProbe;
///
/// let mut ep = ExecProbe::new("function_1");
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecProbe {
    data: manager::ExecData,
    stop_done: bool,
}

#[cfg(test)]
mod tests {

    use crate::ExecProbe;
    use std::thread::sleep;
    use std::time;
    const NB: u64 = 10;
    const SLEEP_1: u64 = 100;
    const SLEEP_2: u64 = 50;
    const MAIN: &str = "main";
    const FUNC_1: &str = "func_1";
    const FUNC_2: &str = "func_2";

    #[test]
    fn hello() {
        let mut i = 0;
        while i < NB {
            let mut o = ExecProbe::new(MAIN);
            func1();
            o.add_point(FUNC_1);
            func2();
            o.add_point(FUNC_2);
            i += 1;
        }

        let list = crate::fetch_results();
        assert_eq!(list.len(), 1);
        let r = list.get(0).unwrap();
        assert_eq!(r.get_name(), MAIN);
        assert_eq!(r.get_exec_count(), NB);
        assert_le!(
            r.get_avg_duration().as_millis(),
            (SLEEP_1 + SLEEP_2 + 1) as u128
        );
        assert_le!(
            r.get_total_duration().as_millis(),
            ((SLEEP_1 + SLEEP_2 + 1) * NB) as u128
        );
        assert_ge!(
            r.get_avg_duration().as_millis(),
            (SLEEP_1 + SLEEP_2) as u128
        );
        assert_ge!(
            r.get_total_duration().as_millis(),
            ((SLEEP_1 + SLEEP_2) * NB) as u128
        );
        assert_eq!(r.get_elements().len(), 2);
        let v = r.get_elements().get(0).unwrap();
        assert_eq!(v.get_name(), FUNC_1);
        assert_eq!(v.get_exec_count(), NB);
        assert_le!(v.get_avg_duration().as_millis(), (SLEEP_1 + 1) as u128);
        assert_le!(
            v.get_total_duration().as_millis(),
            ((SLEEP_1 + 1) * NB) as u128
        );
        assert_ge!(v.get_avg_duration().as_millis(), SLEEP_1 as u128);
        assert_ge!(v.get_total_duration().as_millis(), (SLEEP_1 * NB) as u128);
        assert_eq!(v.get_elements().len(), 0);
        let v = r.get_elements().get(1).unwrap();
        assert_eq!(v.get_name(), FUNC_2);
        assert_eq!(v.get_exec_count(), NB);
        assert_le!(v.get_avg_duration().as_millis(), (SLEEP_2 + 1) as u128);
        assert_le!(
            v.get_total_duration().as_millis(),
            ((SLEEP_2 + 1) * NB) as u128
        );
        assert_ge!(v.get_avg_duration().as_millis(), SLEEP_2 as u128);
        assert_ge!(v.get_total_duration().as_millis(), (SLEEP_2 * NB) as u128);
        assert_eq!(v.get_elements().len(), 0);
    }

    fn func1() {
        sleep(time::Duration::from_millis(SLEEP_1));
    }

    fn func2() {
        sleep(time::Duration::from_millis(SLEEP_2));
    }
}
