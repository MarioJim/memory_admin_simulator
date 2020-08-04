# memory_admin_simulator

## Team 7
- Mario Emilio Jiménez Vizcaíno A01173359
- Karla Fernanda Ceseña Aragón A00818843
- Kevin Torres Martínez A01656257
- Ana Paula Aguilar Favela A01192323

<br/>

## Additional Info
| language | Version | Lines of code       | Lines of code w/o comments    |
|----------|---------|---------------------|-------------------------------|
|  Rust    |  1.45.2 |        1211         |              932              |

<br/>

Final project for Operating Systems written in Rust.

A toy simulator used to compare between some page replacement algorithms:

- First In First Out
- Least Recently Used
- Randomly selected

The simulator has 2048 bytes of real memory and 4096 bytes of swap space divided into frames of 16 bytes by default, but these values can be changed.

<br/>

***

## Index 
1. [Usage](#Usage)
2. [Installation guide](#Installation-process)
3. [Execution guide](#Execution-process)

<br/>

***

## Usage

It works by parsing a plain text file with five instruction types:

### P \<bytes: u16> \<pid: u16>

It loads a process with pid `pid` and size `bytes` into real memory, it can't be greater than the real memory size. Also, every frame loaded from disk takes a second.

### A \<address: u16> \<pid: u16> \<modifies: bool>

Accesses an address at `address` of process `pid`. If `modifies` is true it logs another message. It takes 0.1 seconds, and if the page isn't on real memory and it has to be loaded from the swap space it takes 1 more second.

### L \<pid: u16>

Frees the frames allocated by a process `pid` from the real memory and the swap space. It takes 0.1 seconds per page.

### C \<comment: String>

It logs the string `comment` into the output.

### F

It resets the simulator (empties both memories and resets time) and prints the following statistics:

- Turnaround time per process
- Average turnaround time
- Number of swap-ins and swap-outs

### E

It ends the simulation and prints an exit message

<br/>

***

## Installation process

It is necessary to follow a series of steps to be able to execute Rust code. You can follow the steps in this file or check it from [Rust installation - Official page](https://www.rust-lang.org/tools/install).

<br/>

### Install Rust
First you need to install ***Rust***. To install it in MacOS you only need to use the ***Curl*** utility (it comes natively in MacOS). Execute the following command from in the terminal:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br/>

### Configure current shell with Cargo
After installing rust in your computer, you need to configure your current shell to use Cargo (Rust package manager). Execute the following command from in the terminal:

```
source $HOME/.cargo/env
```
***Note:*** You can confirm if Cargo was installed correctly by running the command:
```
cargo
```

<br/>

***

## Execution process
To execute rust you can use Cargo. From the terminal located in the project directory, you will only need to execute this command:

```
cargo run <algorithm> <file>
```
***Note:*** It is important to know that sometimes it is required to add some arguments to execute the command, which is the case of our project.

<br/>

Execute the project using the FIFO algorithm:
```
cargo run fifo test1.txt
```
Execute the project using the LRU algorithm:
```
cargo run lru test1.txt
```
