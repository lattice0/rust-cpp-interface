rustc --crate-type=staticlib interface.rs
g++ -o rust_c -L. interface.cpp -linterface -pthread -ldl
#rustc --crate-type=cdylib interface.rs