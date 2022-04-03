use std::{pin::Pin, task::{Context, Poll}};
use futures::{Future, FutureExt, executor::block_on};

enum InnerHandle {
    Tokio(tokio::task::JoinHandle<()>),
    Futures{
        abort_handle : futures::future::AbortHandle,
        task_handle : Option<futures::future::RemoteHandle<()>>
    }
}

impl InnerHandle {
    fn abort(&self) {
        match self{
            InnerHandle::Tokio(handle) => handle.abort(),
            InnerHandle::Futures { abort_handle, .. } => abort_handle.abort(),
        }
    }
}

impl Drop for InnerHandle {
    fn drop(&mut self) {
        // Drop futures handle will drop the task in executor
        // we need call forget(self) to continue the task 
        // Option::take() : &mut self -> self 
        match self {
            InnerHandle::Futures { task_handle, .. } => {
                // never fails
                let task_handle = task_handle.take().unwrap();
                task_handle.forget()
            },
            _ => ()
        }
    }
}

pub struct Handle {
    inner : InnerHandle
}

impl Handle {
    pub(in crate) fn tokio(handle : tokio::task::JoinHandle<()>) -> Self {
        Handle {
            inner : InnerHandle::Tokio(handle)
        }
    }

    pub(in crate) fn futures(abort_handle: futures::future::AbortHandle,task_handle: futures::future::RemoteHandle<()>) -> Self {
        Handle {
            inner : InnerHandle::Futures{
                abort_handle,
                task_handle : Some(task_handle)
            }
        }
    }

    pub fn abort(&self) {
        self.inner.abort();
    }
}

impl Future for Handle {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut self.inner {
            InnerHandle::Tokio(handle) => handle.poll_unpin(cx).map(|_|()),
            InnerHandle::Futures { task_handle, .. } => 
                task_handle.as_mut().unwrap().poll_unpin(cx)
        }
    }
}

pub trait Spawner {
    fn spawn<F>(&mut self,f : F) -> Handle
    where F : Future<Output = ()> + Send + 'static;

    fn spawn_local<F>(&mut self,f : F) -> Handle
    where F : Future<Output = ()> + 'static;

    fn block_on<F>(&self,f : F) -> F::Output
    where F : Future {
        block_on(f)
    }
}
