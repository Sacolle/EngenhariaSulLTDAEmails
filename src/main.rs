mod utilities;
mod config_info;
mod message_builder;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

use models::{Ocor,OcorSoe, Email};
use config_info::DbConfig;
use diesel::prelude::*;
use message_builder::build_message;
use utilities::send_email;
use schema::ocorrencia::dsl as ocortb;
use schema::ocorrencia_soe::dsl as soetb;
use schema::cadastroemails::dsl as emails;


fn main(){
	let db = DbConfig::init("CHAVES_DB_LOCAL").expect("Falha no .ini");
	
	for (url,table) in db.make_table_urls(){
		if let Err(e) = process_table(&url,&table){
			//TODO: log the errors
			println!("{:?}",e);
		}else{
			println!("Tabela no server: {}\nCom link:{}\nAcessada com sucesso",&url,&table);
		}
	}
}


fn process_table(url:&str,table:&str)->Result<(),Box<dyn std::error::Error>>{
	let connec = MysqlConnection::establish(&format!("{}{}",url,table))?;

	let results = ocortb::ocorrencia
		.filter(ocortb::EmailSended.eq("f"))
		.limit(10)
		.load::<Ocor>(&connec)?;

	//TODO: acessar a db de emails para dar fetch no email
	//let destinatario = parse_email(); 

	let empresa_emails = table.split('_').nth(1).unwrap();

	let destinos = emails::cadastroemails
		.filter(emails::Empresa.eq(empresa_emails))
		.load::<Email>(&connec)?;
	
	for instance in results{
		let inst_id = instance.id;

		let ocor_soe = soetb::ocorrencia_soe
			.filter(soetb::OcoID.eq(inst_id))
			.load::<OcorSoe>(&connec)?;

		send_email(&destinos, build_message(instance,ocor_soe)?)?;

		diesel::update(ocortb::ocorrencia.find(inst_id))
			.set(ocortb::EmailSended.eq("t")).execute(&connec)?;
	}
	Ok(())
}

#[cfg(test)]
mod tests{
	use super::*;
	use std::error::Error;
	#[test]
	fn testing_connect()->Result<(),Box<dyn Error>>{
		let db = DbConfig::init("CHAVES_DB_LOCAL")?;

		for (url,table) in db.make_table_urls(){
			MysqlConnection::establish(&format!("{}{}",url,table))?;
		}
		Ok(())
	}
	#[test]
	fn is_query_data_working()->Result<(),Box<dyn Error>>{
		let db = DbConfig::init("CHAVES_DB_LOCAL")?;

		let (url,table) = db.make_table_urls().next().unwrap();
		let connec = MysqlConnection::establish(&format!("{}{}",url,table))?;

		let results = ocortb::ocorrencia
			.filter(ocortb::EmailSended.eq("f"))
			.load::<Ocor>(&connec)?;

		assert!(!results.is_empty());
		Ok(())
	}
	#[test]
	fn is_query_emails_working()->Result<(),Box<dyn Error>>{
		let db = DbConfig::init("CHAVES_DB_LOCAL")?;

		let (url,table) = db.make_table_urls().next().unwrap();
		let connec = MysqlConnection::establish(&format!("{}{}",url,table))?;

		let empresa_emails = table.split('_').nth(1).unwrap();
		println!("pegando emails da empresa: {}",empresa_emails);

		let destinos = emails::cadastroemails
			.filter(emails::Empresa.eq(empresa_emails))
			.load::<Email>(&connec)?;

		assert!(!destinos.is_empty());
		println!("id do primeiro valor: {:?}",destinos[0].email_adrs.clone().unwrap());
		Ok(())
	}

	#[test]
	fn build_table_from_query()->Result<(),Box<dyn std::error::Error>>{
		use std::{fs,io::Write};

		let db = DbConfig::init("CHAVES_DB_LOCAL")?;

		let (url,table) = db.make_table_urls().next().unwrap();
		let connec = MysqlConnection::establish(&format!("{}{}",url,table))?;

		let results = ocortb::ocorrencia
			.filter(ocortb::EmailSended.eq("f"))
			.limit(1)
			.load::<Ocor>(&connec)?;

		for inst in results{
			let ocor_soe = soetb::ocorrencia_soe
				.filter(soetb::OcoID.eq(inst.id))
				.load::<OcorSoe>(&connec)?;
			
			let mut f = fs::File::create("./testres/tabela_from_query.html")?;

			let html = build_message(inst, ocor_soe)?;
			assert!(f.write_all(html.as_bytes()).is_ok());
		}
		Ok(())
	}
	//#[test]
	#[allow(dead_code)]
	fn is_send_email_test_base()->Result<(),Box<dyn std::error::Error>>{
		use lettre::transport::smtp::authentication::Credentials;
		use lettre::{Message, SmtpTransport, Transport };
		
		let (user,senha,_relay) = crate::config_info::get_email_sender();

		let email = Message::builder()
			.from(format!("Engenharia <{}>",&user).parse()?)
			.to("Zampierri <vzampieri@gmail.com>".parse()?)
			.subject("Email teste sem HTML")
			.body(String::from("Aí estas meu cacíque"))?;

		let creds = Credentials::new(user,senha);

		// Open a remote connection to gmail
		let mailer = SmtpTransport::starttls_relay("smtp.office365.com")?
			.credentials(creds)
			.build();


		// Send the email
		match mailer.send(&email) {
			Ok(_) => println!("Email sent successfully!"),
			Err(e) => panic!("Could not send email: {:?}", e),
		}	
		Ok(())
	}
}