use std::error::Error;
use actix_web::HttpResponse;
use log::error;
use crate::domain;

pub(crate) trait NotFoundResponder {
    fn unwrap_or_404(&self) -> HttpResponse;
}

impl NotFoundResponder for Result<Option<domain::todo::Todo>, Box<dyn Error>> {
    fn unwrap_or_404(&self)-> HttpResponse {
        match self {
            Ok(Some(todo)) => HttpResponse::Ok().json(todo),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            },
        }
    }
}
