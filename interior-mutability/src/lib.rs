pub trait Messenger{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T:Messenger>
{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T>
    {
        LimitTracker
        {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(& mut self, value: usize)
    {
        self.value = value;

        let percent_of_max = self.value as f64/ self.max as f64;

        if percent_of_max >= 1.0
        {
            self.messenger.send("Error! You are over your quota!");
        }
        else if percent_of_max >= 0.9
        {
            self.messenger.send("Urgent warning: You have used over 90% of your quota");
        }
        else if percent_of_max >= 0.75
        {
            self.messenger.send("Warning: You have used over 75% of your quota");
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger
    {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger
    {
        pub fn new() -> MockMessenger
        {
            MockMessenger
            {
                sent_messages : RefCell::new(Vec::new())
            }
        }
    }

    impl Messenger for MockMessenger
    {
        fn send(&self, msg: &str)
        {
            self.sent_messages.borrow_mut().push(String::from(msg));
            // two sequential mut ref are allowed
            // self.sent_messages.borrow_mut().push(String::from(msg));

             // but two parallel mut ref are allowed or not forgetting mut will lead to runtime panic
            //let mut one_borrow = self.sent_messages.borrow_mut();
            //let mut two_borrow = self.sent_messages.borrow_mut();

            //one_borrow.push(String::from(msg));
            //two_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn limit_tracker_sends_a_message()
    {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 100);
        tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}