alter table form
    add "AccessNo" varchar(500) not null;

drop index form_url_uindex;

create unique index form_accessno_uindex
    on form ("AccessNo");