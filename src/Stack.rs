pub struct Stack<T>{
    data: Vec<T>
}



impl<T> Stack<T> {

    pub fn new() -> Self {
        Stack { data : Vec::new()}
    }

    pub fn push(&mut self, item_to_add:T){
        self.data.push(item_to_add);
    }

    //In pop operations, i transfer the ownership because i get the item out
    //of the stack
    pub fn pop(&mut self) -> Option<T>{
        return self.data.pop();
    }

    //In peek ones, i dont transfer it because i need to keep operating
    //with the item
    pub fn peek(&self) -> Option<&T>{
        return self.data.last();
    }

    pub fn is_empty(&self) -> bool{
        return self.data.is_empty();
    }

}