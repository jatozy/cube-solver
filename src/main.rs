use cube_solver_hardware::cube_solver_hardware;

fn main() {
    let mut controller = cube_solver_hardware::controller::Controller::new();
    for _ in 0..2 {
        let mut move_finished = controller.move_carriage_to_front();
        move_finished.recv().unwrap();
        move_finished = controller.rotate_turntable_90_degree_clockwise();
        move_finished.recv().unwrap();
        move_finished = controller.move_carriage_to_back();
        move_finished.recv().unwrap();
        move_finished = controller.rotate_turntable_90_degree_counter_clockwise();
        move_finished.recv().unwrap();
    }
}
