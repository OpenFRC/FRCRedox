use std::sync::mpsc::{channel, Sender};
use std::thread;
use futures::{Future, Poll, Async};
use futures::sync::oneshot;

/// A command which interfaces with the hardware on the hardware thread.
///
/// Commands may or may not pass back data to the calling thread.
pub trait Command {
    /// The optional return type of the command.
    type Output: Send;

    /// Calls the command on the hardware thread.
    fn execute(self, hw: &mut HardwareContext) -> Result<Self::Output>;
}

// This basically simulates a Box<FnOnce(&mut HardwareContext)>
// Unfortunately, Box<FnOnce(&mut HardwareContext)>, doesn't work yet and BoxFn is unstable
struct CommandWithReturn {
    func: Box<FnMut(&mut HardwareContext) + Send>
}

error_chain! {
    errors {
        HardwareThreadCrashed {
            description("the hardware thread has crashed")
            display("the hardware thread has crashed")
        }
        Unknown {
            description("unknown error")
            display("unknown error")
        }
    }
}

/// The future used on the calling thread to get the return value from a command.
///
/// This is really just a wrapper around `futures::sync::oneshot::Receiver` which converts the
/// error into something more meaningful.
pub struct CommandFuture<C: Command>(oneshot::Receiver<Result<C::Output>>);

/// The state information stored on the hardware thread.
///
/// This struct also serves to ensure that hardware access occurs only on the hardware thread.
/// All of the methods to interact with the hardware are only accessible with a reference
/// to an instance of this object. To perform hardware actions on another thread, use commands.
// right now there is no state
pub struct HardwareContext;

/// The object used for communicating with the `HardwareContext` from another thread.
#[derive(Clone)]
pub struct CommandSender {
    send: Sender<CommandWithReturn>
}

impl CommandWithReturn {
    fn new<F: FnOnce(&mut HardwareContext) + Send + 'static>(func: F) -> CommandWithReturn {
        // ugly hack borrowed from
        // https:// github.com/stbuehler/rust-boxfnonce/blob/master/src/macros.rs
        let mut func = Some(func);
        CommandWithReturn {
            func: Box::new(move |hw: &mut HardwareContext| {
                func.take().unwrap()(hw)
            })
        }
    }

    fn execute(mut self, hw: &mut HardwareContext) {
        (*self.func)(hw)
    }
}

impl<C: Command> Future for CommandFuture<C> {
    type Item = C::Output;

    type Error = Error;

    fn poll(&mut self) -> Poll<C::Output, Error> {
        match self.0.poll() {
            Ok(Async::Ready(Ok(result))) => Ok(Async::Ready(result)),
            Ok(Async::Ready(Err(error))) => Err(error),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(_) => Err(ErrorKind::HardwareThreadCrashed.into())
        }
    }

    fn wait(self) -> Result<C::Output> {
        match self.0.wait() {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(error)) => Err(error),
            Err(_) => Err(ErrorKind::HardwareThreadCrashed.into())
        }
    }
}

impl CommandSender {
    /// Sends a command to the hardware thread and returns a future for the return value.
    pub fn run<C: Command + Send + 'static>(&self, command: C) -> Result<CommandFuture<C>> {
        let (sender, future) = oneshot::channel();
        match self.send.send(CommandWithReturn::new(move |hardware: &mut HardwareContext| {
            sender.complete(command.execute(hardware));
        })) {
            Ok(_) => Ok(CommandFuture(future)),
            Err(_) => Err(ErrorKind::HardwareThreadCrashed.into())
        }
    }
}

/// Spawns the hardware thread.
///
/// This will return a `CommandSender` for communicating with the hardware thread, which can be
/// cloned any number of times.
pub fn spawn_hardware_thread() -> CommandSender {
    let (tx_command, rx_command) = channel::<CommandWithReturn>();
    thread::spawn(move || {
        let mut hardware = HardwareContext;

        for cmd in rx_command.iter() {
            cmd.execute(&mut hardware);
        }
    });
    CommandSender {
        send: tx_command
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use hal::commands::*;

    struct DummyCommand;

    impl Command for DummyCommand {
        type Output = u32;

        fn execute(self, _: &mut HardwareContext) -> Result<u32> {
            Ok(42)
        }
    }

    struct DummySlowCommand;
    impl Command for DummySlowCommand {
        type Output = u32;

        fn execute(self, _: &mut HardwareContext) -> Result<u32> {
            thread::sleep(Duration::from_millis(500));
            Ok(43)
        }
    }
    
    struct DummyErrorCommand;
    
    impl Command for DummyErrorCommand {
        type Output = u32;

        fn execute(self, _: &mut HardwareContext) -> Result<u32> {
            Err(ErrorKind::Unknown.into())
        }
    }

    #[test]
    fn async_commands() {
        let sender = spawn_hardware_thread();
        let future = sender.run(DummyCommand).unwrap();
        assert_eq!(future.wait().unwrap(), 42);
    }
    
    #[test]
    fn async_commands_errors() {
        let sender = spawn_hardware_thread();
        let future = sender.run(DummyErrorCommand).unwrap();
        assert!(future.wait().is_err());
    }

    #[test]
    fn multiple_async_commands() {
        let sender = spawn_hardware_thread();
        let future_slow = sender.run(DummySlowCommand).unwrap();
        let future = sender.run(DummyCommand).unwrap();
        assert_eq!(future.wait().unwrap(), 42);
        assert_eq!(future_slow.wait().unwrap(), 43);
    }
}

