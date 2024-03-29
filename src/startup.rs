use crate::routes::health_check;
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
		.listen(listener)?
		.run();
	Ok(server)
}
