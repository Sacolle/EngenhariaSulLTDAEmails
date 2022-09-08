use serde::Deserialize;
use serde_json::from_str as JSONparse;
use diesel::mysql::data_types::MysqlTime;

use crate::db::models::{Ocor,OcorSoe};
use crate::error::{MissignFieldError,TableProcessError,};

pub struct TextInfo<'a>{
	pub subestacao:&'a str,
	pub modulo:&'a str,
	pub equipamento:&'a str,
	pub tipo: &'a str,
	pub inicio: Option<chrono::NaiveDateTime>,
	pub termino:Option<chrono::NaiveDateTime>,
	pub duracao:f64
}

impl<'a> TextInfo<'a>{
	pub fn build_from(caso:&'a Ocor)->Result<Self,TableProcessError>{
		let tipo = match caso.tipo_oco.as_ref()
			.ok_or(MissignFieldError::new("Tipo de Ocorrencia"))?
			.as_str(){
				"C" => Ok("Comandado"),
				"R" => Ok("Religamento"),
				"L" => Ok("LockOut"),
				"N" => Ok("Normalizou"),
				tipo => Err(MissignFieldError(format!("Tipo {} não é válido",tipo)))
			}?;

		let subestacao = caso.se.as_ref()
			.ok_or(MissignFieldError::new("SE"))?
			.as_str();

		let modulo= caso.al.as_ref()
			.ok_or(MissignFieldError::new("AL"))?
			.as_str();
		
		let equipamento = caso.eqp.as_ref()
			.ok_or(MissignFieldError::new("EQP"))?
			.as_str();
		
		let inicio  = parse_time(caso.hora_ini);
		let termino = parse_time(caso.hora_fim);
			
		let duracao = caso.duracao.unwrap_or(0.0) as f64;

		Ok(TextInfo { subestacao, modulo, equipamento, tipo, inicio, termino, duracao})
	}
}

#[derive(Deserialize,Debug)]
pub struct FaltasTabela{
	#[serde(alias = "IaF")]
	pub fase_a: f32,
	#[serde(alias = "IbF")]
	pub fase_b: f32,
	#[serde(alias = "IcF")]
	pub fase_c: f32,
	#[serde(alias = "InF")]
	pub fase_n: f32,
}

#[derive(Deserialize,Debug)]
pub struct CondPrePosTabela{
	#[serde(alias = "P")]
	pub potencia_ativa: f32,
	#[serde(alias = "Ia")]
	pub fase_a: f32,
	#[serde(alias = "Ib")]
	pub fase_b: f32,
	#[serde(alias = "Ic")]
	pub fase_c: f32,
	#[serde(alias = "In")]
	pub fase_n: f32,
}
pub struct TableInfo{
	pub faltas: FaltasTabela,
	pub cond_pre: CondPrePosTabela,
	pub cond_pos: CondPrePosTabela,
	pub eventos: Vec<OcorrenciaSoe>
}

impl TableInfo{
	pub fn build_from(caso:&Ocor,soe:Vec<OcorSoe>)->Result<Self,TableProcessError>{
		let cond_pre:CondPrePosTabela = JSONparse(
			caso.condpre.as_ref()
			.ok_or(MissignFieldError::new("condPre"))?
		)?;
		let cond_pos:CondPrePosTabela = JSONparse(
			caso.condpos.as_ref()
			.ok_or(MissignFieldError::new("condPós"))?
		)?;
		let faltas:FaltasTabela = JSONparse(
			caso.faltas.as_ref()
			.ok_or(MissignFieldError::new("tabelaFaltas"))?
		)?;

		let eventos = soe.into_iter()
			.map(|val|OcorrenciaSoe::build_from(val))
			.collect();

		Ok(TableInfo{faltas,cond_pre,cond_pos,eventos})
	}
}

pub struct OcorrenciaSoe{
	pub hora_inicio: Option<chrono::NaiveDateTime>,
	pub hora_fim: Option<chrono::NaiveDateTime>,
	pub mensagem:String,
	pub agente:String
}

impl OcorrenciaSoe{
	pub fn build_from(soe:OcorSoe)->Self{
		let hora_inicio= soe.hora_ini;
		let hora_fim = soe.hora_fim;
		let mensagem = soe.mensagem.unwrap_or(String::new());
		let agente = soe.actor_id.unwrap_or(String::new());
		
		OcorrenciaSoe { hora_inicio, hora_fim, mensagem, agente }
	}
}

pub struct PrevEqp{
	pub faltas: FaltasTabela,
	pub inicio: Option<chrono::NaiveDateTime>,
	pub prot_sen: String,
	pub prot_atu: String
}

impl PrevEqp{
	pub fn build_from(caso: Ocor)->Result<Self,TableProcessError>{
		let faltas:FaltasTabela = JSONparse(
			caso.faltas.as_ref()
			.ok_or(MissignFieldError::new("tabelaFaltas"))?
		)?;
		let inicio = parse_time(caso.hora_oco);
		let prot_sen = caso.prot_sen.unwrap_or(String::new());
		let prot_atu = caso.prot_atu.unwrap_or(String::new());
		Ok(PrevEqp{faltas,inicio,prot_sen,prot_atu})
	}
}

pub fn parse_time(time: MysqlTime)->Option<chrono::NaiveDateTime>{
	match time{
		MysqlTime { year:0, month:0, day:0, hour:0,
			minute:0, second:0, second_part:0,
			.. }=>None,
		MysqlTime { year, month, day,
			hour, minute, second,
			.. }=> Some(chrono::NaiveDate::from_ymd(year as i32,month,day)
				.and_hms(hour,minute,second))
		}
}
