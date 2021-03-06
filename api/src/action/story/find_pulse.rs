use crate::action::action::Action;
use crate::action::event::{Outbound, Scheduled};
use crate::action::routes::Sink;

// -- types --
#[derive(Debug)]
pub struct FindPulse;

// -- impls --
impl Action for FindPulse {
    type Args = ();

    fn new(_: ()) -> Self {
        FindPulse
    }

    fn call(self, sink: Sink) {
        // send message to client
        sink.send(Outbound::FindPulse);

        // schedule evaluation in 30s
        sink.schedule(Scheduled::TestPulse, 30 * 1000);
    }
}
