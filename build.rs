use cmake;

fn main()
{
    std::env::set_var("CC", "/usr/bin/gcc"); // TODO: remove
    std::env::set_var("CXX", "/usr/bin/g++"); // TODO: remove
    let dst = cmake::build("cfbapi");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-flags=-l stdc++"); // TODO: ?
}
