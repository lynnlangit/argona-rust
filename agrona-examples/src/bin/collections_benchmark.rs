use agrona_collections::{IntHashMap, IntHashSet, MutableInteger};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const ITERATIONS: usize = 1_000_000;

fn main() {
    println!("Agrona Rust Collections Benchmark");
    println!("=================================");

    hashmap_comparison();
    hashset_comparison();
    mutable_integer_demo();
}

fn hashmap_comparison() {
    println!("\n1. HashMap Performance Comparison");
    println!("---------------------------------");

    println!("Testing {} iterations", ITERATIONS);

    println!("\nAgrona IntHashMap:");
    let start = Instant::now();
    let mut agrona_map = IntHashMap::new();

    for i in 0..ITERATIONS {
        agrona_map.insert(i as i32, format!("value_{}", i));
    }

    let insert_time = start.elapsed();
    println!("Insert time: {:?} ({:.2} ns/op)",
             insert_time,
             insert_time.as_nanos() as f64 / ITERATIONS as f64);

    let start = Instant::now();
    let mut lookup_count = 0;
    for i in 0..ITERATIONS {
        if agrona_map.get(&(i as i32)).is_some() {
            lookup_count += 1;
        }
    }
    let lookup_time = start.elapsed();
    println!("Lookup time: {:?} ({:.2} ns/op)",
             lookup_time,
             lookup_time.as_nanos() as f64 / ITERATIONS as f64);
    println!("Successful lookups: {}", lookup_count);

    println!("Final map size: {}", agrona_map.len());

    println!("\nStandard HashMap:");
    let start = Instant::now();
    let mut std_map = HashMap::new();

    for i in 0..ITERATIONS {
        std_map.insert(i as i32, format!("value_{}", i));
    }

    let std_insert_time = start.elapsed();
    println!("Insert time: {:?} ({:.2} ns/op)",
             std_insert_time,
             std_insert_time.as_nanos() as f64 / ITERATIONS as f64);

    let start = Instant::now();
    let mut std_lookup_count = 0;
    for i in 0..ITERATIONS {
        if std_map.get(&(i as i32)).is_some() {
            std_lookup_count += 1;
        }
    }
    let std_lookup_time = start.elapsed();
    println!("Lookup time: {:?} ({:.2} ns/op)",
             std_lookup_time,
             std_lookup_time.as_nanos() as f64 / ITERATIONS as f64);
    println!("Successful lookups: {}", std_lookup_count);

    println!("Final map size: {}", std_map.len());

    let insert_speedup = std_insert_time.as_nanos() as f64 / insert_time.as_nanos() as f64;
    let lookup_speedup = std_lookup_time.as_nanos() as f64 / lookup_time.as_nanos() as f64;

    println!("\nPerformance comparison:");
    println!("Insert speedup: {:.2}x", insert_speedup);
    println!("Lookup speedup: {:.2}x", lookup_speedup);
}

fn hashset_comparison() {
    println!("\n2. HashSet Performance Comparison");
    println!("---------------------------------");

    println!("Testing {} iterations", ITERATIONS);

    println!("\nAgrona IntHashSet:");
    let start = Instant::now();
    let mut agrona_set = IntHashSet::new();

    for i in 0..ITERATIONS {
        agrona_set.insert(i as i32);
    }

    let insert_time = start.elapsed();
    println!("Insert time: {:?} ({:.2} ns/op)",
             insert_time,
             insert_time.as_nanos() as f64 / ITERATIONS as f64);

    let start = Instant::now();
    let mut contains_count = 0;
    for i in 0..ITERATIONS {
        if agrona_set.contains(i as i32) {
            contains_count += 1;
        }
    }
    let contains_time = start.elapsed();
    println!("Contains time: {:?} ({:.2} ns/op)",
             contains_time,
             contains_time.as_nanos() as f64 / ITERATIONS as f64);
    println!("Contains hits: {}", contains_count);

    println!("Final set size: {}", agrona_set.len());

    println!("\nStandard HashSet:");
    let start = Instant::now();
    let mut std_set = HashSet::new();

    for i in 0..ITERATIONS {
        std_set.insert(i as i32);
    }

    let std_insert_time = start.elapsed();
    println!("Insert time: {:?} ({:.2} ns/op)",
             std_insert_time,
             std_insert_time.as_nanos() as f64 / ITERATIONS as f64);

    let start = Instant::now();
    let mut std_contains_count = 0;
    for i in 0..ITERATIONS {
        if std_set.contains(&(i as i32)) {
            std_contains_count += 1;
        }
    }
    let std_contains_time = start.elapsed();
    println!("Contains time: {:?} ({:.2} ns/op)",
             std_contains_time,
             std_contains_time.as_nanos() as f64 / ITERATIONS as f64);
    println!("Contains hits: {}", std_contains_count);

    println!("Final set size: {}", std_set.len());

    let insert_speedup = std_insert_time.as_nanos() as f64 / insert_time.as_nanos() as f64;
    let contains_speedup = std_contains_time.as_nanos() as f64 / contains_time.as_nanos() as f64;

    println!("\nPerformance comparison:");
    println!("Insert speedup: {:.2}x", insert_speedup);
    println!("Contains speedup: {:.2}x", contains_speedup);

    println!("\nIteration test:");
    let start = Instant::now();
    let agrona_sum: i64 = agrona_set.iter().map(|x| *x as i64).sum();
    let agrona_iter_time = start.elapsed();

    let start = Instant::now();
    let std_sum: i64 = std_set.iter().map(|x| *x as i64).sum();
    let std_iter_time = start.elapsed();

    println!("Agrona iteration: {:?}, sum: {}", agrona_iter_time, agrona_sum);
    println!("Std iteration: {:?}, sum: {}", std_iter_time, std_sum);

    assert_eq!(agrona_sum, std_sum);
}

fn mutable_integer_demo() {
    println!("\n3. MutableInteger Demo");
    println!("---------------------");

    let mut counter = MutableInteger::new(0);
    println!("Initial value: {}", counter.get());

    println!("Increment: {}", counter.increment());
    println!("Increment: {}", counter.increment());
    println!("Add 10: {}", counter.add_and_get(10));

    println!("Get and add 5: {} (new value: {})",
             counter.get_and_add(5),
             counter.get());

    println!("Compare and set (17 -> 100): {}",
             counter.compare_and_set(17, 100));
    println!("Current value: {}", counter.get());

    println!("Compare and set (50 -> 200): {}",
             counter.compare_and_set(50, 200));
    println!("Current value: {}", counter.get());

    println!("\nPerformance test:");
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        counter.increment();
    }
    let elapsed = start.elapsed();

    println!("Final value: {}", counter.get());
    println!("Time for {} increments: {:?} ({:.2} ns/op)",
             ITERATIONS,
             elapsed,
             elapsed.as_nanos() as f64 / ITERATIONS as f64);

    counter.set(0);

    let start = Instant::now();
    for i in 0..ITERATIONS {
        if !counter.compare_and_set(i as i32, (i + 1) as i32) {
            println!("CAS failed at iteration {}", i);
            break;
        }
    }
    let cas_elapsed = start.elapsed();

    println!("CAS operations: {:?} ({:.2} ns/op)",
             cas_elapsed,
             cas_elapsed.as_nanos() as f64 / ITERATIONS as f64);
    println!("Final CAS value: {}", counter.get());
}