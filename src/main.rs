mod utilities;
mod config_info;
mod message_builder;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

use models::Ocor;
use config_info::{parse_email,parse_db_keys};
use diesel::prelude::*;
/*
use message_builder::build_message;
use utilities::send_email;
*/
fn main(){
	use schema::ocorrencia::dsl::*;

	let db = parse_db_keys().unwrap();
 	let connec = MysqlConnection::establish(&db.table_url()).unwrap();
	
	let results = ocorrencia.filter(EmailSended.eq("f"))
			.limit(1)
			.load::<Ocor>(&connec)
			.expect("erro na obtenção dos elementos da base de dados");


	//let destinatario = parse_email();

	//let email_message = build_message(results).unwrap_or_else(|e| panic!("{:?}",e));
	
	/*
	match send_email(&destinatario, email_message){
		Ok(_) => {
			diesel::update(Ocorrencias.find(ocor.id))
				.set(EmailSended.eq("t"))
				.execute(&connec)
				.unwrap_or_else(|_|{
					//TODO: the thing
					panic!("Falha em fazer o update do query");
				});
		},
		Err(e) =>{
			//criar uma função que loga para um arquivo quando a função falha
			panic!("{:?}",e);
		}
	} */
}