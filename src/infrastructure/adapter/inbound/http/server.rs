use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer, web};
use anyhow::Error;
use tracing_actix_web::TracingLogger;

use crate::application::port::inbound::TokenSupplyService;
use crate::infrastructure::configuration::server::ServerConfig;

use super::route::{circulating_supply, health, total_supply};

/// Main application structure responsible for server initialization.
pub struct HttpApplication {
    /// The port the server is listening on.
    port: u16,

    /// The underlying Actix Web server instance.
    server: Server,
}

impl HttpApplication {
    /// Builds a new application instance with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    /// * `data` - Application data.
    ///
    /// # Returns
    ///
    /// `Result<Self, Error>` - The built application or an error.
    pub async fn build<T>(config: ServerConfig, data: Data<T>) -> Result<Self, Error>
    where
        T: TokenSupplyService + Send + Sync + 'static,
    {
        let address = format!("{}:{}", config.host, config.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .wrap(TracingLogger::default())
                .service(
                    web::scope("/v1")
                        .route(
                            "/circulating",
                            web::get().to::<_, (Data<T>,)>(circulating_supply),
                        )
                        .route("/total", web::get().to::<_, (Data<T>,)>(total_supply)),
                )
                .route("/healthz", web::get().to(health))
        })
        .listen(listener)?
        .run();

        Ok(Self { port, server })
    }

    /// Returns the port the server is listening on.
    ///
    /// This is useful when the server is configured to use a dynamic port.
    ///
    /// # Returns
    ///
    /// The port number.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Runs the application until it is stopped.
    ///
    /// This method starts the HTTP server and blocks until the server shuts
    /// down, either because of an error or an explicit shutdown request.
    ///
    /// # Returns
    ///
    /// `Result<(), std::io::Error>` - Success or an IO error.
    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
