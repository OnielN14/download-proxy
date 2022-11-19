use actix_web::{
    get,
    http::header::{self, ContentType, DispositionParam},
    middleware::Logger,
    web, App, HttpResponse, HttpServer, Responder,
};
use clap::{arg, command, Parser};
use downloader::{download as file_downloader, ParseFilename};
use env_logger::Env;
use futures::{future, stream};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionMessage {
    message: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DownloadRequestPayload {
    url: String,
    filename: Option<String>,
}

impl DownloadRequestPayload {
    async fn download(self) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let download_response = file_downloader(&self.url).await?;
        let original_filename = download_response.get_filename();
        let filename = &self.filename.unwrap_or(original_filename);
        let file = download_response.bytes().await?;
        let body = stream::once(future::ok::<_, actix_web::Error>(file));

        Ok(HttpResponse::Ok()
            .append_header(header::ContentDisposition {
                disposition: { header::DispositionType::Attachment },
                parameters: vec![DispositionParam::Filename(filename.to_string())],
            })
            .streaming(body))
    }
}

impl Responder for VersionMessage {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap_or(String::from(""));

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

#[get("/")]
async fn index() -> impl Responder {
    VersionMessage {
        message: String::from("Welcome!"),
        version: String::from(env!("CARGO_PKG_VERSION")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli_args = Args::parse();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let listener = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(
                web::resource("/download")
                    .route(
                        web::get().to(|query: web::Query<DownloadRequestPayload>| async {
                            query.into_inner().download().await
                        }),
                    )
                    .route(web::post().to(
                        |data: web::Either<
                            web::Json<DownloadRequestPayload>,
                            web::Form<DownloadRequestPayload>,
                        >| async { data.into_inner().download().await },
                    )),
            )
    })
    .bind((cli_args.host.to_owned(), cli_args.port))?
    .run();

    info!(
        "🚀 Server listen on {host}:{port}",
        host = cli_args.host,
        port = cli_args.port
    );

    listener.await
}
