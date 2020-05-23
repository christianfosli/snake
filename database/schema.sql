if not exists (select *
from sys.databases
where name = N'snake-hs')
create database [snake-hs];

use [snake-hs];

if not exists (select *
from sysobjects
where id = object_id(N'[dbo].[highscores]') and objectproperty(id, N'IsUserTable') = 1)
create table [dbo].[highscores]
(
    [UserName] [nvarchar](100) not null,
    [Score] [int] not null,
    [TimeStamp] [datetimeoffset] not null
);
