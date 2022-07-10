#[derive(Debug)]

//为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait LightTime{
    fn time(&self) -> u8 ;
}

impl LightTime for TrafficLight {
    fn time(&self) -> u8{
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 120,
            TrafficLight::Yellow => 3,
        }

    }
}

fn main() {
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    println!("Yellow(黄色) time is {:?}",yellow.time());
    println!("Red（红色） time is {:?}",red.time());
    println!("Green（绿色） time is {:?}",green.time());
}
