# Rust Agrona Architecture Diagram

```mermaid
graph TB
    %% Main Application Layer
    APP[HFT Application]

    %% Rust Agrona Crates
    subgraph "Rust Agrona Workspace"
        EXAMPLES[agrona-examples]
        CONCURRENT[agrona-concurrent]
        COLLECTIONS[agrona-collections]
        CORE[agrona-core]
    end

    %% Core Components
    subgraph "agrona-core"
        DIRECT[DirectBuffer Trait]
        MUTABLE[MutableBuffer Trait]
        UNSAFE[UnsafeBuffer Implementation]
        UTILS[Buffer Utilities]
    end

    subgraph "agrona-collections"
        INTHASH[IntHashMap<V>]
        INTSET[IntHashSet]
        HASHING[Hashing Utilities]
        ITER[Iterators]
    end

    subgraph "agrona-concurrent"
        ATOMIC[AtomicBuffer]
        RINGBUF[Ring Buffers]
        LOCKFREE[Lock-free Structures]
        AGENTS[Agent Framework]
    end

    subgraph "agrona-examples"
        PERF[Performance Tests]
        DEMOS[Usage Examples]
        BENCH[Benchmarks]
    end

    %% Memory Layer
    subgraph "Memory Management"
        HEAP[Heap Memory]
        STACK[Stack Memory]
        MMAP[Memory Mapped Files]
        DIRECT_MEM[Direct Memory Access]
    end

    %% Hardware Layer
    subgraph "Hardware"
        CPU[CPU Caches]
        RAM[System RAM]
        SIMD[SIMD Instructions]
    end

    %% Connections
    APP --> EXAMPLES
    APP --> CONCURRENT
    APP --> COLLECTIONS
    APP --> CORE

    EXAMPLES --> CONCURRENT
    EXAMPLES --> COLLECTIONS
    EXAMPLES --> CORE

    CONCURRENT --> CORE
    COLLECTIONS --> CORE

    CORE --> DIRECT
    CORE --> MUTABLE
    CORE --> UNSAFE
    CORE --> UTILS

    COLLECTIONS --> INTHASH
    COLLECTIONS --> INTSET
    COLLECTIONS --> HASHING
    COLLECTIONS --> ITER

    CONCURRENT --> ATOMIC
    CONCURRENT --> RINGBUF
    CONCURRENT --> LOCKFREE
    CONCURRENT --> AGENTS

    EXAMPLES --> PERF
    EXAMPLES --> DEMOS
    EXAMPLES --> BENCH

    UNSAFE --> HEAP
    UNSAFE --> STACK
    UNSAFE --> MMAP
    UNSAFE --> DIRECT_MEM

    DIRECT_MEM --> CPU
    DIRECT_MEM --> RAM
    UNSAFE --> SIMD

    %% Styling
    classDef crate fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    classDef trait fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef impl fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef hardware fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef memory fill:#fce4ec,stroke:#880e4f,stroke-width:2px

    class CORE,COLLECTIONS,CONCURRENT,EXAMPLES crate
    class DIRECT,MUTABLE trait
    class UNSAFE,INTHASH,INTSET,ATOMIC impl
    class CPU,RAM,SIMD hardware
    class HEAP,STACK,MMAP,DIRECT_MEM memory
```

## Performance Flow

```mermaid
graph LR
    %% Performance Critical Path
    subgraph "Ultra-Low Latency Path"
        INPUT[Market Data] --> PARSE[Parse ASCII Numbers]
        PARSE --> STORE[Store in Buffer]
        STORE --> LOOKUP[Collection Lookup]
        LOOKUP --> PROCESS[Process Logic]
        PROCESS --> OUTPUT[Send Orders]
    end

    %% Performance Numbers
    subgraph "Performance Metrics"
        P1["put_u32: 0.64 ns"]
        P2["get_u32: 1.12 ns"]
        P3["IntHashMap lookup: 7.36 ns"]
        P4["Bulk ops: 36.2 GB/s"]
    end

    STORE -.-> P1
    STORE -.-> P2
    LOOKUP -.-> P3
    STORE -.-> P4

    %% Styling
    classDef perf fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px
    classDef path fill:#e3f2fd,stroke:#1565c0,stroke-width:2px

    class P1,P2,P3,P4 perf
    class INPUT,PARSE,STORE,LOOKUP,PROCESS,OUTPUT path
```

## Memory Layout

```mermaid
graph TD
    subgraph "UnsafeBuffer Memory Layout"
        PTR[Raw Pointer *mut u8]
        CAP[Capacity: usize]
        OWNED[Owned: bool]
    end

    subgraph "Buffer Memory"
        CACHE[Cache-Aligned (64 bytes)]
        DATA[Contiguous Data Layout]
        BOUNDS[Bounds Checking (Optional)]
    end

    subgraph "Collections Memory"
        ENTRIES[Hash Table Entries]
        PROBE[Linear Probing]
        PRIMITIVE[Primitive Values (No Boxing)]
    end

    PTR --> CACHE
    CACHE --> DATA
    DATA --> BOUNDS

    ENTRIES --> PROBE
    PROBE --> PRIMITIVE

    %% Styling
    classDef memory fill:#ffebee,stroke:#c62828,stroke-width:2px
    classDef layout fill:#f1f8e9,stroke:#33691e,stroke-width:2px

    class PTR,CAP,OWNED memory
    class CACHE,DATA,ENTRIES,PRIMITIVE layout
```