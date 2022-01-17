//! Files, Paths and Directories

use std::env;
use std::path::Path;
use std::path::PathBuf;

mod fpath1 {
    use super::*;

    pub fn fpath1_examples() {
        //let home = env::home_dir().unwrap(); //.expect("no home");
        let mut path = PathBuf::new();
        #[allow(deprecated)]
        match env::home_dir() {
            Some(home) => path.push(home),
            None => println!("no home"),
        }
        //path.push(home);
        path.push(".cargo");

        if path.is_dir() {
            println!("{}", path.display());
        }
    }
}

//
mod fpath2 {
    use super::*;

    pub fn fpath2_examples() {
        let mut path = env::current_dir().expect("Can't access current dir");
        loop {
            println!("-> {}", path.display());
            if !path.pop() {
                break;
            }
        }
    }
}

//
mod fpath3 {
    use super::*;

    //
    pub fn fpath3_examples() {
        let mut path = env::current_dir().expect("Can't access current dir");
        loop {
            path.push("config.txt");
            if path.is_file() {
                println!("gotcha {}", path.display());
                break;
            } else {
                path.pop();
            }

            if !path.pop() {
                break;
            }
        }
    }
}

// ---
mod fpath4 {
    use super::*;

    pub fn fpath4_examples() {
        let file = env::args().skip(1).next().unwrap_or("mod.rs".to_string());
        let path = Path::new(&file);
        match path.metadata() {
            Ok(data) => {
                println!("type: {:?}", data.file_type());
                println!("len : {}", data.len());
                println!("perm: {:?}", data.permissions());
                println!("modified: {:?}", data.modified());
            }
            Err(e) => println!("error {:?}", e),
        }
    }
}
// ---
pub fn fsfpathdirs_examples() {
    fpath1::fpath1_examples();
    fpath2::fpath2_examples();
    fpath3::fpath3_examples();
    fpath4::fpath4_examples();
}
