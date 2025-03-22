use procspawn::{self, spawn};

#[allow(clippy::empty_loop)]
fn main() {
    procspawn::init();
    let mut handle = spawn((), |()| while true {});
    handle.kill().unwrap();
}
