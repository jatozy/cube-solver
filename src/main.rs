use cube_solver_hardware::cube_solver_hardware;

fn main() {
    let mut controller = cube_solver_hardware::controller::Controller::new();
    for _ in 0..5 {
        let rotation_finished = controller.rotate_turntable_90_degree_clockwise();
        rotation_finished.recv().unwrap();
        println!("Rotation Clockwise finished.");
        std::thread::sleep(std::time::Duration::from_secs(3));
        let rotation_finished = controller.rotate_turntable_90_degree_counter_clockwise();
        rotation_finished.recv().unwrap();
        println!("Rotation Counter Clockwise finished.");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
