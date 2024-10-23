fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(255);
    // u8 unsigned  integer until 255 is okay but more than this gives error like 256
}
