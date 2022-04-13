# Homework for `Oneblock+` Substrate Basic Lesson 4
# Description
> 1. 为枚举交通信号灯实现一个trait, trait里包含一个返回时间的方法，不同的灯持续时间不同.
> 2. 实现一个函数, 为u32类型的整数集合求和, 参数类型为 &[u32], 返回类型为Option<u32>, 溢出时返回None.
> 3. 实现一个打印图形面积的函数, 它接收一个可以计算面积的类型作为参数, 比如圆形, 三角形, 正方形, 需要用到泛型和泛型约束.

# Build
> cargo build --release

# Test
> cargo test

# Run
> cargo run --release --bin stoplight  
> cargo run --release --bin stoplight red  
> cargo run --release --bin stoplight brown

> cargo run --release --bin sum 1 2 3  
> cargo run --release --bin sum  
> cargo run --release --bin sum 3 d  

> cargo run --release --bin area circle 1  
> cargo run --release --bin area triangle 3 4 5  
> cargo run --release --bin area square 3