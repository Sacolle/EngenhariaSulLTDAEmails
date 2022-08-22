use configparser::ini::Ini;
use std::collections::HashMap;


use crate::error::{MissignFieldError,TableProcessError};


type Map = HashMap<String,HashMap<String,Option<String>>>;


pub fn load_config(file:&str)->Result<(DbConfig,EmailSender),TableProcessError>{
	let config = Ini::new().load(file)?;
	let db_config = DbConfig::init(&config)?;
	let email_sender = EmailSender::init(&config)?;

	Ok((db_config,email_sender))
}

pub struct DbConfig{
	pub url: String,
	pub email_db: String
}

impl DbConfig {
	pub fn init(map:&Map) -> Result<DbConfig,MissignFieldError>{
		let section = map.get("CHAVES_DB_MARIA")
			.ok_or(MissignFieldError::new("CHAVES_DB_MARIA"))?;

		let email_db = section.get("EMAILDB")
			.ok_or(MissignFieldError::new("nome"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of nome"))?
			.clone();
		
		let user = section.get("USER")
			.ok_or(MissignFieldError::new("user"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of user"))?
			.clone();

		let senha = section.get("SENHA")
			.ok_or(MissignFieldError::new("senha"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of senha"))?
			.clone();

		let url_parcial = section.get("URL")
			.ok_or(MissignFieldError::new("url"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of url"))?
			.clone();

		let url = format!("mysql://{}:{}@{}/",user,senha,url_parcial);		

		Ok(DbConfig { url, email_db })
	}
}

pub struct EmailSender{
	pub nome: String,
	pub email: String,
	pub senha: String
}

impl EmailSender{
	pub fn init(map:&Map)->Result<Self,MissignFieldError>{
		let section = map.get("EMAIL_CREDS")
			.ok_or(MissignFieldError::new("EMAIL_CREDS"))?;

		let nome= section.get("nome")
			.ok_or(MissignFieldError::new("nome"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of nome"))?
			.clone();

		let email = section.get("email")
			.ok_or(MissignFieldError::new("email"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of email"))?
			.clone();

		let senha = section.get("senha")
			.ok_or(MissignFieldError::new("senha"))?
			.as_ref()
			.ok_or(MissignFieldError::new("val of senha"))?
			.clone();

		Ok(EmailSender { nome ,email, senha })
	}
}
#[cfg(test)]
mod tests{
    use super::*;
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
}