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


fn main()->Result<(),Box<dyn std::error::Error>>{
	let db = DbConfig::init("CHAVES_DB_LOCAL")?;
	
	for table_url in db.make_table_urls(){
		let connec = MysqlConnection::establish(&table_url)?;

		let results = ocortb::ocorrencia
			.filter(ocortb::EmailSended.eq("f"))
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
	}
	Ok(())
}


#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn testing_connect(){
		let db = DbConfig::init("CHAVES_DB_LOCAL").expect("Erro parse DbConfig");

		for table_url in db.make_table_urls(){
			let connec = MysqlConnection::establish(&table_url);
			if let Err(err) = connec{
				panic!("{:?}",err);
			}
		}
	}
}