table! {
    cadastroemails (EmailId) {
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
    ocorrencia (OcoID) {
        OcoID -> Integer,
        SE -> Nullable<Char>,
        AL -> Nullable<Char>,
        EQP -> Nullable<Char>,
        DtHrIni -> Timestamp,
        DtHrFim -> Timestamp,
        Duracao -> Nullable<Float>,
        Faltas -> Nullable<Longtext>,
        CondPre -> Nullable<Longtext>,
        CondPos -> Nullable<Longtext>,
        NRelig -> Nullable<Integer>,
        Lockout -> Nullable<Char>,
        ProtAtu -> Nullable<Char>,
        IdCausa -> Nullable<Integer>,
        EmailSended -> Nullable<Char>,
        SMSSended -> Nullable<Char>,
        Causa -> Nullable<Varchar>,
        Obs -> Nullable<Varchar>,
    }
}

table! {
    ocorrencia_soe (SoeID) {
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
    cadastroemails,
    ocorrencia,
    ocorrencia_soe,
);
