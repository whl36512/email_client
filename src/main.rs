extern crate env_logger;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransportBuilder;
use lettre::{ClientSecurity, ClientTlsParameters, EmailTransport};
use lettre_email::EmailBuilder;
use native_tls::Protocol;
use native_tls::TlsConnector;

fn main() {
    env_logger::init();

    let email = EmailBuilder::new()
        .to(("whl36512@gmail.com"))
        .from("do-not-reply@beegrove.com")
        .subject("Example subject 3")
        .text("Example text")
        .build()
        .unwrap();

    pub const DEFAULT_TLS_PROT: &[Protocol] = &[Protocol::Tlsv10];

    let mut tls_builder = TlsConnector::builder().unwrap();
    tls_builder.supported_protocols(DEFAULT_TLS_PROT).unwrap();

    let tls_parameters =
        ClientTlsParameters::new("smtpout.secureserver.net".to_string(), tls_builder.build().unwrap());
        //ClientTlsParameters::new("smtp.gmail.com".to_string(), tls_builder.build().unwrap());

    pub const SUBMISSION_PORT: u16 = 465;

    let mut mailer = SmtpTransportBuilder::new(
        //("smtp.gmail.com", SUBMISSION_PORT),
        ("smtpout.secureserver.net", SUBMISSION_PORT),
        ClientSecurity::Wrapper(tls_parameters),
    ).expect("Failed to create transport")
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            "info@beegrove.com".to_string(),
            "xxxxx".to_string(),
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();

    println!("{:?}", mailer.send(&email));

    mailer.close();
}
