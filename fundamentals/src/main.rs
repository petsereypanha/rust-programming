mod data_type;
mod if_else_if;
mod if_else;
mod functions;
mod loops;
mod ownership;
mod references;
mod enums;
mod hashmap;
mod slice;
mod string;

fn main() {
    data_type::data_type();
    if_else_if::if_else_if();
    if_else::if_else();
    functions::functions();
    loops::loops();
    ownership::ownership_1();
    references::references();
    enums::enums();
    hashmap::hashmap();
    slice::slice();
    string::string();
}