/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Server
    pub host: String,
    pub port: u16,

    // Database
    pub database_url: String,

    // JWT
    pub jwt_secret: String,
    pub jwt_access_expiry_secs: i64,
    pub jwt_refresh_expiry_secs: i64,

    // Google OAuth
    pub google_client_id: String,
    pub google_client_secret: String,

    // Midtrans
    pub midtrans_server_key: String,
    pub midtrans_is_production: bool,

    // Storage (S3-compatible)
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_region: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    /// Panics on missing required vars (fail-fast at startup).
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .expect("PORT must be a valid number"),

            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is required"),

            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET is required"),
            jwt_access_expiry_secs: std::env::var("JWT_ACCESS_EXPIRY_SECS")
                .unwrap_or_else(|_| "900".to_string()) // 15 minutes
                .parse()
                .expect("JWT_ACCESS_EXPIRY_SECS must be a number"),
            jwt_refresh_expiry_secs: std::env::var("JWT_REFRESH_EXPIRY_SECS")
                .unwrap_or_else(|_| "604800".to_string()) // 7 days
                .parse()
                .expect("JWT_REFRESH_EXPIRY_SECS must be a number"),

            google_client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),

            midtrans_server_key: std::env::var("MIDTRANS_SERVER_KEY").unwrap_or_default(),
            midtrans_is_production: std::env::var("MIDTRANS_IS_PRODUCTION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            s3_endpoint: std::env::var("S3_ENDPOINT").unwrap_or_default(),
            s3_bucket: std::env::var("S3_BUCKET").unwrap_or_default(),
            s3_access_key: std::env::var("S3_ACCESS_KEY").unwrap_or_default(),
            s3_secret_key: std::env::var("S3_SECRET_KEY").unwrap_or_default(),
            s3_region: std::env::var("S3_REGION").unwrap_or_else(|_| "auto".to_string()),
        }
    }
}
