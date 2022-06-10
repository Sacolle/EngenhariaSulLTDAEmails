
use serde::Deserialize;
use serde_json::from_str as JSONparse;

use crate::models::{Ocor,OcorSoe};

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

#[derive(Deserialize)]
struct FaltasTabela{
	fase_a: f32,
	fase_b: f32,
	fase_c: f32,
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


#[derive(Deserialize)]
struct CondPrePosTabela{
	potencia_ativa: f32,
	fase_a: f32,
	fase_b: f32,
	fase_c: f32,
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
			val.hora_ini.unwrap_or(chrono_default()),
			val.mensagem.as_ref().unwrap_or(&String::new()),
			val.hora_fim.unwrap_or(chrono_default()),
			val.actor_id.as_ref().unwrap_or(&String::new()),
		));
	}
	tabela
}

fn make_row(info:&ExtraInfo,line: &str)->String{ 
	format!("<tr><td>{}</td><td>{}</td><td>{}</td><td></td></tr>\n",info.0, line, info.1)
}

fn chrono_default()->chrono::NaiveDateTime{
	chrono::NaiveDate::from_ymd(1,1,1).and_hms(1, 1, 1)
}

pub fn build_message(caso: Ocor,soe: Vec<OcorSoe>)->Result<String, Box<dyn std::error::Error>>{
	let mut result = String::from(HTMLHEAD);
	let info = ExtraInfo::new(&caso);

	let pre_ocor:CondPrePosTabela = JSONparse(caso.condpre.as_ref().unwrap())?;
	let pos_ocor:CondPrePosTabela = JSONparse(caso.condpos.as_ref().unwrap())?;
	let faltas:FaltasTabela = JSONparse(caso.faltas.as_ref().unwrap())?;

	result.push_str(&pre_ocor.to_html(&info));
	result.push_str(&faltas.to_html(&info));
	result.push_str(&parse_soe(soe));
	result.push_str(&pos_ocor.to_html(&info));
	
	result.push_str(HTMLTAIL);

	return Ok(result);
}