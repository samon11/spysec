alter table non_deriv_transaction
    alter column "AvgPrice" set null;

alter table non_deriv_transaction
    alter column "Amount" set null;

alter table non_deriv_transaction
    alter column "Relationships" set null;