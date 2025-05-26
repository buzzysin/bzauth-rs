pub mod schema {
    diesel::table! {
        users {
            id -> Text,
            name -> Nullable<Text>,
            email -> Nullable<Text>,
            email_verified -> Nullable<Timestamp>,
            image -> Nullable<Text>,

            // Timestamps
            created_at -> Timestamp,
            updated_at -> Timestamp,

            // Relations:
            // accounts -> Account[]
            // sessions -> Session[]
        }
    }

    diesel::table! {
        // Note - diesel does not support unique constraints on fields when the
        // migration is generated - users will have to add the constraint themselves
        // and handle the error in their application
        accounts {
            id -> Text,
            // Composite key
            provider_id -> Text,
            provider_account_id -> Text,

            // Other keys
            user_id -> Text,
            provider_type -> Text,

            // Token fields
            refresh_token -> Nullable<Text>,
            access_token -> Nullable<Text>,
            expires_at -> Nullable<Timestamp>,
            token_type -> Nullable<Text>,
            scope -> Nullable<Text>,
            id_token -> Nullable<Text>,
            session_state -> Nullable<Text>,

            // Timestamps
            created_at -> Timestamp,
            updated_at -> Timestamp,

            // Relations:
            // user -> User
        }
    }

    diesel::table! {
        sessions {
            id -> Text,
            user_id -> Text,
            token -> Text,
            expires_at -> Timestamp,
            created_at -> Timestamp,
            updated_at -> Timestamp,

            // Relations:
            // user -> User
        }
    }

    diesel::table! {
        verification_tokens {
            id -> Text,
            email -> Text,
            token -> Text,
            expires_at -> Timestamp,
            created_at -> Timestamp,
            updated_at -> Timestamp,

            // Relations:
            // user -> User (by email)
        }
    }

    // TODO: WebAuthn?

    // Relations:
    diesel::joinable!(accounts -> users (user_id));
    diesel::joinable!(sessions -> users (user_id));
    diesel::joinable!(verification_tokens -> users (email));

    diesel::allow_tables_to_appear_in_same_query!(users, accounts, sessions, verification_tokens);
}

pub mod models {
    use diesel::prelude::{Associations, Insertable, Queryable, Selectable};

    #[derive(Clone, Default, Queryable, Selectable)]
    #[diesel(table_name = crate::schema::users)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct User {
        #[diesel(skip_insertion)]
        pub id: String,
        pub name: Option<String>,
        pub email: Option<String>,
        pub email_verified: Option<chrono::NaiveDateTime>,
        pub image: Option<String>,
        pub created_at: chrono::NaiveDateTime,
        pub updated_at: chrono::NaiveDateTime,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::users)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct UserCreate {
        pub name: Option<String>,
        pub email: Option<String>,
        pub email_verified: Option<chrono::NaiveDateTime>,
        pub image: Option<String>,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
    }

    #[derive(Clone, Default, Queryable, Selectable, Associations)]
    #[diesel(table_name = crate::schema::accounts)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(belongs_to(User, foreign_key = user_id))]
    pub struct Account {
        #[diesel(skip_insertion)]
        pub id: String,
        pub provider_id: String,
        pub provider_account_id: String,
        pub user_id: String,
        pub provider_type: String,
        pub refresh_token: Option<String>,
        pub access_token: Option<String>,
        pub expires_at: Option<chrono::NaiveDateTime>,
        pub token_type: Option<String>,
        pub scope: Option<String>,
        pub id_token: Option<String>,
        pub session_state: Option<String>,
        pub created_at: chrono::NaiveDateTime,
        pub updated_at: chrono::NaiveDateTime,
    }

    #[derive(Clone, Insertable)]
    #[diesel(table_name = crate::schema::accounts)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct AccountCreate {
        pub provider_id: String,
        pub provider_account_id: String,
        pub user_id: String,
        pub provider_type: String,
        pub refresh_token: Option<String>,
        pub access_token: Option<String>,
        pub expires_at: Option<chrono::NaiveDateTime>,
        pub token_type: Option<String>,
        pub scope: Option<String>,
        pub id_token: Option<String>,
        pub session_state: Option<String>,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
    }

    #[derive(Clone, Default, Queryable, Selectable, Associations)]
    #[diesel(table_name = crate::schema::sessions)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(belongs_to(User, foreign_key = user_id))]
    pub struct Session {
        #[diesel(skip_insertion)]
        pub id: String,
        pub user_id: String,
        pub token: String,
        pub expires_at: chrono::NaiveDateTime,
        pub created_at: chrono::NaiveDateTime,
        pub updated_at: chrono::NaiveDateTime,
    }

    #[derive(Clone, Insertable)]
    #[diesel(table_name = crate::schema::sessions)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct SessionCreate {
        pub user_id: String,
        pub token: String,
        pub expires_at: chrono::NaiveDateTime,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
    }

    #[derive(Clone, Default, Queryable, Selectable, Associations)]
    #[diesel(table_name = crate::schema::verification_tokens)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(belongs_to(User, foreign_key = email))]
    pub struct VerificationToken {
        #[diesel(skip_insertion)]
        pub id: String,
        pub email: String,
        pub token: String,
        pub expires_at: chrono::NaiveDateTime,
    }

    #[derive(Clone, Insertable)]
    #[diesel(table_name = crate::schema::verification_tokens)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct VerificationTokenCreate {
        pub email: String,
        pub token: String,
        pub expires_at: chrono::NaiveDateTime,
    }
}

