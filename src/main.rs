
mod stack;
use stack::Stack;


fn main() {


    let mut stack1 : Stack<i32> = Stack::new();

    println!("{}", stack1.is_empty());


    stack1.push(23);
    stack1.push(24);
    stack1.push(25);
    stack1.push(26);
    stack1.push(27);
    stack1.push(28);
    stack1.push(29);


    for i in 0..9999990 {
        let a = stack1.pop();
        if( !stack1.is_empty())  {
            println!("{}",a.unwrap());
        }
    }

}
