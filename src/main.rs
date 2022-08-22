mod utilities;
mod config_info;
mod message_builder;
mod error;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

use models::{Ocor,OcorSoe, Email};
use config_info::{EmailSender};
use diesel::prelude::*;
use message_builder::build_message;
use utilities::send_email;
use error::{TableProcessError};
use schema::Ocorrencia::dsl as ocortb;
use schema::Ocorrencia_SOE::dsl as soetb;
use schema::CadastroEmails::dsl as emails;

//temp para evitar envio de email em prod
use std::env;
use std::{fs,io::Write};


fn main(){
	let today = chrono::Utc::today();
	let mut log_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}_erros",today))
        .expect("Erro em gerar arquivo, terminação completa sem execução.");

	if let Err(err) = laco_de_operacao(&mut log_file){
		log_error(&mut log_file, err.to_string())	
	};
}

fn log_error(file:&mut fs::File,error:String){
	let now = chrono::Utc::now();	
	let error_msg = format!("{}: {}",now,error);

	assert!(file.write_all(error_msg.as_bytes()).is_ok());
}

fn laco_de_operacao(file:&mut fs::File)->Result<(),TableProcessError>{
	let (db,email) = config_info::load_config("config.ini")?;

	let email_table_conection = MysqlConnection::establish(&format!("{}{}",&db.url,&db.email_db))?;

	let empresas = emails::CadastroEmails
		.select(emails::Empresa)
		.distinct()
		.load::<Option<String>>(&email_table_conection)?;

	for empresa in empresas.into_iter().filter(|emp|emp.is_some()){
		let emp = empresa.unwrap();
		match process_table(&db.url, &emp, &email,&email_table_conection){
			Ok(_) => println!("Tabela {} acessada com sucesso",&emp),
			Err(e) => {
				println!("Failure at table {}:\n{}",&emp,e);
				log_error(file, format!("Falha da base da empresa {}:{}",&emp,e));
			} 
		}
	}
	Ok(())
}

fn process_table(url:&str,empresa:&str,sender:&EmailSender,email_db:&MysqlConnection)->Result<(),TableProcessError>{
	let connec = MysqlConnection::establish(&format!("{}SGO_{}",url,empresa))?;

	let results = ocortb::Ocorrencia
		.filter(ocortb::EmailSended.eq("N"))
		.limit(10)
		.load::<Ocor>(&connec)?;

	let destinos = emails::CadastroEmails
		.filter(emails::Empresa.eq(empresa))
		.load::<Email>(email_db)?;
	
	for instance in results{
		let inst_id = instance.id;

		let ocor_soe = soetb::Ocorrencia_SOE
			.filter(soetb::OcoID.eq(inst_id))
			.load::<OcorSoe>(&connec)?;

		//retornar o título junto
		let (title, email_body) = build_message(empresa,instance,ocor_soe)?;

		if env::var("SEND").is_ok(){
			println!("Sending...");
			send_email(sender, &destinos, title,email_body)?;
			diesel::update(ocortb::Ocorrencia.find(inst_id))
				.set(ocortb::EmailSended.eq("S")).execute(&connec)?;
		}else{
			let filename = format!("./testres/{}{}.html",empresa,inst_id);
			println!("Generating results at: {}",&filename);
			let mut f = fs::File::create(filename).unwrap();

			assert!(f.write_all(email_body.as_bytes()).is_ok());
		}
	}
	Ok(())
}
/*
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

		let results = ocortb::Ocorrencia
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

		let destinos = emails::CadastroEmails
			.filter(emails::Empresa.eq(empresa_emails))
			.load::<Email>(&connec)?;

		assert!(!destinos.is_empty());
		println!("id do primeiro valor: {:?}",destinos[0].email_adrs.clone().unwrap());
		Ok(())
	}

	#[test]
	fn build_table_from_query(){
		use std::{fs,io::Write};

		let db = DbConfig::init("CHAVES_DB_LOCAL").unwrap();

		let (url,table) = db.make_table_urls().next().unwrap();
		let connec = MysqlConnection::establish(&format!("{}{}",url,table)).unwrap();

		let results = ocortb::Ocorrencia
			.filter(ocortb::EmailSended.eq("f"))
			.limit(1)
			.load::<Ocor>(&connec).unwrap();

		for inst in results{
			let ocor_soe = soetb::Ocorrencia_SOE
				.filter(soetb::OcoID.eq(inst.id))
				.load::<OcorSoe>(&connec).unwrap();
			
			let mut f = fs::File::create("./testres/tabela_from_query.html").unwrap();

			let html = build_message(inst, ocor_soe).unwrap();
			assert!(f.write_all(html.as_bytes()).is_ok());
		}
	}
	//TODO: consertar o test
	//#[test]
	#[allow(dead_code)]
	fn is_send_email_test_base()->Result<(),Box<dyn std::error::Error>>{
		use lettre::transport::smtp::authentication::Credentials;
		use lettre::{Message, SmtpTransport, Transport };
		
		let (user,senha,_relay) = (String::new(),String::new(),String::new());

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
} */