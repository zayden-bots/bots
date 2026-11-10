use sqlx::migrate::Migrator;

pub static MIGRATOR: Migrator = sqlx::migrate!();

#[must_use]
pub fn migrator() -> Migrator {
    let mut migrator = sqlx::migrate!();
    migrator.set_ignore_missing(true);
    migrator
}
