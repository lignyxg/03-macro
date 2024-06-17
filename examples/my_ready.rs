use std::future::{Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

#[macro_export]
macro_rules! my_ready {
    ($x:expr) => {
        match $x {
            task::Poll::Ready(v) => v,
            task::Poll::Pending => task::Poll::Pending,
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let my_fut = MyFut{polled: false, val: 42};
    let ret = my_fut.await;
    println!("{ret}");
    Ok(())
}

struct MyFut {
    polled: bool,
    val: usize,
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.val)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}