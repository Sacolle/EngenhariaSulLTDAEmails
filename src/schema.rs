/* 
* Bom, as tabelas da empresa são capitalizadas (má prática).
* o ORM que estou usando trabalha como se a DB fosse em linux,
* o que tem o nome das tabelas só em minusculo.
* Então é esse negocio meio hack ae mesmo pq é
*/

table! {
	#[allow(non_snake_case)]
    CadastroEmails (EmailId) {
        EmailId -> Integer,
        Empresa -> Nullable<Char>,
        EmailAddr -> Nullable<Char>,
        EmailName -> Nullable<Char>,
        EnvRelig -> Nullable<Char>,
        EnvLockout -> Nullable<Char>,
        EnvNormaliz -> Nullable<Char>,
    }
}

table! {
	#[allow(non_snake_case)]
    Ocorrencia (OcoID) {
        OcoID -> Integer,
        SE -> Nullable<Char>,
        AL -> Nullable<Char>,
        EQP -> Nullable<Char>,
        DtHrAlt -> Timestamp,
        DtHrIni -> Timestamp,
        DtHrFim -> Timestamp,
        DtHrEvtEqp -> Timestamp,
        NRelig -> Nullable<Integer>,
        Duracao -> Nullable<Float>,
		TipoOco -> Nullable<Char>,
        ProtSen -> Nullable<Char>,
        ProtAtu -> Nullable<Char>,
        Faltas -> Nullable<Longtext>,
        CondPre -> Nullable<Longtext>,
        CondPos -> Nullable<Longtext>,
        ///Lockout -> Nullable<Char>,
        EmailSended -> Nullable<Char>,
        SMSSended -> Nullable<Char>,
        IdCausa -> Nullable<Integer>,
		ModifBy -> Nullable<Char>,
        Causa -> Nullable<Varchar>,
        Obs -> Nullable<Varchar>,
    }
}

table! {
	#[allow(non_snake_case)]
    Ocorrencia_SOE (SoeID) {
        SoeID -> Integer,
        OcoID -> Nullable<Integer>,
        E3TimeStamp -> Nullable<Timestamp>,
        EventTime -> Nullable<Timestamp>,
        Complemento -> Nullable<Varchar>,
        Mensagem -> Nullable<Varchar>,
        ActorID -> Nullable<Char>,
    }
}

allow_tables_to_appear_in_same_query!(
    CadastroEmails,
    Ocorrencia,
    Ocorrencia_SOE,
);
