use crate::utils::Utils;

/// A struct that represents a database.
pub struct Database {
    pool: std::sync::Arc<sqlx::MySqlPool>,
}

impl Database {
    /// Connects to a  [`Database`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the database cannot be connected to.
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = sqlx::MySqlPool::connect(&format!("mysql://")).await?;
        Ok(Self {
            pool: std::sync::Arc::new(pool),
        })
    }

    /// Initializes the table with the given name.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to initialize.
    ///
    /// # Errors
    ///
    /// This function will return an error if the table cannot be initialized.
    pub async fn init_table(&self, table: &str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            Utils::sanitize(table)
        ))
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    /// Sets the data of this [`Database`].
    ///
    /// # Arguments
    ///
    /// * `table` - The table to set the data in.
    /// * `key` - The key of the data to set.
    /// * `value` - The value of the data to set.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data cannot be set.
    pub async fn set_data(&self, table: &str, key: &str, value: &str) -> Result<(), sqlx::Error> {
        self.init_table(table).await?;
        sqlx::query(&format!(
            "INSERT OR REPLACE INTO \"{}\" (key, value) VALUES (?1, ?2)",
            Utils::sanitize(table)
        ))
        .bind(key)
        .bind(value)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    /// Updates the data of this [`Database`].
    ///
    /// # Arguments
    ///
    /// * `table` - The table to update the data in.
    /// * `key` - The key of the data to update.
    /// * `value` - The new value of the data.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data cannot be updated.
    pub async fn update_data(
        &self,
        table: &str,
        key: &str,
        value: &str,
    ) -> Result<(), sqlx::Error> {
        self.init_table(table).await?;
        sqlx::query(&format!(
            "UPDATE \"{}\" SET value = ?1 WHERE key = ?2",
            Utils::sanitize(table)
        ))
        .bind(value)
        .bind(key)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    /// Gets the data of this [`Database`].
    ///
    /// # Arguments
    ///
    /// * `table` - The table to get the data from.
    /// * `key` - The key of the data to get.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data cannot be retrieved.
    pub async fn get_data(&self, table: &str, key: &str) -> Result<Option<String>, sqlx::Error> {
        self.init_table(table).await?;
        Ok(sqlx::query_scalar(&format!(
            "SELECT value FROM \"{}\" WHERE key = ?1",
            Utils::sanitize(table)
        ))
        .bind(key)
        .fetch_optional(&*self.pool)
        .await?)
    }

    /// Deletes the data of this [`Database`].
    ///
    /// # Arguments
    ///
    /// * `table` - The table to delete the data from.
    /// * `key` - The key of the data to delete.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data cannot be deleted.
    pub async fn delete_data(&self, table: &str, key: &str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "DELETE FROM \"{}\" WHERE key = ?1",
            Utils::sanitize(table)
        ))
        .bind(key)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    /// Deletes the table with the given name.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to delete.
    ///
    /// # Errors
    ///
    /// This function will return an error if the table cannot be deleted.
    pub async fn delete_table(&self, table: &str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "DROP TABLE IF EXISTS \"{}\"",
            Utils::sanitize(table)
        ))
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
