
static mut STASH: &i32 = &100;
static WORTH_POINTING_AT: i32 = 1000;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
        println!("stash value: {}", STASH);
    }
}

fn main() {
    f(&WORTH_POINTING_AT);
}
