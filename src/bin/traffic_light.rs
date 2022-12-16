trait TrafficLight {
    fn duration(&self) -> u64;
}

enum LightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLight for LightColor {
    fn duration(&self) -> u64 {
        match self {
            LightColor::Red => 10,
            LightColor::Yellow => 3,
            LightColor::Green => 15,
        }
    }
}

fn main() {
    println!(
        "The duration of the red light is: {}",
        LightColor::Red.duration()
    );
    println!(
        "The duration of the yellow light is: {}",
        LightColor::Yellow.duration()
    );
    println!(
        "The duration of the green light is: {}",
        LightColor::Green.duration()
    );
}
