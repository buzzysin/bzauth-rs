use bzauth_rs::contracts::adapt::{
    Adapt, AdaptAccount, AdaptSession, AdaptUser, AdaptVerificationToken, CreateSessionOptions,
    ProviderAccountId, SessionUser, UseVerificationTokenOptions,
};

use crate::mock::{JsonStore, JsonTableInsertQuery, JsonTableSelectQuery, JsonTableUpdateQuery};

pub struct MockAdaptor {
    store: JsonStore,
}

impl MockAdaptor {
    pub fn new(store: JsonStore) -> Self {
        Self { store }
    }
}

#[async_trait::async_trait]
impl Adapt for MockAdaptor {
    async fn create_user(&self, user: AdaptUser) -> AdaptUser {
        let query = JsonTableInsertQuery::new("users", user);

        let result = query.execute(&self.store);
        let created_user: AdaptUser =
            serde_json::from_value(result.first().unwrap().clone()).unwrap();

        // Return the created user
        created_user
    }

    async fn get_user(&self, id: String) -> Option<AdaptUser> {
        let query = JsonTableSelectQuery::new("users").where_clause("id", id);
        let result = query.execute(&self.store);

        if result.is_empty() {
            None
        } else {
            let user: AdaptUser = serde_json::from_value(result.first().unwrap().clone()).unwrap();
            Some(user)
        }
    }

    async fn get_user_by_email(&self, email: String) -> Option<AdaptUser> {
        let query = JsonTableSelectQuery::new("users").where_clause("email", email);
        let result = query.execute(&self.store);

        if result.is_empty() {
            None
        } else {
            let user: AdaptUser = serde_json::from_value(result.first().unwrap().clone()).unwrap();
            Some(user)
        }
    }

    async fn get_user_by_account(&self, provider: ProviderAccountId) -> Option<AdaptUser> {
        let query = JsonTableSelectQuery::new("accounts")
            .where_clause("provider_id", provider.provider_id)
            .where_clause("provider_account_id", provider.provider_account_id);

        let account = query.execute(&self.store);

        if account.is_empty() {
            None
        } else {
            let user_id = account.first().unwrap()["user_id"]
                .as_str()
                .unwrap()
                .to_string();
            self.get_user(user_id).await
        }
    }

    async fn update_user(&self, user: AdaptUser) -> AdaptUser {
        let user_id = user.id.clone();
        let query = JsonTableUpdateQuery::new("users", user).where_clause("id", user_id);

        let result = query.execute(&self.store);
        let updated_user: AdaptUser =
            serde_json::from_value(result.first().unwrap().clone()).unwrap();

        // Return the updated user
        updated_user
    }

    async fn delete_user(&self, id: String) -> () {
        let query = JsonTableUpdateQuery::new("users", serde_json::json!({"deleted": true}))
            .where_clause("id", id);

        query.execute(&self.store);
        ()
    }

    async fn get_account(&self, provider: ProviderAccountId) -> Option<AdaptAccount> {
        let query = JsonTableSelectQuery::new("accounts")
            .where_clause("provider_id", provider.provider_id)
            .where_clause("provider_account_id", provider.provider_account_id);

        let result = query.execute(&self.store);

        if result.is_empty() {
            None
        } else {
            let account: AdaptAccount =
                serde_json::from_value(result.first().unwrap().clone()).unwrap();
            Some(account)
        }
    }

    async fn link_account(&self, account: AdaptAccount) -> Option<AdaptAccount> {
        let query = JsonTableInsertQuery::new("accounts", account);

        let result = query.execute(&self.store);
        if result.is_empty() {
            None
        } else {
            let linked_account: AdaptAccount =
                serde_json::from_value(result.first().unwrap().clone()).unwrap();
            Some(linked_account)
        }
    }

    async fn unlink_account(&self, provider: ProviderAccountId) -> () {
        let query = JsonTableUpdateQuery::new("accounts", serde_json::json!({"deleted": true}))
            .where_clause("provider_id", provider.provider_id)
            .where_clause("provider_account_id", provider.provider_account_id);

        query.execute(&self.store);
        ()
    }

    async fn create_session(&self, options: CreateSessionOptions) -> Option<AdaptSession> {
        let session = AdaptSession {
            token: options.token,
            user_id: options.user_id,
            expires_in: options.expires_in,
        };

        let query = JsonTableInsertQuery::new("sessions", session);

        let result = query.execute(&self.store);
        if result.is_empty() {
            None
        } else {
            let created_session: AdaptSession =
                serde_json::from_value(result.first().unwrap().clone()).unwrap();
            Some(created_session)
        }
    }

    async fn get_session_and_user(&self, token: String) -> Option<SessionUser> {
        let query = JsonTableSelectQuery::new("sessions").where_clause("token", token);
        let result = query.execute(&self.store);

        if result.is_empty() {
            None
        } else {
            let session: AdaptSession =
                serde_json::from_value(result.first().unwrap().clone()).unwrap();
            let user = self.get_user(session.user_id.clone()).await?;
            let session_user = SessionUser { session, user };
            Some(session_user)
        }
    }

    async fn update_session(&self, session: AdaptSession) -> AdaptSession {
        let session_id = session.token.clone();
        let query =
            JsonTableUpdateQuery::new("sessions", session).where_clause("token", session_id);

        let result = query.execute(&self.store);
        let updated_session: AdaptSession =
            serde_json::from_value(result.first().unwrap().clone()).unwrap();

        // Return the updated session
        updated_session
    }

    async fn delete_session(&self, token: String) -> () {
        let query = JsonTableUpdateQuery::new("sessions", serde_json::json!({"deleted": true}))
            .where_clause("token", token);

        query.execute(&self.store);
    }

    fn create_verification_token(&self, token: AdaptVerificationToken) -> AdaptVerificationToken {
        let query = JsonTableInsertQuery::new("verification_tokens", token);

        let result = query.execute(&self.store);
        let created_token: AdaptVerificationToken =
            serde_json::from_value(result.first().unwrap().clone()).unwrap();

        // Return the created verification token
        created_token
    }

    fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Option<AdaptVerificationToken> {
        // Delete the token from the store
        let query =
            JsonTableUpdateQuery::new("verification_tokens", serde_json::json!({"used": true}))
                .where_clause("token", options.token);

        let result = query.execute(&self.store);
        if result.is_empty() {
            None
        } else {
            let used_token: AdaptVerificationToken =
                serde_json::from_value(result.first().unwrap().clone()).unwrap();
            Some(used_token)
        }
    }
}
