use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

pub trait Message: Send + 'static {}
impl<T: Send + 'static> Message for T {}

pub trait Module: Send + 'static {}
impl<T: Send + 'static> Module for T {}

/// A trait for modules capable of handling messages of type `M`.
#[async_trait::async_trait]
pub trait Handler<M: Message>: Module {
    /// Handles the message. A module must be able to access a `ModuleRef` to itself through `self_ref`.
    async fn handle(&mut self, self_ref: &ModuleRef<Self>, msg: M);
}

#[async_trait::async_trait]
trait Handlee<T: Module>: Message {
    async fn get_handled(self: Box<Self>, module_ref: &ModuleRef<T>, module: &mut T);
}

#[async_trait::async_trait]
impl<M: Message, T: Handler<M>> Handlee<T> for M {
    async fn get_handled(self: Box<Self>, module_ref: &ModuleRef<T>, module: &mut T) {
        module.handle(module_ref, *self).await;
    }
}

/// A handle returned by `ModuleRef::request_tick()`, can be used to stop sending further ticks.
// You can add fields to this struct
pub struct TimerHandle {
    ticker: tokio::task::JoinHandle<()>,
}

impl TimerHandle {
    /// Stops the sending of ticks resulting from the corresponding call to `ModuleRef::request_tick()`.
    /// If the ticks are already stopped, does nothing.
    pub async fn stop(&self) {
        self.ticker.abort();
    }
}

// You can add fields to this struct.
pub struct System {
    stop_sender: broadcast::Sender<bool>,
    tasks: Vec<JoinHandle<JoinHandle<()>>>,
}

impl System {
    /// Registers the module in the system.
    /// Returns a `ModuleRef`, which can be used then to send messages to the module.
    pub async fn register_module<T: Module>(&mut self, mut module: T) -> ModuleRef<T> {
        let (sender, mut receiver) = unbounded_channel::<Box<dyn Handlee<T>>>();
        let mut stop_recv = self.stop_sender.subscribe();
        let stop_recv2 = self.stop_sender.subscribe();

        let module_ref = ModuleRef::<T>{msg_sender: sender, stop_receiver: stop_recv2};
        let module_clone = module_ref.clone();

        self.tasks.push(tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    _stop = stop_recv.recv() => {
                        break;
                    }
                    msg = receiver.recv() => {
                        msg.unwrap().get_handled(&module_ref, &mut module).await;
                    }
                }
            }
            // To sync dropping receiver
            tokio::spawn(async move {
                let _recv = receiver;
                _ = stop_recv.recv().await.unwrap();
            })
        }));
        module_clone
    }

    /// Creates and starts a new instance of the system.
    pub async fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        System{stop_sender: tx, tasks: vec![]}
    }

    /// Gracefully shuts the system down.
    pub async fn shutdown(&mut self) {
        self.stop_sender.send(false).unwrap();
        let mut all_receiver: Vec<JoinHandle<()>> = vec![];
        for task in self.tasks.drain(..) {
            all_receiver.push(task.await.unwrap());
        }

        // Sync ending (dropping receivers)
        self.stop_sender.send(false).unwrap();
        for recv in all_receiver.drain(..) {
            recv.await.unwrap();
        }
    }
}

/// A reference to a module used for sending messages.
// You can add fields to this struct.
pub struct ModuleRef<T: Module + ?Sized> {
    msg_sender: UnboundedSender<Box<dyn Handlee<T>>>,
    stop_receiver: broadcast::Receiver<bool>,
}

impl<T: Module> ModuleRef<T> {
    /// Sends the message to the module.
    pub async fn send<M: Message>(&self, msg: M)
    where
        T: Handler<M>,
    {
        self.msg_sender.send(Box::new(msg)).unwrap();
    }

    /// Schedules a message to be sent to the module periodically with the given interval.
    /// The first tick is sent after the interval elapses.
    /// Every call to this function results in sending new ticks and does not cancel
    /// ticks resulting from previous calls.
    pub async fn request_tick<M>(&self, message: M, delay: Duration) -> TimerHandle
    where
        M: Message + Clone,
        T: Handler<M>,
    {
        let mut stop_receiver_clone = self.stop_receiver.resubscribe();
        let msg_sender_clone = self.msg_sender.clone();
        TimerHandle {
            ticker: tokio::spawn(async move {
                let boxed_msg = Box::new(message);
                let mut interval = tokio::time::interval(delay);
                interval.tick().await;
                loop {
                    tokio::select! {
                        biased;
                        _ = stop_receiver_clone.recv() => {
                            break;
                        }
                        _ = interval.tick() => {
                            msg_sender_clone.send(boxed_msg.clone()).unwrap();
                        }
                    }
                }
            })
        }
    }
}

impl<T: Module> Clone for ModuleRef<T> {
    /// Creates a new reference to the same module.
    fn clone(&self) -> Self {
        ModuleRef{msg_sender: self.msg_sender.clone(), stop_receiver: self.stop_receiver.resubscribe()}
    }
}
