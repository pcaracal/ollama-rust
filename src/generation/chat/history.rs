use std::sync::{Arc, Mutex, MutexGuard};

use crate::generation::chat::message::Message;

/// This struct uses Arc internally and is safe to be shared/cloned across threads.
#[derive(Debug, Clone, Default)]
pub struct History {
    inner: Arc<Mutex<Vec<Message>>>,
}

impl History {
    /// Push a message to the history.
    ///
    /// # Panics
    /// Locking the inner mutex can panic, but it probably never will.
    pub fn push(&self, new_message: &Message) {
        let mut messages = self.inner.lock().unwrap();
        Self::push_with_merge(&mut messages, new_message);
    }

    /// Extend the history with multiple messages.
    ///
    /// # Panics
    /// Locking the inner mutex can panic, but it probably never will.
    pub fn extend(&self, new_messages: &[Message]) {
        let mut messages = self.inner.lock().unwrap();

        for message in new_messages {
            Self::push_with_merge(&mut messages, message);
        }
    }

    fn push_with_merge(messages: &mut MutexGuard<Vec<Message>>, new_message: &Message) {
        if let Some(last) = messages.last_mut()
            && !last.done
            && last.role == new_message.role
        {
            last.merge_from(new_message);
        } else {
            messages.push(new_message.clone());
        }
    }

    /// Clone and return all messages in the history.
    ///
    /// # Panics
    /// Locking the inner mutex can panic, but it probably never will.
    #[must_use]
    pub fn messages(&self) -> Vec<Message> {
        let messages = self.inner.lock().unwrap();
        messages.clone()
    }

    /// Clone and return the last message in the history, if any.
    ///
    /// # Panics
    /// Locking the inner mutex can panic, but it probably never will.
    #[must_use]
    pub fn last(&self) -> Option<Message> {
        let messages = self.inner.lock().unwrap();
        messages.last().cloned()
    }
}
