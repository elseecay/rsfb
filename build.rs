use cmake;
use bindgen;
use std::path::PathBuf;

fn main()
{
    std::env::set_var("CC", "/usr/bin/gcc"); // TODO: remove
    std::env::set_var("CXX", "/usr/bin/g++"); // TODO: remove
    let dst = cmake::build("cfbapi");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-flags=-l stdc++"); // TODO: ?

    let bindings = bindgen::Builder::default()
        .header("/usr/include/ibase.h")// TODO: ?
        .constified_enum("*")
        .prepend_enum_name(false)
        .layout_tests(false) // TODO: may be turn on
        .rustfmt_bindings(true)
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // TODO: no warnings for generated file
    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("detail/fbapi/ibase.rs"))
        .expect("Couldn't write bindings!");
}
