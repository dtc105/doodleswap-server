use crate::{
    config::database::AppState,
    dto::{errors::auth as err, requests::auth as req, responses::auth as res},
    middleware::authentication::Claims,
    models::auth as models,
};

use actix_web::{
    Error, HttpMessage, HttpRequest, HttpResponse,
    cookie::Cookie,
    error,
    web::{Data, Json},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use validator::Validate;

fn create_cookie<'a>(user_id: i32) -> Result<Cookie<'a>, Error> {
    let secret: String =
        std::env::var("JWT_SECRET").expect("Environment variable `JWT_SECRET` must be defined.");

    let iat: i64 = Utc::now().timestamp();
    let exp: i64 = iat + Duration::days(30).num_seconds();

    let claims: Claims = Claims {
        sub: user_id,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| {
        error::ErrorInternalServerError(format!("Issue signing token for user with id: {user_id}."))
    })?;

    Ok(Cookie::build("Authorization", token)
        .http_only(true)
        .secure(false)
        .path("/")
        .finish())
}

/// Reads the users token
///
/// # Route
/// `GET /auth/token`
///
/// # Responses
/// - `200 Ok`: Returns user data
/// - `401 Unauthorized`: If there is no token
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `GET /auth/token`
///
/// # Example Response 200
/// ```
/// {
///     "id": 123,
///     "role": "user",
///     "username": "JohnDoe123"
/// }
/// ```
pub async fn read_token(req: HttpRequest, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let ext = req.extensions();
    let claims = ext
        .get::<Claims>()
        .ok_or_else(|| error::ErrorUnauthorized("No token."))?;

    let user = sqlx::query_as!(
        models::User,
        r#"
            SELECT *
            FROM doodleswap.user
            WHERE id = $1;
        "#,
        claims.sub
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorNotFound(e.to_string()))?;

    Ok(HttpResponse::Ok().json(res::TokenResponse {
        id: claims.sub,
        role: user.role,
        username: user.username,
    }))
}

/// Handles user login and assigns a JWT
///
/// # Route
/// `POST /auth/login`
///
/// # Request Body
/// - `username`: The username the client is logging in with (3-32 chars)
/// - `password`: The password the client is logging in with (6+ chars)
///
/// # Responses
/// - `200 Ok`: Returns user data
/// - `400 Bad Request`: If missing or invalid parameters
/// - `401 Conflict`: If the username or password is incorrect
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `POST /auth/login`
///
/// # Example Request Body
/// ```
/// {
///     "username": "JohnDoe123",
///     "password" "password"
/// }
/// ```
///
/// # Example Response 200
/// ```
/// {
///     "id": 123,
///     "username": "JohnDoe123",
///     "role": "user"
/// }
/// ```
pub async fn login(
    body: Json<req::LoginCredentials>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Validate body
    body.validate()
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let user = sqlx::query_as!(
        models::User,
        r#"
            SELECT *
            FROM doodleswap.user
            WHERE username = $1;
        "#,
        body.username
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorNotFound(e.to_string()))?;

    // Check the password
    let is_password_correct: bool = verify(&body.password, &user.password_hash)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    if !is_password_correct {
        return Ok(
            HttpResponse::Unauthorized().json(err::unauthorized("Incorrect username or password."))
        );
    }

    // Sign the token and create a cookie
    let cookie = create_cookie(user.id)?;

    return Ok(HttpResponse::Ok().cookie(cookie).json(res::LoginResponse {
        id: user.id,
        role: user.role.as_str().to_owned(),
        username: user.username.as_str().to_owned(),
    }));
}

/// Handles user registration and assigns a JWT
///
/// # Route
/// `POST /auth/register`
///
/// # Request Body
/// - `username`: The username the client is registering with (3-32 chars)
/// - `password`: The password the client is registering with (6+ chars)
///
///
/// # Responses
/// - `201 Created`: Returns registered user data
/// - `400 Bad Request`: If missing or invalid parameters
/// - `409 Conflict`: If the username is already taken
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `POST /auth/register`
///
/// # Example Request Body
/// ```
/// {
///     "username": "JohnDoe123",
///     "password" "password"
/// }
/// ```
///
/// # Example Response 201
/// ```
/// {
///     "id": 123,
///     "username": "JohnDoe123",
///     "role": "user"
/// }
/// ```
pub async fn register(
    body: Json<req::RegistrationCredentials>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Validate body
    body.validate()
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    // Check if username is taken
    let username_taken = sqlx::query_scalar!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM doodleswap.user
                WHERE username = $1
            ) as "username_taken!";
        "#,
        body.username
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    if username_taken {
        return Ok(HttpResponse::Conflict().json(err::username_taken()));
    }

    // Check if email is taken
    let email_taken = sqlx::query_scalar!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM doodleswap.user
                WHERE email = $1
            ) as "email_taken!";
        "#,
        body.email
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    if email_taken {
        return Ok(HttpResponse::Conflict().json(err::email_taken()));
    }

    // Hash the password and insert it into the database
    let password_hash = hash(&body.password, DEFAULT_COST)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let inserted_id: i32 = sqlx::query_scalar!(
        r#"
            INSERT INTO doodleswap.user(email, username, password_hash, role)
            VALUES ($1, $2, $3, 'user')
            RETURNING id;
        "#,
        &body.email,
        &body.username,
        password_hash,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Sign the token and create a cookie
    let cookie = create_cookie(inserted_id)?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(res::RegistrationResponse {
            id: inserted_id,
            role: "user".to_string(),
            username: body.into_inner().username,
        }))
}

pub async fn change_email(
    req: HttpRequest,
    body: Json<req::EmailChange>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotImplemented().finish())
}

pub async fn change_username(
    req: HttpRequest,
    body: Json<req::UsernameChange>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotImplemented().finish())
}

pub async fn change_password(
    req: HttpRequest,
    body: Json<req::PasswordChange>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotImplemented().finish())
}

