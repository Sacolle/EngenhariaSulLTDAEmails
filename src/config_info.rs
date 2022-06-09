use std::collections::HashMap;

use configparser::ini::Ini;

pub struct DbConfig{
	pub url: String,
	pub db: String
}

enum DbTables{
	Single(String),
	Multi(Vec<String>)
}

impl DbConfig {
	pub fn new(map: &HashMap<String, Option<String>>) -> Self{
		let db_url = map.get("URL").unwrap().as_ref().unwrap();
		let user = map.get("USER").unwrap().as_ref().unwrap();
		let senha = map.get("SENHA").unwrap().as_ref().unwrap();
		
		//clonando o valor no momento, não sei se é necessario um método mais eficiente
		//acho q não
		let db = map.get("DB").unwrap().as_ref().unwrap().to_owned();
		
		let url = format!("mysql://{}:{}@{}/",user,senha,db_url);		

		DbConfig { url, db }
	}
	pub fn table_url(&self)->String{
		format!("{}{}",self.url,self.db)
	}
}


pub fn parse_db_keys()->Result<DbConfig,Box<dyn std::error::Error>>{
	let mut config = Ini::new();
	let map = config.load("config.ini")?;

	Ok(DbConfig::new(map.get("CHAVES_DB_LOCAL").unwrap()))
}

pub fn parse_email()->String{
	let mut config = Ini::new();
	//se não carregar, checa as std::env::vars, se tb nao funciona, da panico
	config.load("config.ini").expect("Não tem config.ini");
	
	match config.get("EMAIL_ADRS","email"){
		Some(r) => r,
		None => panic!()
	}
}

pub fn get_email_creds()->String{
	let mut config = Ini::new();
	//se não carregar, checa as std::env::vars, se tb nao funciona, da panico
	config.load("config.ini").expect("Não tem config.ini");
	
	match config.get("GMAIL_CREDS","senha"){
		Some(r) => r,
		None => panic!()
	}
}