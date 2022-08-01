/*pub mod models;
pub mod schema;*/
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport,
	message::{header,MultiPart,SinglePart}
};

use lettre::message::Mailbox;
use crate::models::Email;


pub fn send_email(destinos:&Vec<Email>, html:String) -> Result<(), Box<dyn std::error::Error>>{
	let (user,senha,relay) = crate::config_info::get_email_sender();

	let email = Message::builder()
		.from(format!("Engenharia <{}>",user).parse()?)
		.subject("Testando o envio + html");

	let email = destinos.iter()
		.fold(email, |acc,mail|{
			println!("{}",mail.email_adrs.clone().unwrap());
			match &mail.email_adrs{
				Some(val)=>acc.to(
						Mailbox::new(
							mail.email_name.clone(),
							val.parse().unwrap())
						),
				None => acc
			}
		});
	println!("The value of the email is:\n{:?}",email);

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
	

	let creds = Credentials::new(user,senha);

	// Open a remote connection to gmail
	let mailer = SmtpTransport::starttls_relay(&relay)?
		.credentials(creds)
		.build();

	// Send the email
	//retorna o enum result da função para main

	match mailer.send(&email){
		Ok(_) => Ok(()),
		Err(e) => Err(Box::new(e))
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use std::error::Error;
	//#[test]
	#[allow(dead_code)]
	fn testing_email_send_func()->Result<(),Box<dyn Error>>{
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
		send_email(&destinos, String::from("test"))?;
		Ok(())
	}
}