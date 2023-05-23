
struct MinStack {
    s1:Vec<i32>,
    s2:Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack { s1: vec![], s2: vec![] }
    }
    
    fn push(&mut self, val: i32) {
        self.s1.push(val);
        if self.s2.is_empty(){
            self.s2.push(val);
        }else{
            self.s2.push(*self.s2.last().unwrap().min(&val));
        }
    }
    
    fn pop(&mut self) {
        self.s1.pop();
        self.s2.pop();
    }
    
    fn top(&self) -> i32 {
        *self.s1.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.s2.last().unwrap()
    }
}