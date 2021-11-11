
use create_library::{Fern, run_simulation};

/// create a separate program to use the create_library
fn main() {
    let mut fern = Fern {
        size: 1.0, growth_rate: 0.001
    };
    run_simulation(&mut fern, 1000);
    println!("final fern size: {}", &fern.size);
}
