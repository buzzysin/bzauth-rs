pub trait AdaptUserOperation<C>
where
    Self: Send + Sync + 'static,
{
    type Model;
    fn create_user(&self, conn: &mut C, user: &Self::Model) -> Self::Model;
    fn find_user_by_id(&self, conn: &mut C, id: &str) -> Option<Self::Model>;
    fn find_user_by_email(&self, conn: &mut C, email: &str) -> Option<Self::Model>;
    fn update_user(&self, conn: &mut C, user: &Self::Model) -> Self::Model;
    fn delete_user(&self, conn: &mut C, id: &str);
}

#[macro_export]
macro_rules! adapt_diesel_user {
    ($table_struct:ident, $connection:ident, Model = $model_type:path, Table = $table_type:path) => {
        impl From<$model_type> for $crate::contracts::adapt::AdaptUser {
            fn from(user: $model_type) -> Self {
                $crate::contracts::User {
                    id: Some(user.id),
                    username: user.name,
                    email: user.email,
                    image: user.image,
                }
                .into()
            }
        }

        impl From<$crate::contracts::adapt::AdaptUser> for $model_type {
            fn from(user: $crate::contracts::adapt::AdaptUser) -> Self {
                let user: $crate::contracts::User = user.into();

                $model_type {
                    name: user.username,
                    email: user.email,
                    image: user.image,
                    ..Default::default()
                }
            }
        }

        impl $crate::adaptors::diesel_traits::AdaptUserOperation<$connection> for $table_struct {
            type Model = $model_type;
            fn create_user(&self, conn: &mut $connection, user: &Self::Model) -> Self::Model {
                // Create a user using the connection
                use diesel::ExpressionMethods;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_insert = (
                    email.eq(user.email.clone()),
                    image.eq(user.image.clone()),
                    name.eq(user.name.clone()),
                    email_verified.eq(user.email_verified.clone()),
                    created_at.eq(now),
                    updated_at.eq(now),
                );

                let user = diesel::insert_into(paste::paste!($table_type::table))
                    .values(&to_insert)
                    .returning(paste::paste!($model_type::as_returning()))
                    .on_conflict(paste::paste!($table_type::email))
                    .do_nothing()
                    .get_result(conn)
                    .unwrap();

                user
            }

            fn find_user_by_id(&self, conn: &mut $connection, id: &str) -> Option<Self::Model> {
                // Find a user by ID using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let user = paste::paste!($table_type::table)
                    .filter(id.eq(id))
                    .first::<Self::Model>(conn)
                    .ok();

                user
            }

            fn find_user_by_email(
                &self,
                conn: &mut $connection,
                email: &str,
            ) -> Option<Self::Model> {
                // Find a user by email using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let user = paste::paste!($table_type::table)
                    .filter(email.eq(email))
                    .first::<Self::Model>(conn)
                    .ok();

                user
            }
            fn update_user(&self, conn: &mut $connection, user: &Self::Model) -> Self::Model {
                // Update a user using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_update = (
                    email.eq(user.email.clone()),
                    image.eq(user.image.clone()),
                    name.eq(user.name.clone()),
                    email_verified.eq(user.email_verified.clone()),
                    updated_at.eq(now),
                );

                let user = diesel::update(paste::paste!($table_type::table))
                    .filter(id.eq(user.id.clone()))
                    .set(to_update)
                    .get_result(conn)
                    .unwrap();

                user
            }
            fn delete_user(&self, conn: &mut $connection, id: &str) {
                // Delete a user using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                diesel::delete(paste::paste!($table_type::table))
                    .filter(id.eq(id))
                    .execute(conn)
                    .ok();
            }
        }
    };
}

pub trait AdaptAccountOperation<C>
where
    Self: Send + Sync + 'static,
{
    type Model;
    type User;
    fn create_account(&self, conn: &mut C, account: &Self::Model) -> Self::Model;
    fn link_account(&self, conn: &mut C, account: &Self::Model) -> Self::Model;
    fn unlink_account(&self, conn: &mut C, provider_id: String, provider_account_id: String);
    fn find_user_by_account(
        &self,
        conn: &mut C,
        provider_id: String,
        provider_account_id: String,
    ) -> Option<(Self::Model, Self::User)>;
    fn find_account_by_id(
        &self,
        conn: &mut C,
        provider_id: String,
        provider_account_id: String,
    ) -> Option<Self::Model>;
}

