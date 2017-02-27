use std::boxed::FnBox;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;
use futures::{Future, Poll, Async};

///A command which interfaces with the hardware on the hardware thread.
///
///Commands may or may not pass back data to the calling thread.
pub trait Command {
    ///The optional return type of the command.
    type Output: Send;

    ///Calls the command on the hardware thread.
    fn execute(self, hw: &mut HardwareContext) -> Result<Self::Output, HardwareError>;
}

///An error resulting from interaction with the hardware.
#[derive(Clone, Debug)]
pub enum HardwareError {
    Dummy
}

struct CommandReturn<C: Command> {
    data: Arc<UnsafeCell<Option<Result<C::Output, HardwareError>>>>,
    is_finished: Arc<AtomicBool>
}

///The future used on the calling thread to get the return value from a command.
///
///A `CommandReturn` on the hardware thread will have an `UnsafeCell` for `data`, but data races
///are prevented because once `is_finished` is set to `true`, ithe hardware thread is not allowed
///to write a second time. `CommandFuture` will not attempt to read from `data` until `is_finished`
///is set.
pub struct CommandFuture<C: Command> {
    data: Arc<UnsafeCell<Option<Result<C::Output, HardwareError>>>>,
    is_finished: Arc<AtomicBool>
}

///The state information stored on the hardware thread.
///
///This struct also serves to ensure that hardware access occurs only on the hardware thread.
///All of the methods to interact with the hardware are only accessible with a reference
///to an instance of this object. To perform hardware actions on another thread, use commands.
//right now there is no state
pub struct HardwareContext;

///The object used for communicating with the HardwareContext from another thread.
#[derive(Clone)]
pub struct CommandSender {
    send: Sender<Box<FnBox(&mut HardwareContext) + Send>>
}

impl<C: Command> CommandFuture<C> {
    ///Initializes a new future.
    ///
    ///This will return both the future as well as the `CommandReturn` which should
    ///be passed to the hardware thread.
    fn new() -> (CommandFuture<C>, CommandReturn<C>) {
        let mut data = Arc::new(UnsafeCell::new(None));
        let is_finished = Arc::new(AtomicBool::new(false));
        let ret = CommandReturn {
            data: data.clone(),
            is_finished: is_finished.clone()
        };
        let future = CommandFuture {
            data: data,
            is_finished: is_finished
        };
        (future, ret)
    }
}

impl<C: Command> Future for CommandFuture<C> where C::Output: Clone {
    type Item = C::Output;

    type Error = HardwareError; //TODO: real error handling!

    //TODO: it might be better if it was returning a reference rather than cloning the output
    //I don't know of a way to do that though because Self::Item needs to have a lifetime then
    fn poll(&mut self) -> Poll<C::Output, HardwareError> {
        if self.is_finished.load(Ordering::Relaxed) {
            unsafe {
                (*self.data.get()).clone().unwrap().map(Async::Ready)
            }
        }
        else {
            Ok(Async::NotReady)
        }
    }

    fn wait(self) -> Result<C::Output, HardwareError> {
        while !self.is_finished.load(Ordering::Relaxed) {
            thread::sleep_ms(5);
        }
        unsafe {
            (*self.data.get()).clone().unwrap()
        }
    }
}

//This is needed because `UnsafeCell` is not Send by default.
//It is Ok for the reasons described above.
unsafe impl<C: Command> Send for CommandReturn<C> { }

impl<C: Command> CommandReturn<C> {
    ///Sets the return value for the command.
    ///
    ///This consumes the `CommandReturn` so that the return value cannot be set multiple times.
    fn set(mut self, value: Result<C::Output, HardwareError>) {
        unsafe {
            *self.data.get() = Some(value);
        }
        //set the flag that this command is complete
        self.is_finished.store(true, Ordering::Relaxed);
    }
}

impl CommandSender {
    ///Sends a command to the hardware thread and returns a future for the return value.
    fn run<C: Command + Send + 'static>(&self, command: C) -> CommandFuture<C> {
        let (future, ret) = CommandFuture::new();
        self.send.send(Box::new(move |hardware: &mut HardwareContext| {
            ret.set(command.execute(hardware));
        }));
        future
    }
}

///Spawns the hardware thread.
///
///This will return a `CommandSender` for communicating with the hardware thread, which can be
///cloned any number of times.
fn spawn_hardware_thread() -> CommandSender {
    let (tx_command, rx_command) = channel::<Box<FnBox(&mut HardwareContext) + Send>>();
    thread::spawn(move || {
        let mut hardware = HardwareContext;

        for cmd in rx_command.iter() {
            cmd.call_box((&mut hardware,));
        }
    });
    CommandSender {
        send: tx_command
    }
}

#[cfg(test)]
mod test {
    use hal::commands::*;

    struct DummyCommand;

    impl Command for DummyCommand {
        type Output = u32;

        fn execute(self, hw: &mut HardwareContext) -> Result<u32, HardwareError> {
            Ok(42)
        }
    }
    
    struct DummyErrorCommand;
    
    impl Command for DummyErrorCommand {
        type Output = u32;

        fn execute(self, hw: &mut HardwareContext) -> Result<u32, HardwareError> {
            Err(HardwareError::Dummy)
        }
    }

    #[test]
    fn async_commands() {
        let sender = spawn_hardware_thread();
        let future = sender.run(DummyCommand);
        assert_eq!(future.wait().unwrap(), 42);
    }
    
    #[test]
    fn async_commands_errors() {
        let sender = spawn_hardware_thread();
        let future = sender.run(DummyErrorCommand);
        assert!(future.wait().is_err());
    }
}

