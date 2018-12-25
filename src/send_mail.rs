extern crate env_logger;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;

use self::lettre::smtp::authentication::{Credentials, Mechanism};
use self::lettre::smtp::ConnectionReuseParameters;
use self::lettre::smtp::SmtpTransportBuilder;
use self::lettre::{ClientSecurity, ClientTlsParameters, EmailTransport};
use self::lettre_email::EmailBuilder;
use self::native_tls::Protocol;
use self::native_tls::TlsConnector;


struct Mail {
		to			: String
	,	from			: String
	,	subject		: String
	,	text		: String
}

impl Mail {
fn send(&self ) -> Result<i32, Box<Error>> {

    let email = EmailBuilder::new()
        .to((self.to))
        .from(self.from)
        .subject(self.subject)
        .text(self.subject)
        .build()
        .unwrap();

    pub const DEFAULT_TLS_PROT: &[Protocol] = &[Protocol::Tlsv10];
    pub const SUBMISSION_PORT: u16 = 465;

    let mut tls_builder = TlsConnector::builder().unwrap();
    tls_builder.supported_protocols(DEFAULT_TLS_PROT).unwrap();



	//let username = &env::var("GMAIL_USERNAME").unwrap_or("user".to_string())[..] ;
	let username = &env::var("GMAIL_USERNAME").expect("ERROR: GMAIL_USERNAME is not set") ;
	let password = &env::var("GMAIL_PASSWORD").expect("ERROR: GMAIL_PASSWORD is not set")
	let smtp_server = &env::var("SMTP_SERVER").expect("ERROR: SMTP_SERVER is not set") ;

    let tls_parameters = ClientTlsParameters::new(smtp_server.to_string(), tls_builder.build().unwrap());

    let mut mailer = SmtpTransportBuilder::new(
        (smtp_server, SUBMISSION_PORT),
        ClientSecurity::Wrapper(tls_parameters),
    )	.expect("Failed to create transport to SMTP_SERVER")
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            	username.to_string()
            ,	password.to_string()
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();
	let result= mailer.send(&email) ;

    info!("{:?}", result);
    mailer.close();
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    #[test]
    fn test() {
        let _ = env_logger::try_init();

		let m = Mail { 		from	:	"abc@beegrove.com"
						,	to		:	"whl36512@gmail.com"
						,	subject	:	"subject line"
						,	text	:	"text content"
					}

		m.send();
		
        //assert_eq!(3, add_one(2));
    }

}
