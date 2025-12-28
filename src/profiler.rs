use std::collections::HashMap;
/// Profiler and runtime metrics
/// Tracks function calls, execution time, and hot paths
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FunctionProfile {
    pub name: String,
    pub call_count: Arc<AtomicUsize>,
    pub total_time_ns: Arc<AtomicUsize>,
}

#[derive(Debug, Clone)]
pub struct ProfileData {
    pub function: String,
    pub call_count: usize,
    pub total_time_ms: f64,
    pub avg_time_us: f64,
}

pub struct Profiler {
    profiles: HashMap<String, FunctionProfile>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    pub fn register_function(&mut self, name: &str) -> Arc<AtomicUsize> {
        let counter = Arc::new(AtomicUsize::new(0));
        let profile = FunctionProfile {
            name: name.to_string(),
            call_count: counter.clone(),
            total_time_ns: Arc::new(AtomicUsize::new(0)),
        };
        self.profiles.insert(name.to_string(), profile);
        counter
    }

    pub fn get_hot_functions(&self, threshold: usize) -> Vec<String> {
        let mut functions: Vec<_> = self
            .profiles
            .iter()
            .filter_map(|(name, profile)| {
                let count = profile.call_count.load(Ordering::Relaxed);
                if count >= threshold {
                    Some((name.clone(), count))
                } else {
                    None
                }
            })
            .collect();

        functions.sort_by(|a, b| b.1.cmp(&a.1));
        functions.into_iter().map(|(name, _)| name).collect()
    }

    pub fn report(&self) -> Vec<ProfileData> {
        let mut results: Vec<_> = self
            .profiles
            .iter()
            .map(|(name, profile)| {
                let call_count = profile.call_count.load(Ordering::Relaxed);
                let total_time_ns = profile.total_time_ns.load(Ordering::Relaxed);
                let total_time_ms = total_time_ns as f64 / 1_000_000.0;
                let avg_time_us = if call_count > 0 {
                    (total_time_ns as f64) / (call_count as f64 * 1_000.0)
                } else {
                    0.0
                };

                ProfileData {
                    function: name.clone(),
                    call_count,
                    total_time_ms,
                    avg_time_us,
                }
            })
            .collect();

        results.sort_by(|a, b| b.total_time_ms.partial_cmp(&a.total_time_ms).unwrap());
        results
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! profile_function {
    ($name:expr, $body:block) => {{
        let start = std::time::Instant::now();
        let result = $body;
        let elapsed = start.elapsed();
        // In a real implementation, would send to profiler
        log::debug!("{} took {:?}", $name, elapsed);
        result
    }};
}
