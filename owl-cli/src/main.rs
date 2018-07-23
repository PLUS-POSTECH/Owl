use std::io;
use std::env;

fn monitor() {
    // show game status
    println!("monitor");
}

fn submit() {
    // submit exploit request
    println!("submit");
}

fn test() {
    // execute exploit on the server to test
    println!("test");
}


fn main() {
    for (cnt, arg) in env::args().enumerate() {
        if cnt == 0 { continue; }

        match arg.as_ref() {
            "monitor" => monitor(),
            "exploit" => println!("exploit"),
            "submit"  => submit(),
            "test"    => test(),
            _         => println!("Default"),
        }
    }
}
