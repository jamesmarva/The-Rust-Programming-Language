use std::cell::RefCell;


trait Messenger {

    fn send(&self, messsage: &str);
}


struct MockMessenger {
    sent_messages_vec: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger{
            // sent_messages_vec: RefCell::new(Vec::new()),
            sent_messages_vec: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages_vec.borrow_mut();
        // let mut two_borro = self.sent_messages_vec.borrow_mut();
    }
}


mod tests {
    use super::*;
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        mock_messenger.send("asfasfdasfas");

    }
}