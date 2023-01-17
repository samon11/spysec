// @generated automatically by Diesel CLI.

diesel::table! {
    form (FormId) {
        FormId -> Int8,
        IssuerId -> Int4,
        DateReported -> Date,
        FormType -> Varchar,
        url -> Varchar,
    }
}

diesel::table! {
    individual (IndividualId) {
        IndividualId -> Int4,
        cik -> Varchar,
        FullName -> Varchar,
        FirstName -> Nullable<Varchar>,
        LastName -> Nullable<Varchar>,
    }
}

diesel::table! {
    issuer (IssuerId) {
        IssuerId -> Int4,
        Name -> Varchar,
        Symbol -> Varchar,
        cik -> Varchar,
    }
}

diesel::table! {
    non_deriv_transaction (TransactionId) {
        TransactionId -> Int8,
        DateReported -> Date,
        FormId -> Int8,
        IssuerId -> Int4,
        IndividualId -> Int4,
        ActionCode -> Nullable<Bpchar>,
        OwnershipCode -> Nullable<Bpchar>,
        TransactionCode -> Nullable<Bpchar>,
        SharesBalance -> Numeric,
        SharesTraded -> Numeric,
        AvgPrice -> Numeric,
        Amount -> Numeric,
        Relationships -> Array<Int4>,
    }
}

diesel::joinable!(form -> issuer (IssuerId));
diesel::joinable!(non_deriv_transaction -> form (FormId));
diesel::joinable!(non_deriv_transaction -> individual (IndividualId));
diesel::joinable!(non_deriv_transaction -> issuer (IssuerId));

diesel::allow_tables_to_appear_in_same_query!(
    form,
    individual,
    issuer,
    non_deriv_transaction,
);
