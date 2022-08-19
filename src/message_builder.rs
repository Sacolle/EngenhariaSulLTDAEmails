
use serde::Deserialize;
use serde_json::from_str as JSONparse;

use crate::models::{Ocor,OcorSoe};
use crate::error::{MissignFieldError,TableProcessError};

const HTMLHEAD: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
	<head>
		<meta charset="UTF-8">
		<meta name="viewport" content="width=device-width, initial-scale=1.0">
		<title>tabela</title>
		<style>
			table,td,tr,th{
				border: 1px solid black;
			}
			td,th{
				padding: 2.5px;
			}
			.titulos{
				text-align: center;
			}
		</style>
	</head>
	<body>
		<table style="width: 50%;">
	"#;

const HTMLTAIL: &str = r#"
			</table>
		</body>
	</html>
	"#;

const HEADROW: &str = r#"
			<tr class="titulos">
			  <td>Hora do Evento</td>
			  <td>Mensagem</td>
			  <td>Hora da Gravação</td>
			  <td>Operador</td>
			</tr>
"#;

trait HTMLTable{
	fn to_html(&self, info:&ExtraInfo)->String;
}

#[derive(Deserialize,Debug)]
struct FaltasTabela{
	#[serde(alias = "IFa")]
	fase_a: f32,
	#[serde(alias = "IFb")]
	fase_b: f32,
	#[serde(alias = "IFc")]
	fase_c: f32,
	#[serde(alias = "IFn")]
	fase_n: f32,
}

impl HTMLTable for FaltasTabela{
	fn to_html(&self, info:&ExtraInfo)->String {
		let mut tabela = String::from("<tr><th colspan = 4>CORRENTES DE FALTA</th></tr>\n");
		tabela.push_str(HEADROW);

		tabela.push_str(&make_row(info, &format!("Corrente de falta Fase A = {}",self.fase_a)));
		tabela.push_str(&make_row(info, &format!("Corrente de falta Fase B = {}",self.fase_b)));
		tabela.push_str(&make_row(info, &format!("Corrente de falta Fase C = {}",self.fase_c)));
		tabela.push_str(&make_row(info, &format!("Corrente de falta Neutro = {}",self.fase_n)));
		tabela
	}
}


#[derive(Deserialize,Debug)]
struct CondPrePosTabela{
	#[serde(alias = "P")]
	potencia_ativa: f32,
	#[serde(alias = "Ia")]
	fase_a: f32,
	#[serde(alias = "Ib")]
	fase_b: f32,
	#[serde(alias = "Ic")]
	fase_c: f32,
	#[serde(alias = "In")]
	fase_n: f32,
}

impl HTMLTable for CondPrePosTabela{
	fn to_html(&self, info: &ExtraInfo)->String {
		let mut tabela = String::from("<tr><th colspan = 4>CONDIÇÃO OPERAÇÃO PRÉ-OCORRÊNCIA</th></tr>\n");
		tabela.push_str(HEADROW);

		tabela.push_str(&make_row(info, &format!("Potência Ativa = {}",self.potencia_ativa)));
		tabela.push_str(&make_row(info, &format!("Corrente na Fase A = {}",self.fase_a)));
		tabela.push_str(&make_row(info, &format!("Corrente na Fase B = {}",self.fase_b)));
		tabela.push_str(&make_row(info, &format!("Corrente na Fase C = {}",self.fase_c)));
		tabela.push_str(&make_row(info, &format!("Corrente no Neutro = {}",self.fase_n)));
		tabela
	}
}

struct ExtraInfo<'a>( &'a chrono::NaiveDateTime, &'a chrono::NaiveDateTime);

impl<'a> ExtraInfo<'a>{
	fn new(ocor: &'a Ocor)->Self{
		ExtraInfo ( &ocor.hora_ini,  &ocor.hora_fim)
	} 
}

pub fn parse_soe(soe:Vec<OcorSoe>)->String{
	let mut tabela = String::from("<tr><th colspan = 4>EVENTOS</th></tr>\n");
	tabela.push_str(HEADROW);

	for val in soe{
		tabela.push_str(&format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
			val.hora_ini.unwrap_or(chrono_def()),
			val.mensagem.as_ref().unwrap_or(&String::new()),
			val.hora_fim.unwrap_or(chrono_def()),
			val.actor_id.as_ref().unwrap_or(&String::new()),
		));
	}
	tabela
}

