mod utilities;
mod config_info;
mod message_builder;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

use models::{Ocor,OcorSoe};
use config_info::{parse_email,DbConfig};
use diesel::prelude::*;
use message_builder::build_message;
use utilities::send_email;
use schema::ocorrencia::dsl as ocortb;
use schema::ocorrencia_soe::dsl as soetb;


fn main(){
	let db = DbConfig::init("CHAVES_DB_LOCAL").expect("Falha no .ini");
	
	for table_url in db.make_table_urls(){
		if let Err(e) = process_table(&table_url){
			//TODO: log the errors
			println!("{:?}",e);
		}else{
			println!("Tabela com link:\n{}\nAcessada com sucesso",&table_url);
		}
	}
}
fn process_table(table_url:&str)->Result<(),Box<dyn std::error::Error>>{
	let connec = MysqlConnection::establish(table_url)?;

	let results = ocortb::ocorrencia
		.filter(ocortb::EmailSended.eq("f"))
		.limit(10)
		.load::<Ocor>(&connec)?;

	let destinatario = parse_email(); //TODO: acessar a db de emails para dar fetch no email

	for instance in results{
		let inst_id = instance.id;

		let ocor_soe = soetb::ocorrencia_soe
			.filter(soetb::OcoID.eq(inst_id))
			.load::<OcorSoe>(&connec)?;

		send_email(&destinatario, build_message(instance,ocor_soe)?)?;

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

		for table_url in db.make_table_urls(){
			MysqlConnection::establish(&table_url)?;
		}
		Ok(())
	}
	#[test]
	fn is_query_data_wroking()->Result<(),Box<dyn Error>>{
		let db = DbConfig::init("CHAVES_DB_LOCAL")?;

		let table_url = db.make_table_urls().next().unwrap();
		let connec = MysqlConnection::establish(&table_url)?;

		let results = ocortb::ocorrencia
			.filter(ocortb::EmailSended.eq("f"))
			.load::<Ocor>(&connec)?;

		assert!(!results.is_empty());
		Ok(())
	}
	#[test]
	fn build_table_from_query()->Result<(),Box<dyn std::error::Error>>{
		use std::{fs,io::Write};

		let db = DbConfig::init("CHAVES_DB_LOCAL")?;

		let table_url = db.make_table_urls().next().unwrap();
		let connec = MysqlConnection::establish(&table_url)?;

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
	#[test]
	fn is_send_email_working()->Result<(),Box<dyn std::error::Error>>{
		let destinatario = parse_email(); //TODO: acessar a db de emails para dar fetch no email

		send_email(&destinatario, String::from("Test"))?;
		Ok(())
	}
}