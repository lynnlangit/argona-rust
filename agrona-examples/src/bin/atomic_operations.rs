use agrona_concurrent::{AtomicBuffer, BusySpinIdleStrategy, BackoffIdleStrategy, IdleStrategy};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

const BUFFER_SIZE: usize = 1024;
const ITERATIONS: usize = 1_000_000;

fn main() {
    println!("Agrona Rust Atomic Operations Example");
    println!("====================================");

    basic_atomic_operations();
    concurrent_counter_example();
    idle_strategy_comparison();
    producer_consumer_example();
}

fn basic_atomic_operations() {
    println!("\n1. Basic Atomic Operations");
    println!("-------------------------");

    let mut buffer = AtomicBuffer::new(BUFFER_SIZE).expect("Failed to create atomic buffer");

    buffer.put_volatile_u32(0, 42).unwrap();
    println!("Volatile write u32: {}", buffer.get_volatile_u32(0).unwrap());

    buffer.put_ordered_u64(8, 1234567890123456789u64).unwrap();
    println!("Ordered write u64: {}", buffer.get_volatile_u64(8).unwrap());

    println!("Compare-and-set u32 (42 -> 100): {}",
             buffer.compare_and_set_u32(0, 42, 100).unwrap());
    println!("After CAS: {}", buffer.get_volatile_u32(0).unwrap());

    println!("Compare-and-set u32 (42 -> 200): {}",
             buffer.compare_and_set_u32(0, 42, 200).unwrap());
    println!("After failed CAS: {}", buffer.get_volatile_u32(0).unwrap());

    let old_value = buffer.get_and_add_u32(0, 50).unwrap();
    println!("Get-and-add u32 (old: {}, new: {})", old_value, buffer.get_volatile_u32(0).unwrap());
}

fn concurrent_counter_example() {
    println!("\n2. Concurrent Counter Example");
    println!("-----------------------------");

    let buffer = Arc::new(std::sync::Mutex::new(
        AtomicBuffer::new(BUFFER_SIZE).expect("Failed to create atomic buffer")
    ));

    buffer.lock().unwrap().put_volatile_u64(0, 0).unwrap();

    let num_threads = 4;
    let increments_per_thread = 250_000;
    let barrier = Arc::new(Barrier::new(num_threads + 1));

    let mut handles = Vec::new();

    for thread_id in 0..num_threads {
        let buffer_clone = Arc::clone(&buffer);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let start = Instant::now();
            for _ in 0..increments_per_thread {
                buffer_clone.lock().unwrap().get_and_add_u64(0, 1).unwrap();
            }
            let elapsed = start.elapsed();

            println!("Thread {} completed in {:?}", thread_id, elapsed);
        });

        handles.push(handle);
    }

    println!("Starting {} threads, each incrementing {} times...", num_threads, increments_per_thread);
    let start = Instant::now();
    barrier.wait();

    for handle in handles {
        handle.join().unwrap();
    }

    let total_time = start.elapsed();
    let final_count = buffer.lock().unwrap().get_volatile_u64(0).unwrap();
    let expected_count = (num_threads * increments_per_thread) as u64;

    println!("Final count: {} (expected: {})", final_count, expected_count);
    println!("Total time: {:?}", total_time);
    println!("Operations/second: {:.0}",
             expected_count as f64 / total_time.as_secs_f64());

    assert_eq!(final_count, expected_count);
}

fn idle_strategy_comparison() {
    println!("\n3. Idle Strategy Comparison");
    println!("--------------------------");

    let strategies: Vec<(Box<dyn IdleStrategy>, &str)> = vec![
        (Box::new(BusySpinIdleStrategy::new()), "BusySpinIdleStrategy"),
        (Box::new(BackoffIdleStrategy::default()), "BackoffIdleStrategy"),
    ];

    for (mut strategy, name) in strategies {
        println!("\nTesting {}", name);

        let start = Instant::now();
        let test_duration = Duration::from_millis(100);

        let mut work_cycles = 0;
        let mut idle_cycles = 0;

        while start.elapsed() < test_duration {
            if work_cycles % 10 == 0 {
                strategy.idle(0);
                idle_cycles += 1;
            } else {
                strategy.idle(1);
                work_cycles += 1;
            }
        }

        println!("  Work cycles: {}", work_cycles);
        println!("  Idle cycles: {}", idle_cycles);
        println!("  Total cycles: {}", work_cycles + idle_cycles);
    }
}

