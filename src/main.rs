use cube_solver_hardware::cube_solver_hardware;

fn main() {
    let mut controller = cube_solver_hardware::controller::Controller::new();
    for _ in 0..5 {
        let move_finished = controller.move_carriage_to_front();
        move_finished.recv().unwrap();
        println!("Move Carriage to Front finished.");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let move_finished = controller.move_carriage_to_back();
        move_finished.recv().unwrap();
        println!("Move Carriage to Back finished.");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
