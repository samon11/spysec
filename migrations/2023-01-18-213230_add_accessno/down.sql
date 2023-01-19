drop index form_accessno_uindex;

alter table form
    drop "AccessNo";

create unique index form_url_uindex
    on form (url);