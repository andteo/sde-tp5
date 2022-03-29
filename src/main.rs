use nix::libc::fork;

fn main() {
    /*let mut line =String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let lstr=line.as_str();
    let mut b=Box::new(String::from("Asta e un text fix".to_string()));
    b.push_str(lstr);
    println!("{}",b);*/ //ex1

    unsafe{
        fork()
    }
}
