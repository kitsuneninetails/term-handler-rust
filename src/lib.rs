#[macro_use] extern crate lazy_static;
extern crate signal_simple;

use signal_simple::signal::{signal as sig, SIGTERM, SIGINT, int};
use signal_simple::channel::SyncChannel;

lazy_static! {
    static ref TERM_HANDLER_CHANNEL: SyncChannel<()> = SyncChannel::new();
}

fn term_handler(_: int) {
    TERM_HANDLER_CHANNEL.send(()).unwrap();
}

pub fn wait_for_term() {
    sig(SIGTERM, term_handler, None);
    sig(SIGINT, term_handler, None);

    let _ = TERM_HANDLER_CHANNEL.recv();
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_simple::signal::kill;

    #[test]
    fn test_term() {
        ::std::thread::spawn(|| {
            ::std::thread::sleep(::std::time::Duration::from_secs(2));
            kill(SIGTERM);
        });
        wait_for_term();
    }
}
