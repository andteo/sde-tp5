use std::{thread, time};
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write, Pid, getpid, getppid}};

fn main() {
    /*let mut line =String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let lstr=line.as_str();
    let mut b=Box::new(String::from("Asta e un text fix".to_string()));
    b.push_str(lstr);
    println!("{}",b);*/ //ex1

    match unsafe{fork()}{
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Eu sunt parintele, pidul meu e {}, iar copilul are pid {}",getpid(), child);
            println!("Astept sa termine copilul");
            thread::sleep(time::Duration::from_secs(1));
            waitpid(child, None).unwrap();
            println!("A terminat, gata");
        }
        Ok(ForkResult::Child) => {
            println!("Eu sunt copilul, am pidul {}, iar parintele are pid {}", getpid(), getppid());
            thread::sleep(time::Duration::from_secs(5));
            println!("Am asteptat 5 secunde inainte sa afisez asta");
        }
        Err(_) => println!("Fork failed"),
    }
}
