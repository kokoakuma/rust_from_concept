mod module_hello;
use module_hello::module_hello_2::print_hello_2;
fn main() {
    module_hello::print_hello();  
    print_hello_2(); 
}