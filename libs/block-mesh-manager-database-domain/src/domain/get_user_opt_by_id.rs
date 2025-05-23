use crate::domain::user::User;
use crate::domain::user::UserRole;
use database_utils::utils::option_uuid::OptionUuid;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_user_opt_by_id", skip_all)]
pub async fn get_user_opt_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"
        SELECT
        id,
        email,
        created_at,
        password as "password: Secret<String>",
        wallet_address,
        role as "role: UserRole",
        invited_by as "invited_by: OptionUuid",
        verified_email
        FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
