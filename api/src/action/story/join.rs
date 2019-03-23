use crate::core::db;
use crate::domain::story;
use crate::action::event::Outbound;
use crate::action::routes::Sink;
use crate::action::action::Action;
use super::notify::notify_new_author;

// types
#[derive(Debug)]
pub struct Join;

// impls
impl Action for Join {
    type Args = ();

    fn new(_: ()) -> Self {
        Join
    }

    fn call(self, sink: Sink) {
        let conn = db::connect();
        let repo = story::Repo::new(&conn);

        // find story
        let mut story = match repo.find_or_create_for_today() {
            Ok(s)  => s,
            Err(_) => return sink.send(Outbound::ShowInternalError)
        };

        // join story
        story.join(sink.id().into());

        // save updates
        if let Err(_) = repo.save_queue(&mut story) {
            return sink.send(Outbound::ShowInternalError);
        }

        // notify author
        notify_new_author(&story, &sink);
    }
}
