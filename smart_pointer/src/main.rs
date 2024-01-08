use std::cell::RefCell;
use std::ops::Deref;

fn main() {
    let smart_pointer = MyBox::new("test");

    println!("MyBox value: {:?}", smart_pointer);
    println!("dereferenced MyBox value: {:?}", *smart_pointer);
    println!("dereferenced MyBox value: {:?}", *smart_pointer);
    println!("dereferenced MyBox value: {:?}", *smart_pointer);
    println!("deref_counter: {}", smart_pointer.get_deref_counter());
}


#[derive(Debug)]
struct MyBox<T> {
    data: T,
    deref_counter: RefCell<u32>
}

impl<T> MyBox<T> {
    fn new(data: T) -> MyBox<T> {
        MyBox{
            data: data,
            deref_counter: RefCell::new(0),
        }
    }

    fn get_deref_counter(&self) -> u32 {
        *self.deref_counter.borrow()
    }

    fn inc_deref_counter(&self) {
        *self.deref_counter.borrow_mut() += 1;
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inc_deref_counter();

        &self.data
    }
}