fn make_row(info:&ExtraInfo,line: &str)->String{ 
	format!("<tr><td>{}</td><td>{}</td><td>{}</td><td></td></tr>\n",info.0, line, info.1)
}

fn chrono_def()->chrono::NaiveDateTime{
	chrono::NaiveDate::from_ymd(1,1,1).and_hms(1, 1, 1)
}

pub fn build_message(caso: Ocor,soe: Vec<OcorSoe>)->Result<String, TableProcessError>{
	let mut result = String::from(HTMLHEAD);
	let info = ExtraInfo::new(&caso);

	let pre_ocor:CondPrePosTabela = JSONparse(
		caso.condpre.as_ref()
		.ok_or(MissignFieldError::new("condPre"))?
	)?;
	let pos_ocor:CondPrePosTabela = JSONparse(
		caso.condpos.as_ref()
		.ok_or(MissignFieldError::new("condPós"))?
	)?;
	let faltas:FaltasTabela = JSONparse(
		caso.faltas.as_ref()
		.ok_or(MissignFieldError::new("tabelaFaltas"))?
	)?;

	result.push_str(&pre_ocor.to_html(&info));
	result.push_str(&faltas.to_html(&info));
	result.push_str(&parse_soe(soe));
	result.push_str(&pos_ocor.to_html(&info));
	
	result.push_str(HTMLTAIL);

	return Ok(result);
}

#[cfg(test)]
mod tests{
	use super::*;
	use std::{fs,io::Write};

	#[test]
	fn proper_json_faltas(){
		let faltas = Some(String::from(r#"{"IFa":1.5,"IFb":1.5,"IFc":1.5,"IFn":1.5}"#));
		let pre_ocor:Result<FaltasTabela, serde_json::Error> = JSONparse(faltas.as_ref().unwrap());
		
		match pre_ocor{
			Ok(val)=>{
				assert_eq!(val.fase_a,1.5);
				assert_eq!(val.fase_b,1.5);
				assert_eq!(val.fase_c,1.5);
				assert_eq!(val.fase_n,1.5);
			},
			Err(e)=>panic!("Erro no parse: {}",e)
		}
	}

	#[test]
	fn proper_json_prepos(){
		let prepos = Some(String::from(r#"{"P":1.5,"Ia":1.5,"Ib":1.5,"Ic":1.5,"In":1.5}"#));
		let pre_ocor:Result<CondPrePosTabela, serde_json::Error> = JSONparse(prepos.as_ref().unwrap());
		
		match pre_ocor{
			Ok(val)=>{
				assert_eq!(val.potencia_ativa,1.5);
				assert_eq!(val.fase_a,1.5);
				assert_eq!(val.fase_b,1.5);
				assert_eq!(val.fase_c,1.5);
				assert_eq!(val.fase_n,1.5);
			},
			Err(e)=>panic!("Erro no parse: {}",e)
		}
	}

	#[test]
	fn proper_hmtl(){
		let mut f = fs::File::create("./testres/tabela_para_envio.html").unwrap();

		let caso = Ocor {
			id: 1,
			se: None,
			al: None,
			eqp: None,
			hora_ini: chrono_def(),
			hora_fim: chrono_def(),
			duracao: None,
			faltas: Some(String::from(r#"{"IFa":96.5,"IFb":8.9,"IFc":94.2,"IFn":68.5}"#)),
			condpre: Some(String::from(r#"{"P":85.5,"Ia":62,"Ib":9.4,"Ic":22.3,"In":68.8}"#)),
			condpos: Some(String::from(r#"{"P":12.8,"Ia":47.1,"Ib":24.2,"Ic":26.1,"In":0.2}"#)),
			num_relig: None,
			prot_atu: None,
			id_cause: None,
			email_sended: None,
			sms_sended: None,
			cause: None,
			observacao: None,
			tipo_oco:None,
			prot_sen: None,
			modified_by:None
		};

		let soe = vec![
			OcorSoe{
				id: 1,
				oco_id: None,
				hora_ini: None,
				hora_fim: None,
				complemento: None,
				mensagem: Some(String::from("mensagem teste 1 soe")),
				actor_id: None
			},
			OcorSoe{
				id: 2,
				oco_id: None,
				hora_ini: None,
				hora_fim: None,
				complemento: None,
				mensagem: Some(String::from("mensagem teste 2 soe")),
				actor_id: None
			},
		];

		let html = build_message(caso, soe).unwrap();
		assert!(f.write_all(html.as_bytes()).is_ok());
	}
}