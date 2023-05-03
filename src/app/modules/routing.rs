use crate::app::modules::answers::controller::routes as answers_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/answer", answers_routes() )
    })
}
