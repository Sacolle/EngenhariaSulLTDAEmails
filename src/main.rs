mod error;
mod db;
mod io;
mod templating;

#[macro_use]
extern crate diesel;

use chrono::Datelike;
use db::{chunks, query};
use io::{config_info::EmailSender, send_email::send_email};
use diesel::prelude::*;

//use message_builder::build_message;
use templating::build_from_template;

use error::TableProcessError;

use std::{fs,io::Write};

fn main(){
	let day = chrono::Utc::today();
	let mut log_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}-{}_log.txt",day.year(),day.month()))
        .expect("Erro em gerar arquivo, terminação completa sem execução.");

	let mut err_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}-{}_err.txt",day.year(),day.month()))
        .expect("Erro em gerar arquivo, terminação completa sem execução.");

	if let Err(err) = laco_de_operacao(&mut log_file, &mut err_file){
		log_error(&mut err_file, err.to_string())	
	};
}

fn log_error(file:&mut fs::File,error:String){
	let now = chrono::Utc::now();	
	let error_msg = format!("{}: {}\n",now,error);

	assert!(file.write_all(error_msg.as_bytes()).is_ok());
}

fn log_emails(file:&mut fs::File,empresa: &str, ids : Vec<i32>){
	let now = chrono::Utc::now();	
	let emails_sent = format!("{}: {} enviou emails com ids: {:?}\n",now, empresa, ids);

	assert!(file.write_all(emails_sent.as_bytes()).is_ok())
}

fn laco_de_operacao(log_file:&mut fs::File, err_file:&mut fs::File)->Result<(),TableProcessError>{
	let (db,email) = io::config_info::load_config("config.ini")?;

	let mut email_table_connection = MysqlConnection::establish(&format!("{}{}",&db.url,&db.email_db))?;

	let empresas = query::empresas(&mut email_table_connection)?;

	for empresa in empresas.into_iter().filter(|emp|emp.is_some()){
		let emp = empresa.unwrap();
		match process_table(&db.url, &emp, &email,&mut email_table_connection){
			Ok(ids_enviados) => {
				println!("Tabela {} acessada com sucesso",&emp);
				if let Some(ids) = ids_enviados{
					log_emails(log_file, &emp, ids);
				}
			},
			Err(e) => {
				println!("Failure at table {}:\n{}",&emp,e);
				log_error(err_file, format!("Falha da base da empresa {}:{}",&emp,e));
			} 
		}
	}
	Ok(())
}

fn process_table(url:&str,empresa:&str,sender:&EmailSender,email_db:&mut MysqlConnection)->Result<Option<Vec<i32>>,TableProcessError>{
	let mut connec = MysqlConnection::establish(&format!("{}SGO_{}",url,empresa))?;

	let results = query::ocorrencias(&mut connec, 10)?;
	
	let mut sent_emails = Vec::new();
	if results.is_empty(){
		println!("Nenhum resultado da tabela SGO_{}",empresa);
		return Ok(None);
	}

	let destinos = query::emails(email_db, empresa)?;
	
	for instance in results{
		let inst_id = instance.id;

		let ocor_soe = query::ocorrencias_soe(&mut connec, inst_id)?;
		let ex_info = chunks::TextInfo::build_from(&instance)?;
		let equipamentos = query::equipamentos(&mut connec, &ex_info, inst_id, 5)?;
		//retornar o título junto
		let (title, email_body) = build_from_template(empresa,&instance, ex_info, ocor_soe, equipamentos)?;

		send_email(sender, &destinos, title,email_body)?;
		let _ = query::update_ocorrencias(&mut connec, inst_id)?;

		sent_emails.push(inst_id);
	}
	Ok(Some(sent_emails))
}

#[cfg(test)]
mod tests{
	use std::collections::HashSet;
	use super::*;
	use crate::io::config_info;
	use diesel::dsl::not;
	use db::models::{Ocor,OcorSoe};
	use db::schema::{
		Ocorrencia::dsl as ocortb,
		Ocorrencia_SOE::dsl as soetb,
		CadastroEmails::dsl as emails
	};
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

		let extra = chunks::TextInfo::build_from(&instance).unwrap();

		let equipamentos = ocortb::Ocorrencia
			.filter(
				ocortb::SE.eq(extra.subestacao)
				.and(ocortb::AL.eq(extra.modulo)
				.and(ocortb::EQP.eq(extra.equipamento)))
			)
			.filter(not(ocortb::OcoID.eq(inst_id)))
			.order_by(ocortb::DtHrOco.desc())
			.limit(5)
			.load::<Ocor>(&mut connec).unwrap();

		//retornar o título junto
		let (_ ,email_body) = build_from_template(&empresa,&instance,extra, ocor_soe,equipamentos).unwrap();

		let filename = format!("./testres/{}{}.html",empresa,inst_id);
		let mut f = std::fs::File::create(filename).unwrap();

		assert!(f.write_all(email_body.as_bytes()).is_ok());
	}
}