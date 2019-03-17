use serde_derive::Deserialize;
use crate::core::db;
use crate::domain::story;
use crate::action::event::*;
use crate::action::routes::Sink;
use crate::action::action::Action;
use super::notify::*;

// types
pub struct AddLine;

#[derive(Deserialize, Debug)]
pub struct NewLine<'a> {
    text:  &'a str,
    name:  Option<&'a str>,
    email: Option<&'a str>
}

// impls
impl<'a> Action<'a> for AddLine {
    type Args = NewLine<'a>;

    fn call(&self, line: NewLine<'a>, sink: Sink) {
        let conn = db::connect();
        let repo = story::Repo::new(&conn);

        // find story
        let mut story = match repo.find_for_today() {
            Ok(s)  => s,
            Err(_) => return sink.send(Event::ShowInternalError)
        };

        // add line to story
        story.add_line(
            line.text,
            line.name,
            line.email
        );

        story.leave(sink.id().into());

        // save updates
        if let Err(_) = repo.save_queue_and_new_line(&mut story) {
            return sink.send(Event::ShowInternalError);
        }

        // send updates to story authors
        notify_authors_with_new_positions(&story, &sink);

        sink.send(Event::ShowThanks);
    }
}
