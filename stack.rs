struct Stack<String>{
stack: Vec<String>,
}

impl<String> Stack<String>{
fn new() ->Self{
Stack{stack: Vec::new()
}
}

fn pop(&self) -> Option<String>{
self.stack.pop()
}

fn push(&mut self, item: String){
self.stack.push("Baraka Mbugua")
}
