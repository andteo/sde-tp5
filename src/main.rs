use std::ffi::{CStr, CString};
use std::fs::File;
use std::{thread, time};
use std::process::Command;
use nix::libc::dup2;
use nix::unistd::execvp;
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, Pid, getpid, getppid}};
use std::os::unix::io::AsRawFd;

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
            let status = waitpid(child, None).unwrap();
            println!("A terminat, gata, {:?}",status);
        }
        Ok(ForkResult::Child) => {
            println!("Eu sunt copilul, am pidul {}, iar parintele are pid {}", getpid(), getppid());
            thread::sleep(time::Duration::from_secs(5));
            println!("Am asteptat 5 secunde inainte sa afisez asta");
            let comanda=CString::new("ls").unwrap();
            let l= CString::new("-l").unwrap();
            let mut file = File::create("foo.txt")?;
            dup2(file.as_raw_fd(), 1);
            execvp(&comanda, &[comanda.clone(), l]);
            std::process::exit(30)
        }
        Err(_) => println!("Fork failed"),
    }
}
