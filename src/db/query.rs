use diesel::{MysqlConnection,result};
use diesel::prelude::*;
use crate::db::models::{Ocor,OcorSoe, Email};
use crate::db::schema::{
	Ocorrencia::dsl as ocortb,
	Ocorrencia_SOE::dsl as soetb,
	CadastroEmails::dsl as emails
};
use crate::db::chunks::TextInfo;


pub fn equipamentos(connec: &mut MysqlConnection,ex_info:&TextInfo,inst_id:i32)->Result<Vec<Ocor>, result::Error>{
	ocortb::Ocorrencia
		.filter(
			ocortb::SE.eq(ex_info.subestacao)
			.and(ocortb::AL.eq(ex_info.modulo)
			.and(ocortb::EQP.eq(ex_info.equipamento)))
		)
		.filter(diesel::dsl::not(ocortb::OcoID.eq(inst_id)))
		.order_by(ocortb::DtHrOco.desc())
		.limit(5)
		.load::<Ocor>(connec)
}

pub fn empresas(connec: &mut MysqlConnection)->Result<Vec<Option<String>>, result::Error>{
	emails::CadastroEmails
		.select(emails::Empresa)
		.distinct()
		.load::<Option<String>>(connec)
}

pub fn emails(connec: &mut MysqlConnection, empresa:&str)->Result<Vec<Email>, result::Error>{
	emails::CadastroEmails
		.filter(emails::Empresa.eq(empresa))
		.load::<Email>(connec)
}

pub fn ocorrencias(connec: &mut MysqlConnection, limit:i64)->Result<Vec<Ocor>, result::Error>{
	ocortb::Ocorrencia
		.filter(ocortb::EmailSended.eq("N"))
		.limit(limit)
		.load::<Ocor>(connec)
}

pub fn ocorrencias_soe(connec: &mut MysqlConnection, id:i32)->Result<Vec<OcorSoe>, result::Error>{
	soetb::Ocorrencia_SOE
		.filter(soetb::OcoID.eq(id))
		.load::<OcorSoe>(connec)
}

pub fn update_ocorrencias(connec: &mut MysqlConnection, id:i32)->Result<usize, result::Error>{
	diesel::update(ocortb::Ocorrencia.find(id))
		.set(ocortb::EmailSended.eq("S"))
		.execute(connec)
}