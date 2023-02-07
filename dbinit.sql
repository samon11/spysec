create table if not exists individual
(
    "IndividualId" serial
        constraint individual_pk
            primary key,
    cik            varchar(30)  not null,
    "FullName"     varchar(500) not null,
    "FirstName"    varchar(500),
    "LastName"     varchar(500)
);

alter table individual
    owner to postgres;

create unique index if not exists individual_cik_uindex
    on individual (cik);

create table if not exists issuer
(
    "IssuerId" serial
        constraint issuer_pk
            primary key,
    "Name"     varchar(500) not null,
    "Symbol"   varchar(10)  not null,
    cik        varchar(30)  not null
);

alter table issuer
    owner to postgres;

create unique index if not exists issuer_symbol_cik_uindex
    on issuer ("Symbol", cik);

create table if not exists form
(
    "FormId"       bigint       default nextval('"form_FormId_seq"'::regclass) not null
        constraint form_pk
            primary key,
    "IssuerId"     integer                                                     not null
        constraint form_issuer_issuerid_fk
            references issuer,
    "DateReported" date                                                        not null,
    "FormType"     varchar(10)                                                 not null,
    "TxtURL"       varchar(500)                                                not null,
    "AccessNo"     varchar(500)                                                not null,
    "WebURL"       varchar(500) default ''::character varying                  not null
);

alter table form
    owner to postgres;

create unique index if not exists form_accessno_uindex
    on form ("AccessNo");

create table if not exists non_deriv_transaction
(
    "TransactionId"   bigserial
        constraint non_deriv_transaction_pk
            primary key,
    "DateReported"    date           not null,
    "FormId"          bigint         not null
        constraint non_deriv_transaction_form_formid_fk
            references form,
    "IssuerId"        integer        not null
        constraint non_deriv_transaction_issuer_issuerid_fk
            references issuer,
    "IndividualId"    integer        not null
        constraint non_deriv_transaction_individual_individualid_fk
            references individual,
    "ActionCode"      char,
    "OwnershipCode"   char,
    "TransactionCode" char,
    "SharesBalance"   numeric(20, 3) not null,
    "SharesTraded"    numeric(20, 3) not null,
    "AvgPrice"        numeric(20, 3) not null,
    "Amount"          numeric(20, 3) not null,
    "Relationships"   integer[]      not null
);

alter table non_deriv_transaction
    owner to postgres;

create unique index if not exists non_deriv_transaction_formid_datereported_sharesbalance_uindex
    on non_deriv_transaction ("FormId", "DateReported", "SharesBalance");

create table if not exists __diesel_schema_migrations
(
    version varchar(50)                         not null
        primary key,
    run_on  timestamp default CURRENT_TIMESTAMP not null
);

alter table __diesel_schema_migrations
    owner to postgres;

create table if not exists relationships
(
    "RelationshipId" integer      not null
        constraint relationships_pk
            primary key,
    "Name"           varchar(100) not null
);

alter table relationships
    owner to postgres;

insert into relationships ("RelationshipId", "Name")
values (1, 'OTHER'), (2, 'TEN PERCENT'), (3, 'DIRECTOR'), (4, 'OFFICER');
