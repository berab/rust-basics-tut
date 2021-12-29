#[allow(unused_variables)]
// STACK:
// fixed sized variables can be in stack mem. Collections or vectors cannot be since they are not fixed in size
fn main() {
    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64);// can be solved by cloning but too expensive or we can return it again but is it really necessary since things can get more complex with more params?
    println!("In stack_procedure {}", heap_f64);//no longer owns any memory at all
}

fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
}