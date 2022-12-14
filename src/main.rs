mod signal;
mod sum;
mod area;

use signal::Duration;
use sum::sum;

fn main() {
    println!("================================================================================");
    println!("05-为枚举通信号灯实际发现一个trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同:");
    let red_signal = signal::Signal::Red;
    let green_signal=signal::Signal::Green;
    println!("Red signal duration: {}", red_signal.duration());
    println!("Green signal duration: {}", green_signal.duration());
    println!("================================================================================");
    println!("06-实现一个函数，为u32类型的整数集合请求和，参考类型为&[u32]，返回类型为Option，溢出时返回None:");
    let numbers = [1, 2, 3, 4];
    let result = sum(&numbers);
    println!("Sum: {:?}", result);
    println!("================================================================================");
    println!("07-实现一个打印图形面积的函数，它接收一个可以计算面积的类函数为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束:");
}