#[macro_export]
macro_rules! adapt_diesel_account {
    ($table_struct:ident, $connection:ident, Model = $model_type:path, User = $user_struct:path, Table = $table_type:path, UserTable = $user_table_type:path) => {
        impl From<$model_type> for $crate::contracts::adapt::AdaptAccount {
            fn from(account: $model_type) -> Self {
                #[allow(unreachable_code)]
                $crate::contracts::Account {
                    id: account.id.into(),
                    user_id: account.user_id.into(),
                    provider_id: account.provider_id.into(),
                    // session_state: account.session_state.into(),
                    token: $crate::contracts::Token {
                        access_token: account.access_token.into(),
                        token_type: account.token_type.into(),
                        refresh_token: account.refresh_token.into(),
                        expires_in: {
                            let now = chrono::Utc::now().naive_utc();
                            let expires_at = account.expires_at.unwrap_or(now);
                            let expires_in = (expires_at - now).num_seconds();
                            Some(expires_in as u64)
                        },
                        scope: account.scope.into(),
                        id_token: account.id_token.into(),
                        others: {
                            let mut others = std::collections::HashMap::<String, String>::new();
                            if let Some(session_state) = account.session_state {
                                others.insert("session_state".to_string(), session_state);
                            }
                            others
                        },
                    }
                    .into(),
                }
                .into()
            }
        }

        impl From<$crate::contracts::adapt::AdaptAccount> for $model_type {
            fn from(account: $crate::contracts::adapt::AdaptAccount) -> Self {
                let account: $crate::contracts::Account = account.into();
                let token = account.token.clone().unwrap();
                $model_type {
                    id: account.id.unwrap(),
                    user_id: account.user_id.unwrap(),
                    provider_id: account.provider_id.unwrap(),
                    access_token: token.access_token,
                    refresh_token: token.refresh_token,
                    expires_at: {
                        let now = chrono::Utc::now().naive_utc();
                        let expires_at = token.expires_in.unwrap_or(0);
                        let expires_at = now + chrono::Duration::seconds(expires_at as i64);
                        Some(expires_at)
                    },
                    token_type: token.token_type,
                    scope: token.scope,
                    id_token: token.id_token,
                    session_state: {
                        let mut session_state = String::new();
                        if let Some(mut session_state) = token.others.get("session_state") {
                            session_state = &session_state.clone();
                        }
                        Some(session_state)
                    },
                    ..Default::default()
                }
            }
        }

        impl $crate::adaptors::diesel_traits::AdaptAccountOperation<$connection> for $table_struct {
            type Model = $model_type;
            type User = $user_struct;

            fn create_account(&self, conn: &mut $connection, account: &Self::Model) -> Self::Model {
                // Create an account using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_insert = (
                    user_id.eq(account.user_id.clone()),
                    access_token.eq(account.access_token.clone()),
                    token_type.eq(account.token_type.clone()),
                    scope.eq(account.scope.clone()),
                    id_token.eq(account.id_token.clone()),
                    session_state.eq(account.session_state.clone()),
                    created_at.eq(now),
                    updated_at.eq(now),
                );

                let account = diesel::insert_into(paste::paste!($table_type::table))
                    .values(&to_insert)
                    .returning(paste::paste!($model_type::as_returning()))
                    .on_conflict(paste::paste!($table_type::user_id))
                    .do_nothing()
                    .get_result(conn)
                    .unwrap();

                account
            }

            fn link_account(&self, conn: &mut $connection, account: &Self::Model) -> Self::Model {
                // Link (create) an account using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_insert = (
                    user_id.eq(account.user_id.clone()),
                    access_token.eq(account.access_token.clone()),
                    token_type.eq(account.token_type.clone()),
                    scope.eq(account.scope.clone()),
                    id_token.eq(account.id_token.clone()),
                    session_state.eq(account.session_state.clone()),
                    created_at.eq(now),
                    updated_at.eq(now),
                );

                let account = diesel::insert_into(paste::paste!($table_type::table))
                    .values(&to_insert)
                    .returning(paste::paste!($model_type::as_returning()))
                    .on_conflict(paste::paste!($table_type::user_id))
                    .do_nothing()
                    .get_result(conn)
                    .unwrap();

                account
            }

            fn unlink_account(
                &self,
                conn: &mut $connection,
                provider_id: String,
                provider_account_id: String,
            ) {
                // Unlink (delete) an account using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                diesel::delete(paste::paste!($table_type::table))
                    .filter(provider_id.eq(provider_id))
                    .filter(provider_account_id.eq(provider_account_id))
                    .execute(conn)
                    .ok();
            }

            fn find_user_by_account(
                &self,
                conn: &mut $connection,
                provider_id: String,
                provider_account_id: String,
            ) -> Option<(Self::Model, Self::User)> {
                // Find a user by account using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let account_user = paste::paste!($table_type::table)
                    .filter(provider_id.eq(provider_id))
                    .filter(provider_account_id.eq(provider_account_id))
                    .inner_join(paste::paste!($user_table_type::table))
                    .select((
                        paste::paste!($model_type::as_returning()),
                        paste::paste!($user_struct::as_returning()),
                    ))
                    .first::<(Self::Model, Self::User)>(conn)
                    .ok();

                account_user
            }

            fn find_account_by_id(
                &self,
                conn: &mut $connection,
                provider_id: String,
                provider_account_id: String,
            ) -> Option<Self::Model> {
                // Find an account by ID using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let account = paste::paste!($table_type::table)
                    .filter(provider_id.eq(provider_id))
                    .filter(provider_account_id.eq(provider_account_id))
                    .first::<Self::Model>(conn)
                    .ok();

                account
            }
        }
    };
}

