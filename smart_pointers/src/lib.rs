#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell; // allows interior mutability

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
			// borrow_mut used with RefCell to mutate vec through push
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
		let mock_messenger = MockMessenger::new();
		// limit_tracker only has immutable reference to mock_messenger
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

		/* will result in mutate of mock_messenger's sent_messages vec
		 * works because we wrapped the vec in a RefCell
		 */
        limit_tracker.set_value(80);

		// we also use borrow here to account for the RefCell wrapper
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        /* had we tried another route like altering the Messenger trait
         * signature to take mutable reference we would have also been 
         * disallowed, as borrowing rules would prevent a mutable borrow
         * after LimitTracker's immutable borrow. Making the borrow from
         * LimitTracker mutable would also been disallowed then, as 
         * mutating self.messenger.sent_messages would require a second 
         * mutable borrow.
         */
    }
}

pub trait Messenger {
    fn send(&self, msg: &str); // immutable reference in signature
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T, // immutable reference to messenger struct
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
		// all these send calls only borrow messenger immutably!
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
