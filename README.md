## High performance sync style Rust logging facility dedicated for Linux

Baselog is modified from the [simplelog](https://github.com/Drakulix/simplelog.rs). 

Plus a little more we want：
1. highest performance under Linux (but without async)
    + lock free and nvme disk bandwidth saturation
2. file rotating support
3. loggers opt-ins are more flexible 

### Why `no async`?
In fact, high performance logging is a false proposition.

What we want is to maximize the total performance of a system with minimized logging overheads. 

### License
Baselog is distributed under the terms of the Apache License (Version 2.0).

### Credits
* [simplelog](https://github.com/Drakulix/simplelog.rs)



