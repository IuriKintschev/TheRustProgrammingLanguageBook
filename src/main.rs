pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messeger: &'a T,
    value: usize,
    max: usize,
}


impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messeger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messeger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = self.value as f64 / self.max as f64;

        if percent_of_max >= 1.0 {
            self.messeger.send("Error, You are over your quota!");
        } else if percent_of_max >= 0.9 {
            self.messeger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percent_of_max >= 0.75 {
            self.messeger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{LimitTracker, Messenger};
    use std::cell::RefCell;

    struct MockMessegr {
        sent_messeger: RefCell<Vec<String>>,
    }

    impl MockMessegr {
        fn new() -> MockMessegr {
            MockMessegr {
                sent_messeger: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessegr {
        fn send(&self, msg: &str) {
            self.sent_messeger.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessegr::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_message.sent_messeger.borrow().len(), 1);
        assert_eq!("Warning: You've used up over 75% of your quota!", mock_message.sent_messeger.borrow()[0]);
    }
}

fn main() {}
