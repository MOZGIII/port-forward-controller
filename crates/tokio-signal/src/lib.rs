//! A oneshot value-less signal.

#![allow(missing_docs)]

use std::future::Future;

#[derive(Debug)]
pub struct Sender(tokio::sync::oneshot::Sender<core::convert::Infallible>);

#[derive(Debug)]
pub struct Receiver(tokio::sync::oneshot::Receiver<core::convert::Infallible>);

pub fn channel() -> (Sender, Receiver) {
    let (s, r) = tokio::sync::oneshot::channel();
    (Sender(s), Receiver(r))
}

impl Sender {
    pub fn send(self) {
        drop(self.0);
    }
}

impl Receiver {
    pub fn received(&mut self) -> bool {
        match self.0.try_recv() {
            Ok(_) => unreachable!(),
            Err(tokio::sync::oneshot::error::TryRecvError::Closed) => true,
            Err(tokio::sync::oneshot::error::TryRecvError::Empty) => false,
        }
    }
}

impl Future for Receiver {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let pinned = std::pin::Pin::new(&mut self.0);
        let _ = std::task::ready!(pinned.poll(cx));
        std::task::Poll::Ready(())
    }
}