pub trait AdaptSessionOperation<C>
where
    Self: Send + Sync + 'static,
{
    type Model;
    type User;
    fn create_session(&self, conn: &mut C, user: Self::Model) -> Self::Model;
    fn update_session(&self, conn: &mut C, user: Self::Model) -> Self::Model;
    fn find_session_and_user(&self, conn: &mut C, token: &str)
    -> Option<(Self::Model, Self::User)>;
    fn delete_session(&self, conn: &mut C, token: &str);
}

#[macro_export]
macro_rules! adapt_diesel_session {
    ($table_struct:ident, $connection:ident, Model = $model_type:path, User = $user_struct:path, Table = $table_type:path, UserTable = $user_table_type:path) => {
        impl From<$model_type> for $crate::contracts::adapt::AdaptSession {
            fn from(session: $model_type) -> Self {
                $crate::contracts::adapt::AdaptSession {
                    user_id: session.user_id,
                    token: session.token,
                    expires_in: {
                        let now = chrono::Utc::now().naive_utc();
                        let expires_at = session.expires_at;
                        let expires_in = (expires_at - now).num_seconds();
                        expires_in as u64
                    },
                }
            }
        }

        impl From<$crate::contracts::adapt::AdaptSession> for $model_type {
            fn from(session: $crate::contracts::adapt::AdaptSession) -> Self {
                $model_type {
                    user_id: session.user_id,
                    token: session.token,
                    expires_at: {
                        let now = chrono::Utc::now().naive_utc();
                        let expires_in = session.expires_in;
                        let expires_at = now + chrono::Duration::seconds(expires_in as i64);
                        expires_at
                    },
                    ..Default::default()
                }
            }
        }

        impl $crate::adaptors::diesel_traits::AdaptSessionOperation<$connection> for $table_struct {
            type Model = $model_type;
            type User = $user_struct;
            fn create_session(&self, conn: &mut $connection, session: Self::Model) -> Self::Model {
                // Create a session using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_insert = (
                    user_id.eq(session.user_id.clone()),
                    token.eq(session.token.clone()),
                    expires_at.eq(session.expires_at.clone()),
                    created_at.eq(now),
                    updated_at.eq(now),
                );

                let session = diesel::insert_into(paste::paste!($table_type::table))
                    .values(&to_insert)
                    .returning(paste::paste!($model_type::as_returning()))
                    .on_conflict(paste::paste!($table_type::user_id))
                    .do_nothing()
                    .get_result(conn)
                    .unwrap();

                session
            }

            fn update_session(&self, conn: &mut $connection, session: Self::Model) -> Self::Model {
                // Update a session using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_update = (
                    user_id.eq(session.user_id.clone()),
                    token.eq(session.token.clone()),
                    expires_at.eq(session.expires_at.clone()),
                    updated_at.eq(now),
                );

                let session = diesel::update(paste::paste!($table_type::table))
                    .filter(token.eq(session.token.clone()))
                    .set(to_update)
                    .get_result(conn)
                    .unwrap();

                session
            }
            fn find_session_and_user(
                &self,
                conn: &mut $connection,
                token: &str,
            ) -> Option<(Self::Model, Self::User)> {
                // Find a session and user using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }
                let session_user = paste::paste!($table_type::table)
                    .filter(token.eq(token))
                    .inner_join(paste::paste!($user_table_type::table))
                    .select((
                        paste::paste!($model_type::as_returning()),
                        paste::paste!($user_struct::as_returning()),
                    ))
                    .first::<(Self::Model, Self::User)>(conn)
                    .ok();

                session_user
            }

            fn delete_session(&self, conn: &mut $connection, token: &str) {
                // Delete a session using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                diesel::delete(paste::paste!($table_type::table))
                    .filter(token.eq(token))
                    .execute(conn)
                    .ok();
            }
        }
    };
}

