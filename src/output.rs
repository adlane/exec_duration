use std::fmt;
use std::time::Duration;

pub(crate) type DurationUnit = u128;

/// Execution duration metrics
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
///     println!("[{}] costs {} seconds", r.get_name(), r.get_total_duration().as_secs());
///     for part in r.get_elements().iter() {
///         println!("[{}::{}] costs {} seconds ({}%)",
///             r.get_name(), part.get_name(),
///             part.get_total_duration().as_secs(), part.get_exec_percent()
///         );
///     }
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecDuration {
    name: String,
    count: u64,
    duration: DurationUnit,
    total: DurationUnit,
    childs: Vec<ExecDuration>,
}

impl ExecDuration {
    #[doc(hidden)]
    pub(crate) fn new(name: &str, count: u64, duration: DurationUnit, total: DurationUnit) -> Self {
        ExecDuration {
            name: name.to_string(),
            count,
            duration,
            total,
            childs: Vec::new(),
        }
    }

    #[doc(hidden)]
    pub(crate) fn add(&mut self, v: ExecDuration) {
        self.childs.push(v);
    }

    /// Get execution duration as a percentage
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
    ///     println!("Exec duration [{}] {}%", r.get_name(), r.get_exec_percent());
    /// }
    /// ```
    pub fn get_exec_percent(&self) -> u8 {
        (self.duration * 100 / self.total) as u8
    }

    /// Get execution count
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
    ///     println!("[{}] was executed {} times", r.get_name(), r.get_exec_count());
    /// }
    /// ```
    pub fn get_exec_count(&self) -> u64 {
        self.count
    }

    /// Get average execution time
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
    ///     println!("[{}] costs ~{} seconds in average",
    ///         r.get_name(), r.get_avg_duration().as_secs()
    ///     );
    /// }
    /// ```
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_nanos((self.duration / self.count as DurationUnit) as u64)
    }

    /// Get total execution time
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
    ///     println!("[{}] costs {} seconds", r.get_name(), r.get_total_duration().as_secs());
    /// }
    /// ```
    pub fn get_total_duration(&self) -> Duration {
        Duration::from_nanos(self.duration as u64)
    }

    /// Get elements if any
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
    ///     for part in r.get_elements().iter() {
    ///         println!("[{}::{}] costs {} seconds ({}%)",
    ///             r.get_name(), part.get_name(),
    ///             part.get_total_duration().as_secs(), part.get_exec_percent()
    ///         );
    ///     }
    /// }
    /// ```
    pub fn get_elements(&self) -> &[ExecDuration] {
        &self.childs
    }

    /// Get measured code block name
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
    ///     println!("[{}] costs {} seconds (~{} seconds in average)",
    ///         r.get_name(), r.get_total_duration().as_secs(),
    ///         r.get_avg_duration().as_secs()
    ///     );
    /// }
    /// ```
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl fmt::Display for ExecDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[{}] {}% Call: {:?} T: {:?} Avg: {:?}",
            self.get_name(),
            self.get_exec_percent(),
            self.get_exec_count(),
            self.get_total_duration(),
            self.get_avg_duration(),
        )?;
        for v in self.childs.iter() {
            write!(f, "[{}] {}", self.name, v)?;
        }

        Ok(())
    }
}
