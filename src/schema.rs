// @generated automatically by Diesel CLI.

diesel::table! {
    bridge_tables (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        owner -> Varchar,
        public -> Bool,
    }
}
