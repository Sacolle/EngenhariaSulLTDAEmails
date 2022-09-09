use std::{fmt,error};

#[derive(Debug)]
pub enum TableProcessError{
	MissingData(MissignFieldError),
	MalformedData(serde_json::Error),
	EmailFormError(lettre::error::Error),
	EmailSendError(lettre::transport::smtp::Error),
	EmailParseError(lettre::address::AddressError),
	SqlQueryError(diesel::result::Error),
	SqlConnectionError(diesel::ConnectionError),
	LoadIniError(String),
	TemplatingError(tera::Error)
}
impl fmt::Display for TableProcessError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self{
			Self::MissingData(e)=> write!(f,"{}",e),
			Self::MalformedData(e)=>write!(f,"Failed to parse data to Json:\n\t{}",e),
			Self::EmailFormError(e)=>write!(f,"Email form is configured incorrectly:\n\t{}",e),
			Self::EmailSendError(e)=>write!(f,"Failed to send email:\n\t{}",e),
			Self::EmailParseError(e) => write!(f,"Email Adress is malformed:\n\t{}",e),
			Self::SqlQueryError(e) => write!(f,"SQL query failed:\n\t{}",e),
			Self::SqlConnectionError(e) => write!(f,"Failed to connect to DB:\n\t{}",e),
			Self::LoadIniError(e)=> write!(f,"Falha em carregar o arquivo Ini:\n\t{}",e),
			Self::TemplatingError(e)=> write!(f,"Falha no processamento do template:\n\t{}",e),
		}
	}
}

#[derive(Debug)]
pub struct MissignFieldError(pub String);

impl MissignFieldError{
	pub fn new(w:&str)->Self{
		MissignFieldError(String::from(w))
	}
}

impl fmt::Display for MissignFieldError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,"Missing field {}",self.0)
	}
}
impl error::Error for MissignFieldError{}

impl From<MissignFieldError> for TableProcessError{
	fn from(e: MissignFieldError) -> Self {
		TableProcessError::MissingData(e)
	}
}

impl From<serde_json::Error> for TableProcessError{
	fn from(e: serde_json::Error) -> Self {
		TableProcessError::MalformedData(e)
	}
}

impl From<lettre::error::Error> for TableProcessError{
	fn from(e: lettre::error::Error) -> Self {
		TableProcessError::EmailFormError(e)
	}
}

impl From<lettre::transport::smtp::Error> for TableProcessError{
	fn from(e: lettre::transport::smtp::Error) -> Self {
		TableProcessError::EmailSendError(e)
	}
}

impl From<lettre::address::AddressError> for TableProcessError{
	fn from(e: lettre::address::AddressError) -> Self {
		TableProcessError::EmailParseError(e)
	}
}

impl From<diesel::result::Error> for TableProcessError{
	fn from(e: diesel::result::Error) -> Self {
		TableProcessError::SqlQueryError(e)
	}
}

impl From<diesel::ConnectionError> for TableProcessError{
	fn from(e: diesel::ConnectionError) -> Self {
		TableProcessError::SqlConnectionError(e)
	}
}

impl From<String> for TableProcessError{
	fn from(e: String) -> Self {
		TableProcessError::LoadIniError(e)
	}
}

impl From<tera::Error> for TableProcessError{
	fn from(e: tera::Error) -> Self {
		TableProcessError::TemplatingError(e)
	}
}