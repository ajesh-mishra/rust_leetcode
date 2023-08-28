pub struct MyStack {
    stack: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack { stack: Vec::new() }
    }
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }
    fn pop(&mut self) -> i32 {
        if let Some(x) = self.stack.pop() {
            return x;
        }
        0
    }
    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }
    fn empty(&self) -> bool {
        self.stack.len() == 0
    }
}

pub fn test() {
    let mut obj = MyStack::new();
    obj.push(4);
    let ret_2: i32 = obj.pop();
    let ret_3: i32 = obj.top();
    let ret_4: bool = obj.empty();
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
    println!("{:?}", ret_4);
}
