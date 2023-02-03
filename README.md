# Aeron client written in Rust

[![minimum rustc version](https://img.shields.io/badge/rustc-1.39+-green.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html)

## About *aeron-client*
Aeron is efficient reliable UDP and IPC message transport. Originally it was developed by RealLogic 
and hosted on GitHub [real-logic/aeron](https://github.com/real-logic/aeron)

Aeron has two main components: 
* Media driver
* Client library

The client library is linked with applications and allows several applications to talk with each
other through Media driver(s). For more information about how Aeron works please read 
[documentation](https://github.com/real-logic/aeron/wiki).

*aeron-client* library implements client functionality to work with Aeron Media driver. To get functioning system
one need to download (and compile) Media driver from [real-logic/aeron](https://github.com/real-logic/aeron) and write
some Aeron enabled application using *aeron-client*. 
Examples of such applications could be found inside the [bin](https://github.com/sheophe/aeron-client/tree/master/src/bin) and
in the library integration [tests](https://github.com/sheophe/aeron-client/tree/master/tests).

## Why Aeron?
Aeron could be a good choice if you need to organize fast and ordered message streams. There are number
of widely used messaging frameworks and to choose the right one for your particular case it is better to test 
them if close to real-life situations and compare the performance. 
Just some hints why Aeron could be a good choice:
* Supports various transmission media without the need to change application code when switching between the media. 
Application can use IPC (shared memory) today and could switch to UDP in a matter of config change and restart.
* Can work over UDP which is generally faster than over TCP (as some other messaging systems do)
* Provides reliable and ordered message flow (even over UPD). Not all fast messaging frameworks guaranty messages order but Aeron do

## Status
* Integration tests are successful on macOS and Linux
* Unit test are mainly successful (224/227 ok on macOS)

## Running library tests
Integration tests for *aeron-client* assume that Media Driver executable (aeronmd) is present in the PATH. So prior
to run these tests install *aeronmd* accordingly.
Also integration tests designed to run sequentially one by one. This is beacuse these tests execute *aeronmd* binary
to communicate with, and would conflict when run simultaneously. Therefore use
```
cargo test -- --test-threads=1
```

To run integration test only use
```
cargo test --test '*' -- --test-threads=1
```
command to run them.
