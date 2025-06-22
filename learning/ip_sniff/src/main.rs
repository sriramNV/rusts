use std::env;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2{
            return Err("not enough arguments");
        }else if args.len() > 4{
            return Err("too many arguments");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
}

