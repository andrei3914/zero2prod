use crate::helpers::is_valid_input_string;
use crate::routes::error_chain_fmt;
use actix_web::{http::StatusCode, ResponseError};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[derive(thiserror::Error)]
pub enum ConfirmationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("There is no subscriber associated with the provided token.")]
    UnknownToken,
}

impl std::fmt::Debug for ConfirmationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnknownToken => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmationError> {
    let token = &parameters.subscription_token;
    if !is_valid_input_string(token, 25) {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let subscriber_id = get_subscriber_id_from_token(&pool, token)
        .await
        .context("Failed to retrieve the subscriber id associated with the provided token.")?
        .ok_or(ConfirmationError::UnknownToken)?;

    let status = check_subscriber_already_confirmed(&pool, subscriber_id)
        .await
        .context("Failed to retrieve subscriber status.")?;
    if status == "confirmed" {
        return Ok(HttpResponse::Conflict().finish());
    }

    confirm_subscriber(&pool, subscriber_id)
        .await
        .context("Failed to update the subscriber status to `confirmed`.")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Check if subscriber already confirmed the subscription",
    skip(pool, subscriber_id)
)]
pub async fn check_subscriber_already_confirmed(
    pool: &PgPool,
    subscriber_id: Uuid,
) -> Result<String, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT status FROM subscriptions WHERE id = $1"#,
        subscriber_id
    )
    .fetch_one(pool)
    .await?;
    Ok(result.status)
}

#[tracing::instrument(name = "Get subscriber id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &String,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Confirm subscriber", skip(pool, subscriber_id))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status='confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
