/*pub mod models;relay
pub mod schema;*/
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport,
	message::{header,MultiPart,SinglePart}
};

use lettre::message::Mailbox;
use crate::models::Email;
use crate::config_info::EmailSender;
use crate::error::{MissignFieldError,TableProcessError};


pub fn send_email(sender:&EmailSender,destinos:&Vec<Email>, html:String) -> Result<(),TableProcessError>{

	let mut email = Message::builder()
		.from(format!("{} <{}>",&sender.nome,&sender.email).parse()?)
		.subject("Testando o envio + html");

	//Ao definir os tipos de errors, mudar a função para ok_or
	for mbox in destinos{
		email = email.to(Mailbox::new(
			mbox.email_name.clone(),
			mbox.email_adrs.as_ref()
				.ok_or(MissignFieldError::new("email_adrs"))?
				.parse()?)
			);
	}
	//println!("The value of the email is:\n{:?}",email);
	let email = email.multipart(
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

	let creds = Credentials::new(sender.email.clone(),sender.senha.clone());

	// Open a remote connection to gmail
	let mailer = SmtpTransport::starttls_relay("smtp.office365.com")?
		.credentials(creds)
		.build();

	// Send the email
	mailer.send(&email)?;
	Ok(())
}


#[cfg(test)]
mod tests{
	use super::*;
	//#[test]
	#[allow(dead_code)]
	fn testing_email_send_func(){
		let destinos = vec![
			Email{
				id: 0,
				empresa: None,
				email_adrs: Some(String::from("pedro.h.b.colle@gmail.com")),
				email_name: Some(String::from("Pedro")),
				env_relig: None,
				env_lockout: None,
				env_normaliz:None
			},
			Email{
				id: 0,
				empresa: None,
				email_adrs: Some(String::from("joaosilva11235813@gmail.com")),
				email_name: Some(String::from("Joao")),
				env_relig: None,
				env_lockout: None,
				env_normaliz:None
			}
		];
		send_email(&EmailSender::get("GMAIL_CREDS").unwrap(),&destinos, String::from("test")).unwrap();
	}
}