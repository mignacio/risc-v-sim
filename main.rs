pub mod registers;

fn main(){
    println!("LENGTH {} WIDTH {}", registers::REGISTER_LENGTH, registers::REGISTER_WIDTH);

    registers::set(15, 15);
    registers::print();
}