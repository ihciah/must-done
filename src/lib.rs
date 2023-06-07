use std::{future::Future, task::Poll};

pin_project_lite::pin_project! {
    pub struct MustDoneFuture<F> {
        #[pin]
        inner: F,
        guard: Option<Guard>,
    }
}

pub trait MustDone {
    fn must_done(self) -> MustDoneFuture<Self>
    where
        Self: Sized;
}

impl<F: Future> Future for MustDoneFuture<F> {
    type Output = F::Output;

    #[inline]
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.inner.poll(cx) {
            Poll::Ready(r) => {
                std::mem::forget(this.guard.take());
                Poll::Ready(r)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<F> MustDoneFuture<F> {
    #[inline]
    pub fn new(inner: F) -> Self {
        Self {
            inner,
            guard: Some(Guard),
        }
    }
}

impl<F> From<F> for MustDoneFuture<F> {
    #[inline]
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl<T: Future + Sized> MustDone for T {
    #[inline]
    fn must_done(self) -> MustDoneFuture<Self>
    where
        Self: Sized,
    {
        MustDoneFuture::new(self)
    }
}

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        extern "C" {
            #[link_name = "oops"]
            fn trigger() -> !;
        }

        unsafe { trigger() };
    }
}
