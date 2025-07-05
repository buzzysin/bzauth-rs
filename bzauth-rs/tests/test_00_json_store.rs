mod mock;

use crate::mock::{JsonStore, JsonStoreType};

const SEED_DATA: &str = r#"
{
  "users": [
      {"id": 1, "name": "Alice", "email": "alice@email.com"},
      {"id": 2, "name": "Bob", "email": "bob@email.com"}
  ],
  "accounts": [
      {"id": 1, "user_id": 1, "balance": 100.0},
      {"id": 2, "user_id": 2, "balance": 200.0}
  ]
}"#;

fn create_store(kind: &JsonStoreType) -> JsonStore {
    // Clear existing data before seeding
    let store = JsonStore::new(kind);
    let _ = store.clear();

    // Parse the seed data and set it in the store
    let seed_value = serde_json::from_str(SEED_DATA).expect("Failed to parse seed data");
    store
        .set_data(seed_value)
        .expect("Failed to parse seed data");

    // Return the store with seeded data
    store
}

mod memory {
    use super::create_store;
    use crate::mock::{
        JsonStoreTypes, JsonTableDeleteQuery, JsonTableInsertQuery, JsonTableSelectQuery,
        JsonTableUpdateQuery,
    };

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_select() {
        let store = create_store(&JsonStoreTypes::Memory);

        let query = JsonTableSelectQuery::new("users");
        let users_result = query.execute(&store);
        assert_eq!(users_result.len(), 2);
        assert_eq!(users_result[0]["id"], 1);
        assert_eq!(users_result[0]["name"], "Alice");
        assert_eq!(users_result[0]["email"], "alice@email.com");
        assert_eq!(users_result[1]["id"], 2);
        assert_eq!(users_result[1]["name"], "Bob");
        assert_eq!(users_result[1]["email"], "bob@email.com");

        // Verify the accounts data
        let accounts_query = JsonTableSelectQuery::new("accounts");
        let accounts_result = accounts_query.execute(&store);
        assert_eq!(accounts_result.len(), 2);
        assert_eq!(accounts_result[0]["id"], 1);
        assert_eq!(accounts_result[0]["user_id"], 1);
        assert_eq!(accounts_result[0]["balance"], 100.0);
        assert_eq!(accounts_result[1]["id"], 2);
        assert_eq!(accounts_result[1]["user_id"], 2);
        assert_eq!(accounts_result[1]["balance"], 200.0);
    }

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_insert() {
        let store = super::create_store(&JsonStoreTypes::Memory);

        let value = serde_json::json!({
            "id": 3,
            "name": "Charlie",
            "email": "charlie@email.com"
        });

        let query = JsonTableInsertQuery::new("users", value);

        let result = query.execute(&store);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["id"], 3);
        assert_eq!(result[0]["name"], "Charlie");
        assert_eq!(result[0]["email"], "charlie@email.com");

        // Verify the inserted data
        let select_query = JsonTableSelectQuery::new("users");
        let select_result = select_query.execute(&store);
        assert_eq!(select_result.len(), 3); // Should include the new entry
        assert!(select_result.iter().any(|user| {
            user["id"] == 3 && user["name"] == "Charlie" && user["email"] == "charlie@email.com"
        }));
    }

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_update() {
        let store = super::create_store(&JsonStoreTypes::Memory);

        let values = serde_json::json!({
            "email": "charlie@newdomain.com"
        });
        let query = JsonTableUpdateQuery::new("users", values).where_clause("id", 1);
        let result = query.execute(&store);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["id"], 1);
        assert_eq!(result[0]["name"], "Alice");
        assert_eq!(result[0]["email"], "charlie@newdomain.com");
    }

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_delete() {
        let store = super::create_store(&JsonStoreTypes::Memory);

        let query = JsonTableDeleteQuery::new("users").where_clause("id", 2);
        let result = query.execute(&store);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["id"], 2);
        assert_eq!(result[0]["name"], "Bob");
        assert_eq!(result[0]["email"], "bob@email.com");

        let select_query = JsonTableSelectQuery::new("users");
        let select_result = select_query.execute(&store);
        assert_eq!(select_result.len(), 1);
        assert!(select_result.iter().any(|user| {
            user["id"] == 1 && user["name"] == "Alice" && user["email"] == "alice@email.com"
        }));
        assert!(!select_result.iter().any(|user| {
            user["id"] == 2 && user["name"] == "Bob" && user["email"] == "bob@email.com"
        }));
    }
}

