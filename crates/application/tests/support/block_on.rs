use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Drives a future to completion on the calling thread.
///
/// Keeps the test suite free of any runtime dependency: application futures
/// never park on external I/O, so polling until ready always terminates.
pub fn block_on<FutureToDrive: Future>(future: FutureToDrive) -> FutureToDrive::Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
