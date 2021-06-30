use crate::output;
use crate::output::DurationUnit;
use rustc_hash::FxHashMap as HashMap;
use std::mem::transmute;
use std::sync::Once;
use std::time::SystemTime;

static START: Once = Once::new();
static mut MANAGER: *mut ExecProbeManager = 0 as *mut ExecProbeManager;

pub(crate) fn get_instance() -> *mut ExecProbeManager {
    START.call_once(|| unsafe {
        let boxed = Box::new(ExecProbeManager::new());
        MANAGER = transmute(boxed);
    });
    unsafe { MANAGER }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct ExecProbeManager {
    values: HashMap<String, Values>,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Values {
    duration: DurationUnit,
    count: u64,
    values: HashMap<String, Value>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Value {
    order: usize,
    count: u64,
    duration: DurationUnit,
}

impl ExecProbeManager {
    pub fn new() -> Self {
        Self {
            values: HashMap::default(),
        }
    }

    fn unsafe_report(v: &mut ExecData) {
        let ctx = get_instance();
        if v.duration > 0 && !v.points.is_empty() {
            unsafe {
                let ctx: &mut ExecProbeManager = &mut *ctx;
                ctx.report(v);
            }
        }
    }

    fn report(&mut self, v: &mut ExecData) {
        if !self.values.contains_key(&v.name) {
            let values = Values {
                values: HashMap::default(),
                duration: 0,
                count: 0,
            };
            self.values.insert(v.name.to_string(), values);
        }
        let mut values = self.values.get_mut(&v.name).unwrap();
        values.duration += v.duration;
        values.count += 1;
        while !v.points.is_empty() {
            let e = v.points.remove(0);
            if !values.values.contains_key(&e.name) {
                values.values.insert(
                    e.name.to_string(),
                    Value {
                        order: values.values.len(),
                        count: 1,
                        duration: e.duration,
                    },
                );
            } else {
                let mut value = values.values.get_mut(&e.name).unwrap();
                value.duration += e.duration;
                value.count += 1;
            }
        }
    }

    pub fn fetch_results(&self) -> Vec<output::ExecDuration> {
        let mut res: Vec<output::ExecDuration> = Vec::new();
        for (key, e) in &self.values {
            let mut elt = output::ExecDuration::new(&key, e.count, e.duration, e.duration);
            let mut keys: Vec<String> = Vec::new();
            for _ in e.values.keys() {
                keys.push(String::new());
            }
            for (name, v) in &e.values {
                keys[v.order].push_str(name.as_str());
            }
            for name in keys.iter() {
                let v = e.values.get(name).unwrap();
                elt.add(output::ExecDuration::new(
                    &name, v.count, v.duration, e.duration,
                ));
            }
            res.push(elt);
        }

        res
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct ExecData {
    pub name: String,
    pub begin_timestamp: std::time::SystemTime,
    pub now: std::time::SystemTime,
    pub duration: DurationUnit,
    pub points: Vec<Point>,
}

impl ExecData {
    pub fn new(name: &str) -> Self {
        let now = std::time::SystemTime::now();
        ExecData {
            name: name.to_string(),
            points: Vec::new(),
            begin_timestamp: now,
            now,
            duration: 0,
        }
    }

    pub fn add_point(&mut self, name: &str) {
        let now = std::time::SystemTime::now();
        if let Ok(d) = now.duration_since(self.now) {
            self.points.push(Point {
                name: name.to_string(),
                duration: d.as_nanos(),
            });
            self.now = now;
        }
    }

    pub fn stop(&mut self) {
        if let Ok(d) = SystemTime::now().duration_since(self.begin_timestamp) {
            self.duration = d.as_nanos();
            ExecProbeManager::unsafe_report(self);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point {
    pub name: String,
    pub duration: DurationUnit,
}
