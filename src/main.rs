use rand::Rng;

trait WheelProducer {
    fn produce_wheel(&self) -> String;
}

struct CarFactory {
    wheel_producer: Box<dyn WheelProducer>,
}

struct SmallWheelProducer;
impl WheelProducer for SmallWheelProducer {
    fn produce_wheel(&self) -> String {
        "Small wheel".to_string()
    }
}

struct LargeWheelProducer;
impl WheelProducer for LargeWheelProducer {
    fn produce_wheel(&self) -> String {
        "Large wheel".to_string()
    }
}

fn get_producer() -> Box<dyn WheelProducer> {
    let size: i32 = rand::thread_rng().gen_range(1..=2);
    println!("size: {size}");
    if size > 1 {
        return Box::new(LargeWheelProducer);
    } else {
        return Box::new(SmallWheelProducer);
    }
}

fn main() {
    let wheel_producer = get_producer();
    let factory = CarFactory { wheel_producer };

    let wheel = factory.wheel_producer.produce_wheel();
    println!("Wheel produced: {}", wheel);
}