fn producer_consumer_example() {
    println!("\n4. Producer-Consumer Example");
    println!("---------------------------");

    let buffer = Arc::new(std::sync::Mutex::new(
        AtomicBuffer::new(BUFFER_SIZE).expect("Failed to create atomic buffer")
    ));

    let head_index = 0;
    let tail_index = 8;
    let data_start = 16;
    let max_messages = (BUFFER_SIZE - data_start) / 8;

    {
        let mut buf = buffer.lock().unwrap();
        buf.put_volatile_u64(head_index, 0).unwrap();
        buf.put_volatile_u64(tail_index, 0).unwrap();
    }

    let producer_buffer = Arc::clone(&buffer);
    let consumer_buffer = Arc::clone(&buffer);

    let barrier = Arc::new(Barrier::new(3));
    let producer_barrier = Arc::clone(&barrier);
    let consumer_barrier = Arc::clone(&barrier);

    let producer = thread::spawn(move || {
        let mut strategy = BusySpinIdleStrategy::new();
        producer_barrier.wait();

        println!("Producer starting...");
        let start = Instant::now();

        for message in 0u64..100_000 {
            loop {
                let mut buf = producer_buffer.lock().unwrap();
                let current_head = buf.get_volatile_u64(head_index).unwrap();
                let current_tail = buf.get_volatile_u64(tail_index).unwrap();

                let next_head = (current_head + 1) % max_messages as u64;

                if next_head != current_tail {
                    let data_offset = data_start + ((current_head % max_messages as u64) * 8) as usize;
                    buf.put_volatile_u64(data_offset, message).unwrap();
                    buf.put_ordered_u64(head_index, next_head).unwrap();
                    break;
                } else {
                    drop(buf);
                    strategy.idle(0);
                }
            }
        }

        let elapsed = start.elapsed();
        println!("Producer completed in {:?}", elapsed);
    });

    let consumer = thread::spawn(move || {
        let mut strategy = BusySpinIdleStrategy::new();
        consumer_barrier.wait();

        println!("Consumer starting...");
        let start = Instant::now();
        let mut messages_received = 0u64;
        let mut last_message = 0u64;

        while messages_received < 100_000 {
            loop {
                let mut buf = consumer_buffer.lock().unwrap();
                let current_head = buf.get_volatile_u64(head_index).unwrap();
                let current_tail = buf.get_volatile_u64(tail_index).unwrap();

                if current_tail != current_head {
                    let data_offset = data_start + ((current_tail % max_messages as u64) * 8) as usize;
                    let message = buf.get_volatile_u64(data_offset).unwrap();

                    if message != last_message + 1 && message != 0 {
                        panic!("Message order violation: expected {}, got {}",
                               last_message + 1, message);
                    }

                    last_message = message;
                    messages_received += 1;

                    let next_tail = (current_tail + 1) % max_messages as u64;
                    buf.put_ordered_u64(tail_index, next_tail).unwrap();
                    break;
                } else {
                    drop(buf);
                    strategy.idle(0);
                }
            }
        }

        let elapsed = start.elapsed();
        println!("Consumer completed in {:?}", elapsed);
        println!("Messages per second: {:.0}",
                 messages_received as f64 / elapsed.as_secs_f64());
    });

    println!("Starting producer-consumer test...");
    barrier.wait();

    producer.join().unwrap();
    consumer.join().unwrap();

    println!("Producer-consumer test completed successfully!");
}