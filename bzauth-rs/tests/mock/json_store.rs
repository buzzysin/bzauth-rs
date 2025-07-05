use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::{Arc, RwLock};

pub type JsonStoreType = Option<String>;

#[allow(non_snake_case)]
pub mod JsonStoreTypes {
    use std::path::Path;

    use super::JsonStoreType;

    #[allow(non_upper_case_globals)]
    pub(crate) const Memory: JsonStoreType = None;

    #[allow(non_snake_case)]
    pub(crate) fn File<P>(path: P) -> JsonStoreType
    where
        P: AsRef<Path>,
    {
        Some(path.as_ref().to_string_lossy().to_string())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JsonStore {
    store: Arc<RwLock<JsonStoreInner>>,
    r#type: JsonStoreType,
}

#[derive(Debug, Clone)]
struct JsonStoreInner {
    data: serde_json::Value,
}

pub(crate) type JsonStoreError = String;
pub(crate) type Result<T> = std::result::Result<T, JsonStoreError>;

impl JsonStoreInner {
    pub(crate) fn new() -> Self {
        Self {
            data: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    pub(crate) fn load(&mut self, path: &str) {
        *self = Self::load_from_file(&path).expect("Failed to load JSON store");
    }

    pub(crate) fn save(&self, path: &str) -> Result<()> {
        Self::save_to_file(&path, &self.data)
    }

    pub(crate) fn load_from_file<P: AsRef<Path>>(path: &P) -> Result<Self> {
        if std::fs::metadata(&path).is_err() {
            Self::create_file_and_load(path)
        } else {
            Self::open_file_and_load(path)
        }
    }

    fn create_file_and_load<P: AsRef<Path>>(path: &P) -> Result<Self> {
        let _ = Self::open_options_retries(
            &path,
            &OpenOptions::new().create(true).write(true).read(true),
            10,
        )?;

        let data = Self::read_json_retries(path)?;
        // Ensure the file is created with an empty object if it was empty
        if data.is_null() || !data.is_object() {
            return Err("File is empty or not a valid JSON object".to_string());
        }

        Ok(Self { data })
    }

    fn open_file_and_load<P: AsRef<Path>>(path: &P) -> Result<Self> {
        let _ = Self::open_options_retries(
            &path,
            &OpenOptions::new().read(true).write(true).truncate(true),
            10,
        )?;

        let data = Self::read_json_retries(path)?;

        Ok(Self { data })
    }

    fn open_options_retries<P: AsRef<Path>>(
        path: &P,
        options: &OpenOptions,
        retries: usize,
    ) -> Result<File> {
        let mut attempts = 0;
        loop {
            match options.open(&path) {
                Ok(file) => return Ok(file),
                Err(_) if attempts < retries => {
                    attempts += 1;
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    fn read_json_retries<P: AsRef<Path>>(path: P) -> Result<serde_json::Value> {
        // Read everything. If it's empty, inject an empty object
        let raw =
            std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

        if raw.is_empty() {
            return Ok(serde_json::Value::Object(serde_json::Map::new()));
        }

        match serde_json::from_str(&raw) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Failed to parse JSON: {}", e)),
        }
    }

    pub(crate) fn save_to_file(path: &str, data: &serde_json::Value) -> Result<()> {
        let _ =
            Self::open_options_retries(&path, &OpenOptions::new().write(true).create(true), 10)?;

        std::fs::write(&path, data.to_string())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }
}

impl JsonStore {
    pub(crate) fn new(store_type: &JsonStoreType) -> Self {
        let store = match store_type {
            Some(path) => Arc::new(RwLock::new(JsonStoreInner::load_from_file(path).unwrap())),
            None => Arc::new(RwLock::new(JsonStoreInner::new())),
        };
        Self {
            store,
            r#type: store_type.clone(),
        }
    }

    pub(crate) fn _get_store_type(&self) -> &JsonStoreType {
        &self.r#type
    }

    pub(crate) fn get_data(&self) -> Result<serde_json::Value> {
        self.r_transaction(|store| Ok(store.data.clone()))
    }

    pub(crate) fn set_data(&self, data: serde_json::Value) -> Result<()> {
        self.w_transaction(|store| {
            store.data = data;
            if let Some(path) = &self.r#type {
                store.save(path)?;
            }

            Ok(())
        })
    }

    fn w_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut JsonStoreInner) -> Result<T>,
    {
        let mut store = self.store.write().map_err(|e| e.to_string())?;
        f(&mut store)
    }

    fn r_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&JsonStoreInner) -> Result<T>,
    {
        let store = self.store.read().map_err(|e| e.to_string())?;
        f(&store)
    }

    pub(crate) fn clear(&self) -> Result<()> {
        self.w_transaction(|store| {
            store.data = serde_json::Value::Object(serde_json::Map::new());
            if let Some(path) = &self.r#type {
                store.save(path)?;
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone)]
pub struct JsonTableSelectQuery {
    pub table: String,
    pub columns: Vec<String>,
    pub where_clause: Vec<(String, serde_json::Value)>,
}

impl JsonTableSelectQuery {
    pub(crate) fn new<T: AsRef<str>>(table: T) -> Self {
        Self {
            table: table.as_ref().to_string(),
            columns: Vec::new(),
            where_clause: Vec::new(),
        }
    }
    pub(crate) fn select<C: AsRef<str>>(mut self, columns: Vec<C>) -> Self {
        self.columns = columns.iter().map(|c| c.as_ref().to_string()).collect();
        self
    }

    pub(crate) fn where_clause<C: AsRef<str>, V: serde::Serialize>(
        mut self,
        column: C,
        value: V,
    ) -> Self {
        let column = column.as_ref().to_string();
        let value = serde_json::to_value(value).unwrap();
        self.where_clause.push((column, value));
        self
    }

    pub(crate) fn execute(&self, store: &JsonStore) -> Vec<serde_json::Value> {
        JsonTableInterface::execute(self, store)
    }
}

#[derive(Debug, Clone)]
pub struct JsonTableInsertQuery {
    table: String,
    values: serde_json::Value,
}

impl JsonTableInsertQuery {
    pub(crate) fn new<T: AsRef<str>, V: serde::Serialize>(table: T, values: V) -> Self {
        let table = table.as_ref().to_string();
        let values = serde_json::to_value(values).unwrap();
        Self { table, values }
    }

    pub(crate) fn execute(&self, store: &JsonStore) -> Vec<serde_json::Value> {
        JsonTableInterface::execute(self, store)
    }
}

#[derive(Debug, Clone)]
pub struct JsonTableUpdateQuery {
    table: String,
    values: serde_json::Value,
    where_clause: Vec<(String, serde_json::Value)>,
}

impl JsonTableUpdateQuery {
    pub(crate) fn new<T: AsRef<str>, V: serde::Serialize>(table: T, values: V) -> Self {
        let table = table.as_ref().to_string();
        let values = serde_json::to_value(values).unwrap();
        Self {
            table,
            values,
            where_clause: Vec::new(),
        }
    }

    pub(crate) fn where_clause<C: AsRef<str>, V: serde::Serialize>(
        mut self,
        column: C,
        value: V,
    ) -> Self {
        let column = column.as_ref().to_string();
        let value = serde_json::to_value(value).unwrap();
        self.where_clause.push((column, value));
        self
    }

    pub(crate) fn execute(&self, store: &JsonStore) -> Vec<serde_json::Value> {
        JsonTableInterface::execute(self, store)
    }
}

#[derive(Debug, Clone)]
pub struct JsonTableDeleteQuery {
    pub table: String,
    pub where_clause: Vec<(String, serde_json::Value)>,
}

impl JsonTableDeleteQuery {
    pub(crate) fn new<T: AsRef<str>>(table: T) -> Self {
        let table = table.as_ref().to_string();
        Self {
            table,
            where_clause: Vec::new(),
        }
    }

    pub(crate) fn where_clause<C: AsRef<str>, V: serde::Serialize>(
        mut self,
        column: C,
        value: V,
    ) -> Self {
        let column = column.as_ref().to_string();
        let value = serde_json::to_value(value).unwrap();
        self.where_clause.push((column, value));
        self
    }

    pub(crate) fn execute(&self, store: &JsonStore) -> Vec<serde_json::Value> {
        JsonTableInterface::execute(self, store)
    }
}

macro_rules! impl_from_query {
    ($query_type:ty, $query_name:ident) => {
        impl<'a> From<&'a $query_type> for JsonTableQuery {
            fn from(query: &$query_type) -> Self {
                JsonTableQuery::$query_name(query.clone())
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum JsonTableQuery {
    Select(JsonTableSelectQuery),
    Insert(JsonTableInsertQuery),
    Update(JsonTableUpdateQuery),
    Delete(JsonTableDeleteQuery),
}

impl JsonTableQuery {
    pub(crate) fn select<T: AsRef<str>>(table: T) -> JsonTableSelectQuery {
        JsonTableSelectQuery::new(table)
    }
    pub(crate) fn insert(table: String, values: serde_json::Value) -> JsonTableInsertQuery {
        JsonTableInsertQuery { table, values }
    }
    pub(crate) fn update<T: AsRef<str>, V: serde::Serialize>(
        table: T,
        values: V,
    ) -> JsonTableUpdateQuery {
        let table = table.as_ref().to_string();
        let values = serde_json::to_value(values).unwrap();

        JsonTableUpdateQuery {
            table,
            values,
            where_clause: Vec::new(),
        }
    }
    pub(crate) fn delete<T: AsRef<str>>(table: T) -> JsonTableDeleteQuery {
        let table = table.as_ref().to_string();
        JsonTableDeleteQuery {
            table,
            where_clause: Vec::new(),
        }
    }
}

impl_from_query!(JsonTableSelectQuery, Select);
impl_from_query!(JsonTableInsertQuery, Insert);
impl_from_query!(JsonTableUpdateQuery, Update);
impl_from_query!(JsonTableDeleteQuery, Delete);

#[derive(Debug, Clone)]
pub struct JsonTableInterface;

impl JsonTableInterface {
    pub(crate) fn execute<Q>(query: &Q, store: &JsonStore) -> Vec<serde_json::Value>
    where
        for<'a> &'a Q: Into<JsonTableQuery>,
    {
        match query.into() {
            JsonTableQuery::Select(q) => Self::execute_select(&q, store),
            JsonTableQuery::Insert(q) => Self::execute_insert(&q, store),
            JsonTableQuery::Update(q) => Self::execute_update(&q, store),
            JsonTableQuery::Delete(q) => Self::execute_delete(&q, store),
        }
    }

    fn execute_select(query: &JsonTableSelectQuery, store: &JsonStore) -> Vec<serde_json::Value> {
        let data = store.get_data().unwrap_or_default();
        let default = Vec::new();
        let table_data = data
            .get(&query.table)
            .and_then(|v| v.as_array())
            .unwrap_or(&default);

        table_data
            .iter()
            .filter(|item| {
                query
                    .where_clause
                    .iter()
                    .all(|(col, val)| item.get(col) == Some(val))
            })
            .map(|item| {
                if query.columns.is_empty() {
                    item.clone()
                } else {
                    serde_json::Value::Object(
                        query
                            .columns
                            .iter()
                            .filter_map(|col| item.get(col).map(|v| (col.clone(), v.clone())))
                            .collect(),
                    )
                }
            })
            .collect()
    }

    fn execute_insert(query: &JsonTableInsertQuery, store: &JsonStore) -> Vec<serde_json::Value> {
        // Return inserted data
        let mut data = store.get_data().unwrap_or_default();
        let inserted;

        if let Some(table) = data.get_mut(&query.table) {
            if let Some(array) = table.as_array_mut() {
                array.push(query.values.clone());
                inserted = query.values.clone();
            } else {
                return vec![];
            }
        } else {
            data[&query.table] = serde_json::Value::Array(vec![query.values.clone()]);
            inserted = query.values.clone();
        }

        store.set_data(data).unwrap();
        vec![inserted]
    }

    fn execute_update(query: &JsonTableUpdateQuery, store: &JsonStore) -> Vec<serde_json::Value> {
        let mut data = store.get_data().unwrap_or_default();
        let mut updated = Vec::new();

        if let Some(table) = data.get_mut(&query.table) {
            if let Some(array) = table.as_array_mut() {
                for item in array.iter_mut() {
                    if query
                        .where_clause
                        .iter()
                        .all(|(col, val)| item.get(col) == Some(val))
                    {
                        for (col, val) in query.values.as_object().unwrap().iter() {
                            item[col] = val.clone();
                        }
                        updated.push(item.clone());
                    }
                }
            }
        }

        store.set_data(data).unwrap();
        updated
    }

    fn execute_delete(query: &JsonTableDeleteQuery, store: &JsonStore) -> Vec<serde_json::Value> {
        let mut data = store.get_data().unwrap_or_default();
        let mut deleted = Vec::new();

        if let Some(table) = data.get_mut(&query.table) {
            if let Some(array) = table.as_array_mut() {
                array.retain(|item| {
                    let should_delete = query
                        .where_clause
                        .iter()
                        .all(|(col, val)| item.get(col) == Some(val));

                    should_delete
                        .then(|| {
                            deleted.push(item.clone());
                            false
                        })
                        .unwrap_or(true)
                });
            }
        }

        store.set_data(data).unwrap();
        deleted
    }
}
