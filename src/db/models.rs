use diesel::mysql::data_types::MysqlTime;

#[derive(Queryable)]
pub struct Ocor{
	pub id: i32,
	pub se: Option<String>,
	pub al: Option<String>,
	pub eqp: Option<String>,
	pub hora_update: MysqlTime,
	pub hora_oco:    MysqlTime,
	pub hora_ini:    MysqlTime,
	pub hora_fim:    MysqlTime,
	pub num_relig: Option<i32>,
	pub duracao: Option<f32>,
	pub tipo_oco: Option<String>,
	pub prot_sen: Option<String>,
	pub prot_atu: Option<String>,
	pub faltas: Option<String>,
	pub condpre: Option<String>,
	pub condpos: Option<String>,
	//pub lockout: Option<String>,
	pub email_sended : Option<String>,
	pub sms_sended : Option<String>,
	pub id_cause:Option<i32>,
	pub modified_by: Option<String>,
	pub cause: Option<String>,
	pub observacao: Option<String>,
}

#[derive(Queryable)]
pub struct OcorSoe{
	pub id: i32,
	pub oco_id: Option<i32>,
	pub time_stamp: Option<chrono::NaiveDateTime>,
	pub hora_evento: Option<chrono::NaiveDateTime>,
	pub complemento: Option<String>,
	pub mensagem: Option<String>,
	pub tipo: Option<String>,
	pub alarme_norm: Option<String>,
	pub actor_id: Option<String>
}

#[derive(Queryable)]
pub struct Email{
	pub id: i32,
	pub empresa: Option<String>,
	pub email_adrs: Option<String>,
	pub email_name: Option<String>,
	pub env_relig: Option<String>,
	pub env_lockout : Option<String>,
	pub env_normaliz: Option<String>,
}
