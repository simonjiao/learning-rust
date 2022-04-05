# Homework for `OneBlock+` Substrate Basic Lesson 2
## Description
> 使用Rust std标准库的功能实现一个tcp server, 要求:
> 1. 能正常运行
> 2. 对tcp client(比如可用telnet等)发过来的消息，并做echo返回
> 3. 对代码每一句做注解
> 4. 做一次标准的错误处理(模式匹配)
## Usage
### Build
> cargo build --release
### Build and Run
1. Use loopback address and random unused port number
    > Cargo run --release

2. Use specified address and random unused port number
    > Cargo run --release -- 10.10.10.128

3. Use specified address and port number
    > Cargo run --release -- 127.0.0.1 4444