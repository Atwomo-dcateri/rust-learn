/*pub enum List{

    Cons(i32, List),
    Nil,
}*/

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);
impl<T> MyBox<T>{
    pub fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

pub struct CustomSmartPointer{
    pub data: String,
}
impl  Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

pub trait Messager {
    fn send(&self, msg: &str) {
        
    }
} 

pub struct LimitTracker<'a, T: Messager>{
    messager: &'a T,
    value: usize,
    max: usize
}

impl <'a, T> LimitTracker<'a, T>
    where T: Messager {
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messager: messager,
            value: 0,
            max: max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {

            self.messager.send("Error: You are over your quota!");
        }else if  percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max  >= 0.75{
            self.messager.send("Warning: You've used up 75% of your qouta!");
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                //sent_messages: vec![]
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messager for MockMessenger {
        /*fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message))
        }*/
        
        fn send (&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));

        }
    }

    

    #[test]
    fn is_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        //assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