pub trait AdaptVerificationTokenOperation<C>
where
    Self: Send + Sync + 'static,
{
    type Model;
    fn create_verification_token(&self, conn: &mut C, token: Self::Model) -> Self::Model;
    fn use_verification_token(&self, conn: &mut C, email: &str, token: &str);
}

#[macro_export]
macro_rules! adapt_diesel_verification_token {
    ($table_struct:ident, $connection:ident, Model = $model_type:path, Table = $table_type:path) => {
        impl From<$model_type> for $crate::contracts::adapt::AdaptVerificationToken {
            fn from(token: $model_type) -> Self {
                $crate::contracts::adapt::AdaptVerificationToken {
                    email: token.email,
                    token: token.token,
                    expires_in: {
                        let now = chrono::Utc::now().naive_utc();
                        let expires_at = token.expires_at;
                        let expires_in = (expires_at - now).num_seconds();
                        expires_in as u64
                    },
                }
                .into()
            }
        }

        impl From<$crate::contracts::adapt::AdaptVerificationToken> for $model_type {
            fn from(token: $crate::contracts::adapt::AdaptVerificationToken) -> Self {
                $model_type {
                    email: token.email,
                    token: token.token,
                    expires_at: {
                        let now = chrono::Utc::now().naive_utc();
                        let expires_in = token.expires_in;
                        let expires_at = now + chrono::Duration::seconds(expires_in as i64);
                        expires_at
                    },
                    ..Default::default()
                }
            }
        }

        impl $crate::adaptors::diesel_traits::AdaptVerificationTokenOperation<$connection>
            for $table_struct
        {
            type Model = $model_type;
            fn create_verification_token(
                &self,
                conn: &mut $connection,
                verification_token: Self::Model,
            ) -> Self::Model {
                // Create a verification token using the connection
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }

                let now = chrono::Utc::now().naive_utc();

                let to_insert = (
                    email.eq(verification_token.email.clone()),
                    token.eq(verification_token.token.clone()),
                    expires_at.eq(verification_token.expires_at.clone()),
                    created_at.eq(now),
                );

                let verification_token = diesel::insert_into(paste::paste!($table_type::table))
                    .values(&to_insert)
                    .returning(paste::paste!($model_type::as_returning()))
                    .on_conflict(paste::paste!($table_type::email))
                    .do_nothing()
                    .get_result(conn)
                    .unwrap();

                verification_token
            }
            fn use_verification_token(&self, conn: &mut $connection, email: &str, token: &str) {
                // Use a verification token using the connection
                // (Using a verification token means deleting it)
                use diesel::ExpressionMethods;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;
                use diesel::SelectableHelper;
                paste::paste! {
                    use $table_type::dsl::*;
                }
                let now = chrono::Utc::now().naive_utc();
                let to_delete = (email.eq(email), token.eq(token));
                diesel::delete(paste::paste!($table_type::table))
                    .filter(email.eq(email))
                    .filter(token.eq(token))
                    .execute(conn)
                    .ok();
            }
        }
    };
}

#[macro_export]
macro_rules! adapt_diesel {
    (
        $adaptor:ident,
        $connection:ident,
        User = $user_struct:path,
        UserTable = $user_table:path,
        Account = $account_struct:path,
        AccountTable = $account_table:path,
        Session = $session_struct:path,
        SessionTable = $session_table:path,
        VerificationToken = $verification_token_struct:path,
        VerificationTokenTable = $verification_token_table:path,
    ) => {
        $crate::adapt_diesel_user!(
            $adaptor,
            $connection,
            Model = $user_struct,
            Table = $user_table
        );
        $crate::adapt_diesel_account!(
            $adaptor,
            $connection,
            Model = $account_struct,
            User = $user_struct,
            Table = $account_table,
            UserTable = $user_table
        );
        $crate::adapt_diesel_session!(
            $adaptor,
            $connection,
            Model = $session_struct,
            User = $user_struct,
            Table = $session_table,
            UserTable = $user_table
        );
        $crate::adapt_diesel_verification_token!(
            $adaptor,
            $connection,
            Model = $verification_token_struct,
            Table = $verification_token_table
        );
    };
}
