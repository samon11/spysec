alter table non_deriv_transaction
    alter column "AvgPrice" set not null;

alter table non_deriv_transaction
    alter column "Amount" set not null;

alter table non_deriv_transaction
    alter column "Relationships" set not null;