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

fn main(){
	use schema::ocorrencia::dsl as ocortb;
	use schema::ocorrencia_soe::dsl as soetb;

	let db = DbConfig::init("CHAVES_DB_LOCAL").expect("Erro parse DbConfig");
	
	for table_url in db.make_table_urls(){
		let connec = MysqlConnection::establish(&table_url).unwrap();

		let results = ocortb::ocorrencia
			.filter(ocortb::EmailSended.eq("f"))
			.load::<Ocor>(&connec)
			.expect("erro na obtenção dos elementos da base de dados");

		//TODO: acessar a db de emails para dar fetch no email
		let destinatario = parse_email();

		for instance in results{
			let inst_id = instance.id;

			let ocor_soe = soetb::ocorrencia_soe.filter(soetb::OcoID.eq(inst_id))
				.load::<OcorSoe>(&connec)
				.expect("erro em pegar os vals da soe table");

			let email_message = build_message(instance,ocor_soe).unwrap();
			
			if let Err(e) = send_email(&destinatario, email_message){
				panic!("{:?}",e);
			}else{
				diesel::update(ocortb::ocorrencia.find(inst_id))
					.set(ocortb::EmailSended.eq("t"))
					.execute(&connec)
					.expect("Falha em fazer o update do query");
			} 
		}
	}
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