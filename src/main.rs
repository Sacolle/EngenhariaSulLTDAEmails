mod utilities;
mod config_info;
mod message_builder;
mod error;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

use chrono::Datelike;
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
use std::{fs,io::Write};

fn main(){
	let day = chrono::Utc::today();
	let mut log_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}-{}_err.txt",day.year(),day.month()))
        .expect("Erro em gerar arquivo, terminação completa sem execução.");

	if let Err(err) = laco_de_operacao(&mut log_file){
		log_error(&mut log_file, err.to_string())	
	};
}

fn log_error(file:&mut fs::File,error:String){
	let now = chrono::Utc::now();	
	let error_msg = format!("{}: {}\n",now,error);

	assert!(file.write_all(error_msg.as_bytes()).is_ok());
}

fn laco_de_operacao(file:&mut fs::File)->Result<(),TableProcessError>{
	let (db,email) = config_info::load_config("config.ini")?;

	let mut email_table_conection = MysqlConnection::establish(&format!("{}{}",&db.url,&db.email_db))?;

	let empresas = emails::CadastroEmails
		.select(emails::Empresa)
		.distinct()
		.load::<Option<String>>(&mut email_table_conection)?;

	for empresa in empresas.into_iter().filter(|emp|emp.is_some()){
		let emp = empresa.unwrap();
		match process_table(&db.url, &emp, &email,&mut email_table_conection){
			Ok(_) => println!("Tabela {} acessada com sucesso",&emp),
			Err(e) => {
				println!("Failure at table {}:\n{}",&emp,e);
				log_error(file, format!("Falha da base da empresa {}:{}",&emp,e));
			} 
		}
	}
	Ok(())
}

fn process_table(url:&str,empresa:&str,sender:&EmailSender,email_db:&mut MysqlConnection)->Result<(),TableProcessError>{
	let mut connec = MysqlConnection::establish(&format!("{}SGO_{}",url,empresa))?;

	let results = ocortb::Ocorrencia
		.filter(ocortb::EmailSended.eq("N"))
		.limit(10)
		.load::<Ocor>(&mut connec)?;
	
	if results.is_empty(){
		println!("Nenhum resultado da tabela SGO_{}",empresa);
		return Ok(());
	}

	let destinos = emails::CadastroEmails
		.filter(emails::Empresa.eq(empresa))
		.load::<Email>(email_db)?;
	
	for instance in results{
		let inst_id = instance.id;

		let ocor_soe = soetb::Ocorrencia_SOE
			.filter(soetb::OcoID.eq(inst_id))
			.load::<OcorSoe>(&mut connec)?;

		//retornar o título junto
		let (title, email_body) = build_message(empresa,instance,ocor_soe)?;

		send_email(sender, &destinos, title,email_body)?;
		diesel::update(ocortb::Ocorrencia.find(inst_id))
			.set(ocortb::EmailSended.eq("S"))
			.execute(&mut connec)?;
	}
	Ok(())
}
#[cfg(test)]
mod tests{
	use std::collections::HashSet;

use super::*;
	use crate::config_info;
	#[test]
	fn testing_connect(){
		let (db,_) = config_info::load_config("config.ini").unwrap();

		assert!(MysqlConnection::establish(&format!("{}{}",db.url,db.email_db)).is_ok());
	}
	#[test]
	fn query_empresas(){
		let (db,_) = config_info::load_config("config.ini").unwrap();

		let mut email_table_conection = MysqlConnection::establish(&format!("{}{}",db.url,db.email_db)).unwrap();

		let empresas = emails::CadastroEmails
			.select(emails::Empresa)
			.distinct()
			.load::<Option<String>>(&mut email_table_conection).unwrap();

		//println!("{:?}",empresas.iter().map(|v|v.as_ref().unwrap().as_str()).collect::<Vec<&str>>());
		let mut uniq = HashSet::new();
		let res = empresas.into_iter().all(move|x|uniq.insert(x.unwrap()));
		assert!(res);
	}
	#[test]
	fn build_table_from_query(){
		let (db,_) = config_info::load_config("config.ini").unwrap();
		let url = db.url;
		let empresa = String::from("CERIM");

		let mut connec = MysqlConnection::establish(&format!("{}SGO_{}",&url,&empresa)).unwrap();

		let results = ocortb::Ocorrencia
			.filter(ocortb::OcoID.eq(1))
			.filter(ocortb::EmailSended.eq("S"))
			.limit(10)
			.load::<Ocor>(&mut connec).unwrap();
		
		let instance = results.into_iter().next();
		assert!(instance.is_some());
		let instance = instance.unwrap();
		let inst_id = instance.id;

		let ocor_soe = soetb::Ocorrencia_SOE
			.filter(soetb::OcoID.eq(inst_id))
			.load::<OcorSoe>(&mut connec).unwrap();

		//retornar o título junto
		let (_ ,email_body) = build_message(&empresa,instance,ocor_soe).unwrap();

		let filename = format!("./testres/{}{}.html",empresa,inst_id);
		let mut f = std::fs::File::create(filename).unwrap();

		assert!(f.write_all(email_body.as_bytes()).is_ok());
	}
}