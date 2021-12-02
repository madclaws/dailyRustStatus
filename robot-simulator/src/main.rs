use robot_simulator::*;
fn main() {
    let robot = Robot::new(0, 0, Direction::North).instructions("LAAARALA");
    // robot.instructions("LAAARALA");
    println!("{:?}", robot.direction());
}