## Sistema de Envio de Emails 
---
Programa escrito utilizando a biblioteca Diesel como um ORM
para realizar os queries na Base de Dados Maria Db da empresa.\
O ciclo de execução do programa consiste em: 
- obter na db as empresas em que o programa deve ser executado
- para cada uma delas obter as N instâncias de ocorrências em que o email não foi enviado 
- para cada uma dessas instâncias, obter as SOE e equipamentos anteriores relaicionados a essa ocorrência
- com isso, gera-se o email com a biblioteca de template Tera e o envia usando a lib Lettre
- caso o envio tenha sucesso, faz o update do campo 'EmailSended' das instancias para "S"

Daria pra migrar tudo para async usando sqlx \
mas eu poderia não \
### Na real eu fiz [lol](https://github.com/Sacolle/EngenhariaSulEmailsAsync/blob/main/src/main.rs)