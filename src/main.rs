/////////////////////////////////////////////////////////
////WEB, EMAIL,
/////////////////////////////////////////////////////////
use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;
use env_logger;
use log::{info, error};

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
    message: String,
}

async fn send_email(form: web::Form<FormData>) -> impl Responder {
    // SMTP-сервер и аутентификационные данные
    let smtp_server = "smtp.mailersend.net";
    let username = "MS_7iPxWd@trial-z86org86n8n4ew13.mlsender.net";
    let password = "YPClaEwcOgENBlDu";
    let from = "MS_7iPxWd@trial-z86org86n8n4ew13.mlsender.net";
    let to = "mazafakajke@gmail.com";

    // Формируем тело письма из данных формы
    let email_body = format!(
        "Name: {}\nEmail: {}\nMessage:\n{}",
        form.name, form.email, form.message
    );

    // Создаем сообщение
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(format!("Message from {}", form.name))
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)
        .unwrap();

    // Устанавливаем учетные данные
    let creds = Credentials::new(username.to_owned(), password.to_owned());

    // Создаем SMTP-транспорт с поддержкой TLS
    let mailer = SmtpTransport::starttls_relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Отправляем email
    let result = match mailer.send(&email) {
        Ok(_) => "Email sent successfully.".to_string(),
        Err(e) => format!("Could not send email: {:?}", e),
    };

    result
}

async fn web_server() -> std::io::Result<()> {
    info!("Starting web server on http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .route("/send-email", web::post().to(send_email))
            .service(
                fs::Files::new("/", "/web_server")
                    .index_file("index.html"),
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
#[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     web_server().await
// }
async fn main() -> std::io::Result<()> {
    // Инициализация логгера
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    web_server().await
}
