use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use dashmap::DashMap;

pub struct Metrics {
    pub request_count: AtomicU64,
    pub error_count: AtomicU64,
    pub avg_response_time: AtomicU64,
    pub endpoint_stats: DashMap<String, EndpointStats>,
}

#[derive(Default)]
pub struct EndpointStats {
    pub count: AtomicU64,
    pub total_time: AtomicU64,
    pub errors: AtomicU64,
}

static METRICS: Lazy<Metrics> = Lazy::new(|| Metrics {
    request_count: AtomicU64::new(0),
    error_count: AtomicU64::new(0),
    avg_response_time: AtomicU64::new(0),
    endpoint_stats: DashMap::new(),
});

impl Metrics {
    pub fn global() -> &'static Metrics {
        &METRICS
    }

    pub fn record_request(&self, endpoint: &str, duration: Duration, is_error: bool) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        
        if is_error {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }

        // Update endpoint-specific stats
        let stats = self.endpoint_stats.entry(endpoint.to_string())
            .or_insert_with(EndpointStats::default);
        
        stats.count.fetch_add(1, Ordering::Relaxed);
        stats.total_time.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        
        if is_error {
            stats.errors.fetch_add(1, Ordering::Relaxed);
        }

        // Update global average (simple moving average approximation)
        let current_avg = self.avg_response_time.load(Ordering::Relaxed);
        let new_avg = if current_avg == 0 {
            duration.as_millis() as u64
        } else {
            (current_avg + duration.as_millis() as u64) / 2
        };
        self.avg_response_time.store(new_avg, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_requests: self.request_count.load(Ordering::Relaxed),
            total_errors: self.error_count.load(Ordering::Relaxed),
            avg_response_time_ms: self.avg_response_time.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub total_errors: u64,
    pub avg_response_time_ms: u64,
}
