use robot_simulator::*;
fn main() {
    let mut robot = Robot::new(0, 0, Direction::North);
    robot.instructions("LAAARALA");
    println!("{:?}", robot.direction());
}