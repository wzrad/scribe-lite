use core::action;
use core::db::Connected;
use domain::story;
use action::event::*;

// types
pub struct AddLine;

// impls
impl<'a> AddLine {
    pub fn call(&self) -> Event<'a> {
        let repo = story::Repo::connect();

        let result = repo
            .today()
            .map_err(AddLine::errors);

        let mut story = match result {
            Ok(s)  => s,
            Err(e) => return Event::ShowThanks(Err(e))
        };

        story.add_line(
            "This is a real fake line",
            Some("Real Fake"),
            Some("real@fake.com")
        );

        let result = repo.save(&mut story)
            .map_err(AddLine::errors);

        Event::ShowThanks(result)
    }
}

impl AddLine {
    fn errors<'a>(_: diesel::result::Error) -> action::Errors<'a> {
        action::Errors {
            messages: "Errors adding line to story."
        }
    }
}
