#![allow(dead_code)]
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

struct SpiderSenses {
    robot: Rc<SpiderRobot>
}

struct SpiderRobot {
    species: String,
    web_enabled: bool,
    hardware_error_count: Cell<i32>,
    log_file: RefCell<String>
}

impl SpiderRobot {

    fn new() -> Self {
        SpiderRobot {
            species: "Spider".to_string(),
            web_enabled: true,
            hardware_error_count: Cell::new(0),
            log_file: RefCell::new("File".to_string())
        }
    }

    /// Increases error count by 1
    fn add_hardware_errors(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    /// true if any hardware errors have been reported
    fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }

    /// write a line to the log file
    fn log (&self, message: &str) {
        // borrow a mutable reference from immutable reference
        let mut file = self.log_file.borrow_mut();
        // this will not compile
        writeln!(file, "{}", message).unwrap();
    }
}

fn main() {
    let robot = Rc::new(SpiderRobot::new());
    let senses = SpiderSenses {robot: robot.clone()};
    println!("Hello, world!");
}
