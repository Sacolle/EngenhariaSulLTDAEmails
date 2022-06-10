use configparser::ini::Ini;
use std::error::Error;

pub struct DbConfig{
	pub url: String,
	pub db: String
}

impl DbConfig {
	pub fn init(section:&str) -> Result<DbConfig,Box<dyn Error>>{
		let mut config = Ini::new();
		config.load("config.ini")?;

		let db = config.get(section,"DB").unwrap();
		
		let url = format!("mysql://{}:{}@{}/",
			config.get(section, "USER").unwrap(),
			config.get(section, "SENHA").unwrap(),
			config.get(section, "URL").unwrap(),
		);		

		Ok(DbConfig { url, db })
	}
	pub fn make_table_urls<'a>(&'a self)-> impl Iterator<Item = String> + 'a {
		self.db
			.split(',')
			.map(|e|format!("{}{}",self.url,e.trim()))
	}
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


#[cfg(test)]
mod tests{
    use super::*;
	#[test]
	fn valid_base_url(){
		let db = DbConfig::init("TEST_SECTION").unwrap();
		assert_eq!("mysql://user:senha@google.com/",&db.url);
	}
	#[test]
	fn valid_ini_file(){
		let mut config = Ini::new();
		let map = config.load("config.ini");
		assert!(map.is_ok());
		let values = map.unwrap();
		for fields in values.iter(){
			for keys in fields.1.iter(){
				if !keys.1.is_some(){
					panic!("campo {} não contém valor",keys.0);
				}
			}
		}
	}
	#[test]
	fn valid_iterator(){
		let db = DbConfig::init("CHAVES_DB_LOCAL").unwrap();
		let test_vec = vec!["test_db"];
		
		for url in db.make_table_urls().zip(test_vec){
			assert_eq!(url.0,format!("{}{}",&db.url,url.1));
		}
	}

}