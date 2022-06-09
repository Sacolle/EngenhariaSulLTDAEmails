CREATE TABLE Ocorrencia (
	OcoID int NOT NULL AUTO_INCREMENT,
	SE char(8) ,
	AL char(8) ,
	EQP char(10) ,
	DtHrIni TIMESTAMP ,
	DtHrFim TIMESTAMP ,
	Duracao float ,
	Faltas json ,
	CondPre json ,
	CondPos json ,
	NRelig int ,
	Lockout char(1) ,
	ProtAtu char(10) ,
	IdCausa int ,
	EmailSended char(1) ,
	SMSSended char(1) ,
	Causa varchar(100) ,
	Obs varchar(100) ,
	PRIMARY KEY (OcoID)
) ENGINE=INNODB;

CREATE TABLE Ocorrencia_SOE (
	SoeID int NOT NULL AUTO_INCREMENT,
	OcoID int ,
	E3TimeStamp TIMESTAMP NULL,
	EventTime TIMESTAMP NULL,
	Complemento varchar(20) ,
	Mensagem varchar(100) ,
	ActorID char(12) ,
	PRIMARY KEY (SoeID) 
) ENGINE=INNODB;