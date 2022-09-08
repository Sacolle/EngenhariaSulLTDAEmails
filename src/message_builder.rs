use crate::db::models::{Ocor,OcorSoe};
use crate::error::TableProcessError;
use crate::db::chunks::{TextInfo,TableInfo,PrevEqp,parse_time};

const HTMLHEAD: &str = r#"
<!DOCTYPE html>
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
	"#;

const HTMLTAIL: &str = r#"
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

const HEADROWEQP: &str = r#"
			<tr class="titulos">
			  <td>Hora do Evento</td>
			  <td>Correntes de Falta</td>
			  <td>Proto Sen</td>
			  <td>Proto Atu</td>
			</tr>
"#;


pub fn build_message(empresa:&str,caso: &Ocor,info: TextInfo ,soe: Vec<OcorSoe>,eqp:Vec<Ocor>)->Result<(String,String),TableProcessError>{
	let table_info = TableInfo::build_from(caso,soe)?;
	let equipamentos:Vec<PrevEqp> = eqp.into_iter()
		.filter_map(|val|PrevEqp::build_from(val).ok())
		.collect();

	let title = format!("{}: {} {} {}",
		&info.tipo,
		&info.subestacao,
		&info.modulo,
		&info.equipamento);

	let message_body = format!("{}{}{}<br>{}{}",
		HTMLHEAD,
		build_head(info,empresa),
		build_table(table_info,caso),
		build_table_eqp(equipamentos),
		HTMLTAIL
		);

	Ok((title,message_body))
}


fn build_head(txt:TextInfo,empresa:&str)->String{
	return format!(r#"
		<p>Prezado Sr(a)</p>
		<p>Voce está recebendo esta mensagem devido a uma ocorrência no sistema elétrico da empresa {}.</p>
		<p style="white-space: pre-line;">Subestação: {}
			Modulo: {}
			Equipamento: {}
		</p>
		<p style="white-space:pre;">Inicio: {}      Termino: {}</p>
		<p>Duração: {}</p>"#,
		empresa,txt.subestacao,txt.modulo,txt.equipamento,txt.inicio,txt.termino,hhmmss(txt.duracao));
}

fn build_table(info:TableInfo,caso:&Ocor)->String{
	let ini = parse_time(caso.hora_ini);
	let fim = parse_time(caso.hora_fim);	

	let pre_ocor = format!(r#"
		<tr>
			<th colspan="4">CONDIÇÃO DE OPERAÇÃO DE PRE-OCORRÊNCIA</th>
		</tr>{}
		<tr><td>{}</td><td>Potência Ativa = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes na fase A = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes na fase B = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes na fase C = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes no Neutro = {}</td><td>{}</td><td></td></tr>"#,
		HEADROW,
		ini, info.cond_pre.potencia_ativa, fim,
		ini, info.cond_pre.fase_a, fim,
		ini, info.cond_pre.fase_b, fim,
		ini, info.cond_pre.fase_c, fim,
		ini, info.cond_pre.fase_n, fim,
	);

	let pos_ocor = format!(r#"
		<tr>
			<th colspan="4">CONDIÇÃO DE OPERAÇÃO DE POS-OCORRÊNCIA</th>
		</tr>{}
		<tr><td>{}</td><td>Potência Ativa = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes na fase A = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes na fase B = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes na fase C = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes no Neutro = {}</td><td>{}</td><td></td></tr>"#,
		HEADROW,
		ini, info.cond_pos.potencia_ativa, fim,
		ini, info.cond_pos.fase_a, fim,
		ini, info.cond_pos.fase_b, fim,
		ini, info.cond_pos.fase_c, fim,
		ini, info.cond_pos.fase_n, fim,
	);
	
	let faltas = format!(r#"
		<tr>
			<th colspan="4">CORRENTES DE FALTA</th>
		</tr>{}
		<tr><td>{}</td><td>Correntes de Falta Fase A = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes de Falta Fase B = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes de Falta Fase C = {}</td><td>{}</td><td></td></tr>
		<tr><td>{}</td><td>Correntes de Falta Fase Neutro = {}</td><td>{}</td><td></td></tr>"#,
		HEADROW,
		ini, info.faltas.fase_a, fim,
		ini, info.faltas.fase_b, fim,
		ini, info.faltas.fase_c, fim,
		ini, info.faltas.fase_n, fim,
	);

	let eventos_iner = info.eventos.into_iter()
		.map(|event| format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
			event.hora_inicio,event.mensagem,event.hora_fim,event.agente))
		.collect::<Vec<String>>()
		.join("\n");

	let eventos = format!(r#"
		<tr>
			<th colspan="4">EVENTOS</th>
		</tr>{}
		{}"#,
		HEADROW,eventos_iner
	);
	
	return format!("<table style=\"width: 1000px;\">\n{}{}{}{}\n</table>",pre_ocor,faltas,eventos,pos_ocor)
			
}
fn build_table_eqp(eqps:Vec<PrevEqp>)->String{
	format!("<table style=\"width: 1000px;\">\n<th colspan=\"4\">Falhas Anteriores Deste Equipamento</th>{}{}\n</table>",
	HEADROWEQP,
	eqps.into_iter()
		.map(|eqp|format!(r#"<tr>
			<td>{}</td>
			<td>A = {} B = {} C = {} Neutro = {}</td>
			<td>{}</td>
			<td>{}</td>
			</tr>"#,
			eqp.inicio,
			eqp.faltas.fase_a,
			eqp.faltas.fase_b,
			eqp.faltas.fase_c,
			eqp.faltas.fase_n,
			eqp.prot_sen,
			eqp.prot_atu))
			.collect::<Vec<String>>()
			.join("\n"))
}

