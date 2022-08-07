use configparser::ini::Ini;
use std::error::Error;


use crate::error::MissignFieldError;

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
	pub fn make_table_urls<'a>(&'a self)-> impl Iterator<Item = (String,String)> + 'a {
		self.db
			.split(',')
			.map(|e|
				(self.url.clone(), e.trim().to_string())
			)
	}
}

pub struct EmailSender{
	pub nome: String,
	pub email: String,
	pub senha: String
}

impl EmailSender{
	pub fn get(section:&str)->Result<Self,Box<dyn Error>>{
		let mut config = Ini::new();
		config.load("config.ini")?;

		let nome= config.get(section,"nome")
			.ok_or(MissignFieldError::new("nome"))?;
		let email = config.get(section,"email")
			.ok_or(MissignFieldError::new("email"))?;
		let senha = config.get(section,"senha")
			.ok_or(MissignFieldError::new("senha"))?;

		Ok(EmailSender { nome ,email, senha })
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
			assert_eq!((url.0).1,url.1.to_owned());
		}
	}

}