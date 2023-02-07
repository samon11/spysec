alter table form
    rename column url to "TxtURL";

alter table form
    add "WebURL" varchar(500) default '' not null;