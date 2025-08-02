use super::locks::{check_forgot_lock, increment_lock_key};
use crate::{AppError, AppState, WebsitePath};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;
use rand::{Rng, thread_rng};
use regex::Regex;
use std::sync::Arc;
use tracing::{debug, warn};

pub static CODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());

pub fn generate_code() -> String {
    let mut rng = thread_rng();

    format!("{:06}", rng.gen_range(0..1_000_000))
}

async fn send_code_email(
    state: Arc<AppState>,
    user_email: &str,
    code: &str,
) -> Result<(), AppError> {
    let email = Message::builder()
        .from(format!("BoilerSwap <{}>", state.config.email.from_email).parse()?)
        .to(user_email.parse()?)
        .subject("BoilerSwap Code")
        .body(format!("Your code is {code}"))?;

    let credentials = Credentials::new(
        state.config.email.from_email.to_string(),
        state.config.email.from_email_password.to_string(),
    );

    let mailer =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&state.config.email.from_email_server)?
            .credentials(credentials)
            .build();

    mailer.send(email).await?;

    Ok(())
}

pub fn spawn_code_task(
    state: Arc<AppState>,
    email: String,
    token: String,
    forgot_key: Option<String>,
    website_path: WebsitePath,
) {
    tokio::spawn(async move {
        if check_forgot_lock(state.clone(), &email, &forgot_key, &website_path).await {
            return;
        }

        if let Err(error) = send_code_email(state.clone(), &email, &token).await {
            match error {
                AppError::LettreAddress(msg) => debug!("Invalid email: {}", msg),
                AppError::LettreTransport(msg) => debug!("Transport error: {}", msg),
                other => warn!("Unexpected error: {:?}", other),
            }

            return;
        }

        if forgot_key.is_some()
            && (increment_lock_key(
                state.clone(),
                website_path.as_ref(),
                &forgot_key.expect("is_some failed"),
                &email,
                &state.config.authentication.verify_lock_duration_seconds,
                &state.config.authentication.verify_max_attempts,
            )
            .await)
                .is_err()
        {}
    });
}
