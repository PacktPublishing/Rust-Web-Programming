use std::io::Cursor;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};



pub struct ReRouter;

impl Fairing for ReRouter {

    fn info(&self) -> Info {
        Info {
            name: "GET rerouter",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {

        if request.method() == Method::Get && response.status() == Status::NotFound {
            let body = format!("URL does not exist");
            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(body));
        }
        return
    }
}