mod filesystem {
    use tempfile::NamedTempFile;

    use super::create_store;
    use crate::mock::{
        JsonStoreTypes, JsonTableDeleteQuery, JsonTableInsertQuery, JsonTableSelectQuery,
        JsonTableUpdateQuery,
    };

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_select() {
        let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
        let path = tmpfile.path();

        let store = create_store(&JsonStoreTypes::File(path));

        let query = JsonTableSelectQuery::new("users");
        let users_result = query.execute(&store);
        assert_eq!(users_result.len(), 2);
        assert_eq!(users_result[0]["id"], 1);
        assert_eq!(users_result[0]["name"], "Alice");
        assert_eq!(users_result[0]["email"], "alice@email.com");
        assert_eq!(users_result[1]["id"], 2);
        assert_eq!(users_result[1]["name"], "Bob");
        assert_eq!(users_result[1]["email"], "bob@email.com");

        // Verify the accounts data
        let accounts_query = JsonTableSelectQuery::new("accounts");
        let accounts_result = accounts_query.execute(&store);
        assert_eq!(accounts_result.len(), 2);
        assert_eq!(accounts_result[0]["id"], 1);
        assert_eq!(accounts_result[0]["user_id"], 1);
        assert_eq!(accounts_result[0]["balance"], 100.0);
        assert_eq!(accounts_result[1]["id"], 2);
        assert_eq!(accounts_result[1]["user_id"], 2);
        assert_eq!(accounts_result[1]["balance"], 200.0);

        // Pretty print the contents of the store
        println!("Store contents after select: {:?}", users_result);
        println!("Accounts contents after select: {:?}", accounts_result);
    }

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_insert() {
        let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
        let path = tmpfile.path();

        let store = create_store(&JsonStoreTypes::File(path));

        let value = serde_json::json!({
            "id": 3,
            "name": "Charlie",
            "email": "charlie@email.com"
        });
        let query = JsonTableInsertQuery::new("users", value);
        let result = query.execute(&store);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["id"], 3);
        assert_eq!(result[0]["name"], "Charlie");
        assert_eq!(result[0]["email"], "charlie@email.com");
        // Verify the inserted data
        let select_query = JsonTableSelectQuery::new("users");
        let select_result = select_query.execute(&store);
        assert_eq!(select_result.len(), 3); // Should include the new entry
        assert!(select_result.iter().any(|user| {
            user["id"] == 3 && user["name"] == "Charlie" && user["email"] == "charlie@email.com"
        }));

        // Pretty print the contents of the store
        println!("Store contents after insert: {:?}", select_result);
    }

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_update() {
        let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
        let path = tmpfile.path();

        let store = create_store(&JsonStoreTypes::File(path));

        let values = serde_json::json!({
            "email": "charlie@newdomain.com"
        });
        let query = JsonTableUpdateQuery::new("users", values).where_clause("id", 1);
        let result = query.execute(&store);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["id"], 1);
        assert_eq!(result[0]["name"], "Alice");
        assert_eq!(result[0]["email"], "charlie@newdomain.com");
        // Verify the updated data
        let select_query = JsonTableSelectQuery::new("users");
        let select_result = select_query.execute(&store);
        assert_eq!(select_result.len(), 2);
        assert!(select_result.iter().any(|user| {
            user["id"] == 1 && user["name"] == "Alice" && user["email"] == "charlie@newdomain.com"
        }));
        assert!(select_result.iter().any(|user| {
            user["id"] == 2 && user["name"] == "Bob" && user["email"] == "bob@email.com"
        }));

        // Pretty print the contents of the store
        println!("Store contents after update: {:?}", select_result);
    }

    #[test]
    #[cfg(not(feature = "test_sequential"))]
    fn test_delete() {
        let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
        let path = tmpfile.path();

        let store = create_store(&JsonStoreTypes::File(path));

        let query = JsonTableDeleteQuery::new("users").where_clause("id", 2);
        let result = query.execute(&store);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["id"], 2);
        assert_eq!(result[0]["name"], "Bob");
        assert_eq!(result[0]["email"], "bob@email.com");

        // Verify the deletion
        let select_query = JsonTableSelectQuery::new("users");
        let select_result = select_query.execute(&store);
        assert_eq!(select_result.len(), 1);
        assert!(select_result.iter().any(|user| {
            user["id"] == 1 && user["name"] == "Alice" && user["email"] == "alice@email.com"
        }));

        // Pretty print the contents of the store
        println!("Store contents after deletion: {:?}", select_result);
    }
}
