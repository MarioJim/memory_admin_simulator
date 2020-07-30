# memory_admin_simulator

Final project for Operating Systems written in Rust

A toy simulator used to compare between two page replacement algorithms:

- First In First Out
- Least Recently Used

The simulator has 2048 bytes of real memory and 4096 bytes of virtual memory divided into frames of 16 bytes by default, but these values can be changed.

## Usage

It works by parsing a plain text file with five instruction types:

### P \<bytes: u16> \<pid: u16>

It loads a process with pid `pid` and size `bytes` into real memory, it can't be greater than the real memory size. Also, every frame loaded from disk takes a second.

### A \<address: u16> \<pid: u16> \<modifies: bool>

Accesses an address at `address` of process `pid`. If `modifies` is true it logs another message. It takes 0.1 seconds, and if the page isn't on real memory and it has to be loaded from virtual memory it takes 1 more second.

### L \<pid: u16>

Frees the frames allocated by a process `pid` from the real and virtual memory. It takes 0.1 seconds per page.

### C \<comment: String>

It logs the string `comment` into the output.

### F

It resets the simulator (empties both memories and resets time) and prints the following statistics:

- Turnaround time (per process and average)
- Page faults per process
- Number of swap-ins and swap-outs

### E

It ends the simulation and prints an exit message
