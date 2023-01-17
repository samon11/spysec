alter table form
    add URL varchar(500) not null;

drop index form_companyid_formtype_datereported_uindex;

create unique index form_url_uindex
    on form (URL);