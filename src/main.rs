mod utilities;
mod config_info;
mod message_builder;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

use models::Ocor;
use config_info::{parse_email,DbConfig};
use diesel::prelude::*;
use message_builder::build_message;
use utilities::send_email;

fn main(){
	use schema::ocorrencia::dsl::*;

	let db = DbConfig::init("CHAVES_DB_LOCAL").expect("Erro parse DbConfig");
	
	
	for table_url in db.make_table_urls(){
		let connec = MysqlConnection::establish(&table_url).unwrap();

		let results = ocorrencia.filter(EmailSended.eq("f"))
			.limit(5)
			.load::<Ocor>(&connec)
			.expect("erro na obtenção dos elementos da base de dados");

		//TODO: acessar a db de emails para dar fetch no email
		let destinatario = parse_email();

		for instance in results{
			let inst_id = instance.id;
			let email_message = build_message(instance,String::new()).unwrap();
			
			
			match send_email(&destinatario, email_message){
				Ok(_) => {
					diesel::update(ocorrencia.find(inst_id))
						.set(EmailSended.eq("t"))
						.execute(&connec)
						.unwrap_or_else(|_|{
							//TODO: the thing
							panic!("Falha em fazer o update do query");
						});
				},
				Err(e) =>{
					panic!("{:?}",e);
				}
			} 
		}
	}
}