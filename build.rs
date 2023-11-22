fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/cppfile.cc")
        
        //.flag_if_supported("-std=c++14")
        .compile("stm32f4-cxx");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cppfile.cc");
    println!("cargo:rerun-if-changed=src/cppfile.h");
}