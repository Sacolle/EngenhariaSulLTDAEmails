use tera::{Context,Tera};

use crate::db::{models::{Ocor,OcorSoe},chunks::{TextInfo,TableInfo,PrevEqp}};
use crate::error::TableProcessError;

/*
* TextInfo contém informações para o header
* TableInfo contém informação para as tabelas de falta e SOE
* Vec<PrevEqp> contém informações para a tabela dos equipamentos anteriores
*/
pub fn build_from_template(empresa:&str, caso: &Ocor, info: TextInfo, soe: Vec<OcorSoe>, eqp:Vec<Ocor>)->Result<(String,String),TableProcessError>{
	let t = Tera::new("templates/*")?;
	let mut ctx = Context::new();

	let table_info = TableInfo::build_from(caso,soe)?;
	let equipamentos:Vec<PrevEqp> = eqp.into_iter()
		.filter_map(|val|PrevEqp::build_from(val).ok())
		.collect();

	let title =  format!("{}:{} {} {}",
		&info.tipo,
		&info.subestacao,
		&info.modulo,
		&info.equipamento);

	ctx.insert("empresa",empresa);
	ctx.insert("subestacao", &info.subestacao);
	ctx.insert("modulo", &info.modulo);
	ctx.insert("equipamento", &info.equipamento);
	ctx.insert("inicio", &info.inicio);
	ctx.insert("termino", &info.termino);
	ctx.insert("duracao", &info.duracao);
	ctx.insert("faltas", &table_info.faltas);
	ctx.insert("pre_ocor", &table_info.cond_pre);
	ctx.insert("pos_ocor", &table_info.cond_pos);
	ctx.insert("ocor_soe", &table_info.eventos);
	ctx.insert("eqps_ant", &equipamentos);

	Ok((title,t.render("base.html", &ctx)?))
}
/*
	empresa
	subestacao
	modulo
	equipamento
	inicio
	termino
	duracao
	faltas
	pre_ocor
	pos_ocor
	ocor_soe {inicio, mensagem, fim, operador}
	eqps_ant {faltas, inicio, prot_sen, prot_atu}
*/



/*
#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn tera_test(){
		
		assert_eq!(target,res);
	}
} */