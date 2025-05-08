#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go"),
    }
}

fn main() {
    let light = TrafficLight::Red;
    action(light);
}