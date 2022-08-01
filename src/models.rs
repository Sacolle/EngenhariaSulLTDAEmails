#[derive(Queryable)]
pub struct Ocor{
	pub id: i32,
	pub se: Option<String>,
	pub al: Option<String>,
	pub eqp: Option<String>,
	pub hora_ini: chrono::NaiveDateTime,
	pub hora_fim: chrono::NaiveDateTime,
	pub duracao: Option<f32>,
	pub faltas: Option<String>,
	pub condpre: Option<String>,
	pub condpos: Option<String>,
	pub num_relig: Option<i32>,
	pub lockout: Option<String>,
	pub prot_atu: Option<String>,
	pub id_cause:Option<i32>,
	pub email_sended : Option<String>,
	pub sms_sended : Option<String>,
	pub cause: Option<String>,
	pub observacao: Option<String>,
}

#[derive(Queryable)]
pub struct OcorSoe{
	pub id: i32,
	pub oco_id: Option<i32>,
	pub hora_ini: Option<chrono::NaiveDateTime>,
	pub hora_fim: Option<chrono::NaiveDateTime>,
	pub complemento: Option<String>,
	pub mensagem: Option<String>,
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