use bzauth_rs::adaptors::diesel::DieselAdaptor;
use bzauth_rs::auth::AuthOptions;
use bzauth_rs::{adaptors::diesel::DieselAdapterOptions, providers::discord::DiscordProvider};
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};

pub struct MyDieselAdaptor;
bzauth_rs::adapt_diesel! {
    MyDieselAdaptor,
    SqliteConnection,
    User = crate::models::User,
    UserTable = crate::schema::users,
    Account = crate::models::Account,
    AccountTable = crate::schema::accounts,
    Session = crate::models::Session,
    SessionTable = crate::schema::sessions,
    VerificationToken = crate::models::VerificationToken,
    VerificationTokenTable = crate::schema::verification_tokens,
}

#[tokio::main]
async fn main() {
    use axum::{Extension, Router};
    use bzauth_rs::{
        auth::AuthSessionOptions,
        providers::GoogleProvider,
        runtimes::axum::runtime::{AxumRuntime, AxumRuntimeOptions},
    };
    use tokio::net::TcpListener;

    unsafe {
        std::env::set_var("DISCORD_CLIENT_ID", "YOUR_DISCORD_CLIENT_ID");
        std::env::set_var("DISCORD_CLIENT_SECRET", "YOUR_DISCORD_CLIENT_SECRET");

        std::env::set_var("GOOGLE_CLIENT_ID", "YOUR_GOOGLE_CLIENT_ID");
        std::env::set_var("GOOGLE_CLIENT_SECRET", "YOUR_GOOGLE_CLIENT_SECRET");
    }

    let diesel_conn_manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let diesel_conn_pool =
        Pool::new(diesel_conn_manager).expect("Failed to create connection pool");

    let auth_options = AuthOptions {
        adaptor: Some(
            DieselAdaptor::from_options(DieselAdapterOptions {
                conn_pool: diesel_conn_pool,
                adaptor: MyDieselAdaptor,
            })
            .into(),
        ),
        providers: vec![
            DiscordProvider::new().into(), // Discord provider
            GoogleProvider::new().into(),  // Google provider
        ],
        callbacks: None,
        session: AuthSessionOptions {
            strategy: Some("database".to_string()),
            ..Default::default()
        }
        .into(),
    };
    let AxumRuntime { routes, auth } =
        AxumRuntime::from_options(AxumRuntimeOptions { auth_options });

    let app = Router::new().merge(routes).layer(Extension(auth));
    let app_listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    // Test the routes
    let handle = tokio::spawn(async {
        axum::serve(app_listener, app.into_make_service())
            .await
            .expect("Failed to start server");
    });

    // Test the providers endpoint
    // todo: Implement the test for the providers endpoint here.

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    handle.abort();
    handle.await.unwrap_err(); // Ensure the server was aborted
}
