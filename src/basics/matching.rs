//! Matching
mod matchtut {
    pub fn matchtut_examples() {
        let data = "Basic Fortran C C++ Python Java D Rust";
        match data.find('F') {
            Some(idx) => {
                let langs = &data[idx..];
                println!("Favorite programming languages:");
                for lang in langs.split_whitespace() {
                    println!("-> {}", lang);
                }
            }
            None => println!("Not a programmer! :)"),
        }
    }
}

pub fn matching_examples() {
    matchtut::matchtut_examples();
}
