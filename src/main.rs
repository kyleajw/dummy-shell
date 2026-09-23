use std::env;
use std::format;
use std::fs;
use std::io;
use std::io::Write;
use std::io::stdout;
use std::path::Path;
use std::path::PathBuf;
use std::println;
use sysinfo::{
    Components, Disks, Networks, System
};


fn main() -> std::io::Result<()>{
    print!("\x1b[2J\x1b[1;1H");
    let mut current_dir = env::current_dir()?;
    loop {
        print!("\x1b[32m{}>>\x1b[0m", current_dir.display());
        stdout().flush().expect("FLUSH FAILURE"); // stdout not flushed implicity when print! is called
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("READLINE FAILURE");
        line = String::from(line.trim());
        let mut args: Vec<&str> = line.split(' ').collect();
        
        match args.remove(0){
            "cd" => change_dir(&mut args, &mut current_dir),
            "ls" => list_directory(&mut args, &current_dir),
            "fetch" => fetch_sysinfo(),
            "cls" => clear(),
            "exit" => return Ok(()),
            _=>println!("\x1b[31mINVALID COMMAND\x1b[0m"),
        }
    }
}

fn change_dir(args: &mut Vec<&str>, current_dir: &mut PathBuf){
    // if args is empty, display current dir, else change directory 
    // https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/cd
    if args.is_empty(){
        println!("{}", current_dir.display());
    }else{
        let path = args.join(" ");
        if dir_exists(&path, current_dir){
            current_dir.push(PathBuf::from(path));

            simplify_path(current_dir);
        }else{
            println!("\x1b[31mINVALID PATH\x1b[0m");
        }
    }
}

fn clear(){
    print!("\x1b[2J\x1b[1;1H");
}

fn dir_exists(path: &str, current_dir: &mut PathBuf) -> bool{
    Path::new(&current_dir.join(path)).is_dir()
}

fn simplify_path(path: &mut PathBuf){
    let full_path = path.display().to_string().replace("/", "\\");
    let path_stack: Vec<&str> = full_path.split('\\').collect();
    let mut stack: Vec<&str> = Vec::new();
    for w in path_stack{
        if w == ".." && !stack.is_empty(){
            stack.pop();
        }
        else if w != "." && w != ""{
            stack.push(w);
        }
    }
    // println!("{:?}", stack);
    if stack.is_empty(){
        *path = PathBuf::from("\\");
    }else if stack.len() == 1{
        let p = stack.remove(0).to_owned() + "\\";
        *path = PathBuf::from(p)
    
    }else{
        let mut p = String::from(stack.remove(0));
        
        for w in stack{
            let s = "\\".to_owned() + w;
            p.push_str(s.as_str());
        }
        *path = PathBuf::from(p);
    }
}

fn list_directory(args: &mut Vec<&str>, current_dir: &PathBuf){
    for item in fs::read_dir(current_dir).expect("READ ERROR"){
        let entry = item.expect("ENTRY ERROR");
        let mut entry_string = String::new();
        let path = entry.path();
        if path.is_dir(){
            entry_string = format!("\x1b[1;36m{}\x1b[0m\x1b[22m",entry.file_name().into_string().expect("ENTRY STRING CONVERSION ERROR"));
        }
        else{
            entry_string = format!("{}", entry.file_name().into_string().expect("ENTRY STRING CONVERSION ERROR"))
        }
        println!("{}", entry_string);
    }
}

fn fetch_sysinfo(){
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("OS:       {} ({})", System::long_os_version().unwrap(), System::cpu_arch());
    println!("Host:     {}", System::host_name().unwrap());
    println!("Kernel:   {}", System::kernel_long_version());
    println!("Uptime:   {}", uptime_long(System::uptime()));
    println!("Shell:    dummy-shell");
    println!("Memory:   {:.2}GB/{:.2}GB", sys.used_memory() as f64/1073741824 as f64, sys.total_memory() as f64/1073741824 as f64);
    sys.refresh_cpu_usage();
    println!("CPU Usage {}%", sys.global_cpu_usage())
}

fn uptime_long(uptime: u64) -> String{
    let mut r = uptime;
    let mut hours = 0;
    let mut mins = 0;
    let mut uptime_str = String::new();
    if uptime > 3600{
        hours = uptime / 3600;
        r = uptime - (hours * 3600);
        uptime_str.push_str(&format!("{} hours, ", hours));
    }
    if r > 60{
        mins = r / 60;
        r = r - (mins * 60);
        uptime_str.push_str(&format!("{} minutes, ", mins));

    }
    uptime_str.push_str(&format!("{} seconds", r));
    uptime_str
}