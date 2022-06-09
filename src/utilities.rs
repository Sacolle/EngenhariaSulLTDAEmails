/*pub mod models;
pub mod schema;*/
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport,
	message::{header,MultiPart,SinglePart}
};



pub fn send_email(destino:&str, html:String) -> Result<(), Box<dyn std::error::Error>>{
	let receiver = format!("pessoa <{}>",destino);

	let email = Message::builder()
		.from("ME <joaosilva11235813@gmail.com>".parse()?)
		.to(receiver.parse()?)
		.subject("Testando o envio + html")
		.multipart(
			MultiPart::alternative() // This is composed of two parts.
			.singlepart(
				SinglePart::builder()
					.header(header::ContentType::TEXT_PLAIN)
					.body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
			)
			.singlepart(
				SinglePart::builder()
					.header(header::ContentType::TEXT_HTML)
					.body(html),
			),
		)?;
	
	let creds = Credentials::new("joaosilva11235813@gmail.com".to_string(), crate::config_info::get_email_creds());

	// Open a remote connection to gmail
	let mailer = SmtpTransport::relay("smtp.gmail.com")?
		.credentials(creds)
		.build();

	// Send the email
	//retorna o enum result da função para main
	match mailer.send(&email){
		Ok(_) => Ok(()),
		Err(e) => Err(Box::new(e))
	}
}