fn hhmmss(secs:f64)->String{
	let min = secs as u32/ 60;
	let hour = min/60;
	let sec = secs%60.0;
	format!("{:0wid$}:{:0wid$}:{:.4}",hour, min%60, sec, wid = 2)
}


#[cfg(test)]
mod tests{
	use super::*;
	use std::{fs,io::Write};
	use crate::db::chunks::*;
	use serde_json::from_str as JSONparse;


	#[test]
	fn hour_convert(){
		let secs = 3701.2f64;
		assert_eq!(hhmmss(secs),String::from("01:01:41.2000"));
	}

	#[test]
	fn proper_json_faltas(){
		let faltas = Some(String::from(r#"{"IaF":1.5,"IbF":1.5,"IcF":1.5,"InF":1.5}"#));
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


	use diesel::mysql::data_types::{MysqlTimestampType,MysqlTime};
	fn time_default()->MysqlTime{
		MysqlTime::new(0, 0, 0,
			0,0,0,0,
			false, MysqlTimestampType::MYSQL_TIMESTAMP_DATETIME,0)
	}

	#[test]
	fn proper_hmtl(){
		let mut f = fs::File::create("./testres/tabela_para_envio.html").unwrap();

		let caso = Ocor {
			id: 1,
			se: Some(String::from("SE")),
			al: Some(String::from("AL")),
			eqp: Some(String::from("EQP")),
			hora_update: time_default(),
			hora_oco:    time_default(),
			hora_ini:    time_default(),
			hora_fim:    time_default(),
			duracao: None,
			faltas: Some(String::from(r#"{"IaF":96.5,"IbF":8.9,"IcF":94.2,"InF":68.5}"#)),
			condpre: Some(String::from(r#"{"P":85.5,"Ia":62,"Ib":9.4,"Ic":22.3,"In":68.8}"#)),
			condpos: Some(String::from(r#"{"P":12.8,"Ia":47.1,"Ib":24.2,"Ic":26.1,"In":0.2}"#)),
			num_relig: None,
			prot_atu: None,
			id_cause: None,
			email_sended: None,
			sms_sended: None,
			cause: None,
			observacao: None,
			tipo_oco:Some(String::from("C")),
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
				tipo : None,
				alarme_norm: None,
				actor_id: None
			},
			OcorSoe{
				id: 2,
				oco_id: None,
				hora_ini: None,
				hora_fim: None,
				complemento: None,
				mensagem: Some(String::from("mensagem teste 2 soe")),
				tipo : None,
				alarme_norm: None,
				actor_id: None
			},
		];
		let eqp = vec![
			Ocor{
				id: 1,
				se: Some(String::from("SE")),
				al: Some(String::from("AL")),
				eqp: Some(String::from("EQP")),
				hora_update: time_default(),
				hora_oco:    time_default(),
				hora_ini:    time_default(),
				hora_fim:    time_default(),
				duracao: None,
				faltas: Some(String::from(r#"{"IaF":96.5,"IbF":8.9,"IcF":94.2,"InF":68.5}"#)),
				condpre: Some(String::from(r#"{"P":85.5,"Ia":62,"Ib":9.4,"Ic":22.3,"In":68.8}"#)),
				condpos: Some(String::from(r#"{"P":12.8,"Ia":47.1,"Ib":24.2,"Ic":26.1,"In":0.2}"#)),
				num_relig: None,
				prot_atu: None,
				id_cause: None,
				email_sended: None,
				sms_sended: None,
				cause: None,
				observacao: None,
				tipo_oco:Some(String::from("C")),
				prot_sen: None,
				modified_by:None
			}
		];

		let info = TextInfo::build_from(&caso).unwrap();

		let (_titulo,html) = build_message("EMPRESA TESTE",&caso, info,soe,eqp).unwrap();
		assert!(f.write_all(html.as_bytes()).is_ok());
	}
}