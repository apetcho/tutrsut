//! Files, Paths and Directories

use std::env;
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

pub fn fsfpathdirs_examples() {
    fpath1::fpath1_examples();
}
