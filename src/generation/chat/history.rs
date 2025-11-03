use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};

use crate::generation::chat::message::Message;

/// This struct uses Arc internally and is safe to be shared/cloned across threads.
#[derive(Debug, Clone, Default)]
pub struct History {
    inner: Arc<Mutex<Vec<Message>>>,
}

pub type HistoryMutexGuard<'a> = MutexGuard<'a, Vec<Message>>;
pub type HistoryPoisonError<'a> = PoisonError<HistoryMutexGuard<'a>>;

impl History {
    /// Push a message to the history.
    pub fn push(&self, new_message: &Message) -> Result<(), HistoryPoisonError> {
        let mut messages = self.messages_mut()?;
        Self::push_with_merge(&mut messages, new_message);
        Ok(())
    }

    /// Extend the history with multiple messages.
    pub fn extend(&self, new_messages: &[Message]) -> Result<(), HistoryPoisonError> {
        let mut messages = self.messages_mut()?;
        for message in new_messages {
            Self::push_with_merge(&mut messages, message);
        }
        Ok(())
    }

    fn push_with_merge(messages: &mut Vec<Message>, new_message: &Message) {
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
    #[must_use]
    pub fn messages(&self) -> Result<Vec<Message>, HistoryPoisonError> {
        Ok(self.inner.lock()?.clone())
    }

    /// Get a mutable reference to the inner messages vector.
    pub fn messages_mut(&self) -> LockResult<HistoryMutexGuard> {
        self.inner.lock()
    }

    /// Clone and return the last message in the history, if any.
    #[must_use]
    pub fn last(&self) -> Result<Option<Message>, HistoryPoisonError> {
        let messages = self.inner.lock()?;
        Ok(messages.last().cloned())
    }
}
