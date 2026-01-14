mod ping;
use std::env;
use colored::Colorize;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let args2 : Vec<&str> = args.iter().map(|x| x.as_str()).collect();


    match args2.as_slice()
    {
        [_bin, "--help"] | [_bin, "-h"] => { help(); }
        [_bin, "--about"] | [_bin, "-a"] => { about(); }
        [_bin, ip ] => { ping::ping(ip, None, None); }
        [_bin, ip, "-p" ,port] => {
            ping::ping(ip, port.parse::<u16>().ok(), None);
        }
        [_bin, ip, "-p" ,port, "-t",timeout] => {
            ping::ping(ip, port.parse::<u16>().ok(), timeout.parse::<u16>().ok());
        }
        _ => { help(); }
    }
}


fn help()
{
    eprintln!(
        "{}", "Usage: {bin} <IP> [-p <port>] [-t <timeout>] \
    Options: -h, --help Show this help \
    -a, --about show about this util
    ".bright_red().bold()
    );
}

fn about(){
    eprintln!("
        Mussk / C-Ping
        Github: https://github.com/pupsikdhd
        MIT license
        @pupsikdhd - all rights reversed
    ")
}