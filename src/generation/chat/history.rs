use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};

use crate::generation::chat::message::{Message, Role};

/// This struct uses Arc internally and is safe to be shared/cloned across threads.
#[derive(Debug, Clone, Default)]
pub struct History {
    inner: Arc<Mutex<Vec<Message>>>,
}

pub type HistoryMutexGuard<'a> = MutexGuard<'a, Vec<Message>>;
pub type HistoryPoisonError<'a> = PoisonError<HistoryMutexGuard<'a>>;

impl History {
    #[must_use]
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(messages)),
        }
    }

    /// Delete the history.
    ///
    /// # Errors
    /// If the internal mutex is poisoned.
    pub fn clear(&self) -> Result<(), HistoryPoisonError<'_>> {
        self.messages_mut()?.clear();
        Ok(())
    }

    /// Push a message to the history.
    ///
    /// # Errors
    /// If the internal mutex is poisoned.
    pub fn push(&self, new_message: &Message) -> Result<(), HistoryPoisonError<'_>> {
        let mut messages = self.messages_mut()?;
        Self::push_with_merge(&mut messages, new_message);
        Ok(())
    }

    /// Extend the history with multiple messages.
    ///
    /// # Errors
    /// If the internal mutex is poisoned.
    pub fn extend(&self, new_messages: &[Message]) -> Result<(), HistoryPoisonError<'_>> {
        let mut messages = self.messages_mut()?;
        for message in new_messages {
            Self::push_with_merge(&mut messages, message);
        }
        Ok(())
    }

    fn push_with_merge(messages: &mut Vec<Message>, new_message: &Message) {
        if let Some(last) = messages.last_mut()
            && !last.done
            && last.role == Role::Assistant
            && new_message.role == Role::Assistant
        {
            last.merge_from(new_message);
        } else {
            messages.push(new_message.clone());
        }
    }

    /// Clone and return all messages in the history.
    ///
    /// # Errors
    /// If the internal mutex is poisoned.
    pub fn messages(&self) -> Result<Vec<Message>, HistoryPoisonError<'_>> {
        Ok(self.inner.lock()?.clone())
    }

    /// Get a mutable reference to the inner messages vector.
    ///
    /// # Errors
    /// If the internal mutex is poisoned.
    pub fn messages_mut(&self) -> LockResult<HistoryMutexGuard<'_>> {
        self.inner.lock()
    }

    /// Clone and return the last message in the history, if any.
    ///
    /// # Errors
    /// If the internal mutex is poisoned.
    pub fn last(&self) -> Result<Option<Message>, HistoryPoisonError<'_>> {
        let messages = self.inner.lock()?;
        Ok(messages.last().cloned())
    }
}
