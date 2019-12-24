use std::time;
use std::thread;
use std::thread::sleep;

static NTHREADS: i32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    let sleep_time = time::Duration::from_millis(500);
    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            printer(i);
            sleep(sleep_time);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}


fn printer(i: i32) {
    println!("this is thread number {}", i);

}
