use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub trait IdleStrategy {
    fn idle(&mut self, work_count: usize);
    fn reset(&mut self);
}

pub struct BusySpinIdleStrategy;

impl BusySpinIdleStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BusySpinIdleStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleStrategy for BusySpinIdleStrategy {
    #[inline]
    fn idle(&mut self, _work_count: usize) {
        core::hint::spin_loop();
    }

    #[inline]
    fn reset(&mut self) {
    }
}

pub struct BackoffIdleStrategy {
    max_yields: u64,
    max_spins: u64,
    min_park_duration: Duration,
    max_park_duration: Duration,
    yields: u64,
    spins: u64,
    park_duration: Duration,
}

impl BackoffIdleStrategy {
    pub fn new(
        max_spins: u64,
        max_yields: u64,
        min_park_duration: Duration,
        max_park_duration: Duration,
    ) -> Self {
        Self {
            max_yields,
            max_spins,
            min_park_duration,
            max_park_duration,
            yields: 0,
            spins: 0,
            park_duration: min_park_duration,
        }
    }
}

impl Default for BackoffIdleStrategy {
    fn default() -> Self {
        Self::new(
            10,
            5,
            Duration::from_nanos(1),
            Duration::from_millis(1),
        )
    }
}

impl IdleStrategy for BackoffIdleStrategy {
    fn idle(&mut self, work_count: usize) {
        if work_count > 0 {
            self.reset();
        } else if self.spins < self.max_spins {
            self.spins += 1;
            core::hint::spin_loop();
        } else if self.yields < self.max_yields {
            self.yields += 1;
            thread::yield_now();
        } else {
            thread::sleep(self.park_duration);

            self.park_duration = std::cmp::min(
                Duration::from_nanos(self.park_duration.as_nanos() as u64 * 2),
                self.max_park_duration,
            );
        }
    }

    fn reset(&mut self) {
        self.spins = 0;
        self.yields = 0;
        self.park_duration = self.min_park_duration;
    }
}

pub struct SleepingIdleStrategy {
    sleep_duration: Duration,
}

impl SleepingIdleStrategy {
    pub fn new(sleep_duration: Duration) -> Self {
        Self { sleep_duration }
    }
}

impl Default for SleepingIdleStrategy {
    fn default() -> Self {
        Self::new(Duration::from_millis(1))
    }
}

impl IdleStrategy for SleepingIdleStrategy {
    fn idle(&mut self, work_count: usize) {
        if work_count == 0 {
            thread::sleep(self.sleep_duration);
        }
    }

    fn reset(&mut self) {
    }
}

pub struct ControllableIdleStrategy {
    status: AtomicU64,
    busy_spin_strategy: BusySpinIdleStrategy,
    backoff_strategy: BackoffIdleStrategy,
}

const RUNNING: u64 = 0;
const SPINNING: u64 = 1;
const YIELDING: u64 = 2;
const PARKING: u64 = 3;

impl ControllableIdleStrategy {
    pub fn new() -> Self {
        Self {
            status: AtomicU64::new(RUNNING),
            busy_spin_strategy: BusySpinIdleStrategy::new(),
            backoff_strategy: BackoffIdleStrategy::default(),
        }
    }

    pub fn park(&self) {
        self.status.store(PARKING, Ordering::Release);
    }

    pub fn unpark(&self) {
        self.status.store(RUNNING, Ordering::Release);
    }
}

impl Default for ControllableIdleStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleStrategy for ControllableIdleStrategy {
    fn idle(&mut self, work_count: usize) {
        match self.status.load(Ordering::Acquire) {
            RUNNING => {
                self.busy_spin_strategy.idle(work_count);
            }
            PARKING => {
                thread::park();
            }
            _ => {
                self.backoff_strategy.idle(work_count);
            }
        }
    }

    fn reset(&mut self) {
        self.busy_spin_strategy.reset();
        self.backoff_strategy.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_busy_spin_strategy() {
        let mut strategy = BusySpinIdleStrategy::new();

        let start = Instant::now();
        for _ in 0..1000 {
            strategy.idle(0);
        }
        let elapsed = start.elapsed();

        assert!(elapsed < Duration::from_millis(10));
    }

    #[test]
    fn test_backoff_strategy() {
        let mut strategy = BackoffIdleStrategy::default();

        strategy.idle(0);
        strategy.idle(0);
    }

    #[test]
    fn test_sleeping_strategy() {
        let mut strategy = SleepingIdleStrategy::new(Duration::from_nanos(1));

        let start = Instant::now();
        strategy.idle(0);
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_nanos(1));
    }

    #[test]
    fn test_controllable_strategy() {
        let mut strategy = ControllableIdleStrategy::new();

        strategy.idle(1);

        strategy.park();

        strategy.unpark();
        strategy.idle(1);
    }
}