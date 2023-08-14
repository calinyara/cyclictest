# cyclictest in Rust

**cyclictest** is a program used primarily in Linux environments to measure the latency of a real-time system. Specifically, it helps determine the time difference between when an interrupt is generated and when it is serviced by the CPU. This is crucial in real-time systems, where predictable response times are essential for correct operation.


```
cyclictest 

USAGE:
    cyclictest <THREAD_COUNT> <SLEEP_DURATION_MS> <ITERATIONS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <THREAD_COUNT>         Number of threads to run
    <SLEEP_DURATION_MS>    Duration each thread should sleep for (in ms)
    <ITERATIONS>           Number of test iterations
```
