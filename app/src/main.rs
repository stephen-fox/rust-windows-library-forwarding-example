use std::io;

#[link(name = "targetlib.dll", kind = "dylib")]
unsafe extern "C" {
    fn add(left: u64, right: u64) -> u64;
}

fn main() {
    let result = unsafe { add(100, 200) };

    println!("add function returned: {result}");

    println!("press enter to exit");

    let _ = io::stdin().read_line(&mut String::new());
}
