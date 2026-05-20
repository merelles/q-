use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    catalog: Arc<RwLock<Vec<Product>>>,
    integrations: Arc<RwLock<Vec<IntegrationPartner>>>,
    config: AppConfig,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
struct AppConfig {
    database_url: String,
}

impl AppConfig {
    fn from_env() -> Self {
        let default_url = "host=localhost user=postgres password=password dbname=poc_q";
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| default_url.to_owned());
        Self { database_url }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum ProductStatus {
    Proposed,
    InSpecialistReview,
    Validated,
    Rejected,
    Deprecated,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum IntegrationAccessLevel {
    ReadOnly,
    CatalogWrite,
    CatalogAndMediaWrite,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum IntegrationStatus {
    Active,
    Suspended,
    Revoked,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
struct ProductImage {
    oid: String,
    url: String,
    alt_text: Option<String>,
    is_primary: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
struct Product {
    oid: String,
    tenant_oid: i64,
    name: String,
    normalized_name: String,
    status: ProductStatus,
    gtins: Vec<String>,
    images: Vec<ProductImage>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
struct IntegrationPartner {
    oid: String,
    tenant_oid: i64,
    name: String,
    access_level: IntegrationAccessLevel,
    status: IntegrationStatus,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
struct CreateProductRequest {
    tenant_oid: i64,
    name: String,
    normalized_name: String,
    gtins: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
struct CreateProductImageRequest {
    url: String,
    alt_text: Option<String>,
    is_primary: bool,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
struct ApiError {
    message: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
struct ServiceConfigResponse {
    database_url: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        service_config,
        list_products,
        get_product,
        create_product,
        create_product_image,
        list_integrations
    ),
    components(
        schemas(
            Product,
            ProductImage,
            ProductStatus,
            IntegrationPartner,
            IntegrationAccessLevel,
            IntegrationStatus,
            CreateProductRequest,
            CreateProductImageRequest,
            HealthResponse,
            ServiceConfigResponse,
            ApiError
        )
    ),
    tags(
        (name = "catalog", description = "Catalog product core API"),
        (name = "integration", description = "Integration partner API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();

    let state = AppState {
        catalog: Arc::new(RwLock::new(seed_products())),
        integrations: Arc::new(RwLock::new(seed_integrations())),
        config,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/config", get(service_config))
        .route("/api/v1/products", get(list_products).post(create_product))
        .route("/api/v1/products/{oid}", get(get_product))
        .route("/api/v1/products/{oid}/images", post(create_product_image))
        .route("/api/v1/integrations", get(list_integrations))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    println!("catalog-service running at http://{addr}");
    println!("swagger at http://{addr}/swagger-ui");
    axum::serve(listener, app).await.expect("server");
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "catalog",
    responses(
        (status = 200, description = "Service health", body = HealthResponse)
    )
)]
async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        service: "catalog-service",
        status: "ok",
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/config",
    tag = "catalog",
    responses(
        (status = 200, description = "Service configuration snapshot", body = ServiceConfigResponse)
    )
)]
async fn service_config(State(state): State<AppState>) -> impl IntoResponse {
    Json(ServiceConfigResponse {
        database_url: state.config.database_url,
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/products",
    tag = "catalog",
    responses(
        (status = 200, description = "List products", body = [Product])
    )
)]
async fn list_products(State(state): State<AppState>) -> impl IntoResponse {
    let catalog = state.catalog.read().expect("catalog read lock");
    Json(catalog.clone())
}

#[utoipa::path(
    get,
    path = "/api/v1/products/{oid}",
    tag = "catalog",
    params(
        ("oid" = String, Path, description = "Product OID")
    ),
    responses(
        (status = 200, description = "Product", body = Product),
        (status = 404, description = "Product not found", body = ApiError)
    )
)]
async fn get_product(
    Path(oid): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Product>, (StatusCode, Json<ApiError>)> {
    let catalog = state.catalog.read().expect("catalog read lock");
    let product = catalog.iter().find(|p| p.oid == oid).cloned();

    match product {
        Some(found) => Ok(Json(found)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                message: "product not found".to_owned(),
            }),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/products",
    tag = "catalog",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created", body = Product),
        (status = 400, description = "Validation error", body = ApiError)
    )
)]
async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), (StatusCode, Json<ApiError>)> {
    if payload.name.trim().is_empty() || payload.normalized_name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                message: "name and normalized_name are required".to_owned(),
            }),
        ));
    }

    let product = Product {
        oid: Uuid::new_v4().to_string(),
        tenant_oid: payload.tenant_oid,
        name: payload.name.trim().to_owned(),
        normalized_name: payload.normalized_name.trim().to_owned(),
        status: ProductStatus::Proposed,
        gtins: payload.gtins,
        images: Vec::new(),
    };

    let mut catalog = state.catalog.write().expect("catalog write lock");
    catalog.push(product.clone());
    Ok((StatusCode::CREATED, Json(product)))
}

#[utoipa::path(
    post,
    path = "/api/v1/products/{oid}/images",
    tag = "catalog",
    params(
        ("oid" = String, Path, description = "Product OID")
    ),
    request_body = CreateProductImageRequest,
    responses(
        (status = 201, description = "Image created", body = ProductImage),
        (status = 404, description = "Product not found", body = ApiError),
        (status = 400, description = "Validation error", body = ApiError)
    )
)]
async fn create_product_image(
    Path(oid): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<CreateProductImageRequest>,
) -> Result<(StatusCode, Json<ProductImage>), (StatusCode, Json<ApiError>)> {
    if payload.url.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                message: "url is required".to_owned(),
            }),
        ));
    }

    let mut catalog = state.catalog.write().expect("catalog write lock");
    let maybe_product = catalog.iter_mut().find(|p| p.oid == oid);

    let product = match maybe_product {
        Some(product) => product,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    message: "product not found".to_owned(),
                }),
            ))
        }
    };

    let new_image = ProductImage {
        oid: Uuid::new_v4().to_string(),
        url: payload.url,
        alt_text: payload.alt_text,
        is_primary: payload.is_primary,
    };

    if new_image.is_primary {
        for image in &mut product.images {
            image.is_primary = false;
        }
    }
    product.images.push(new_image.clone());

    Ok((StatusCode::CREATED, Json(new_image)))
}

#[utoipa::path(
    get,
    path = "/api/v1/integrations",
    tag = "integration",
    responses(
        (status = 200, description = "List integration partners", body = [IntegrationPartner])
    )
)]
async fn list_integrations(State(state): State<AppState>) -> impl IntoResponse {
    let integrations = state
        .integrations
        .read()
        .expect("integrations read lock");
    Json(integrations.clone())
}

fn seed_products() -> Vec<Product> {
    vec![Product {
        oid: "prd-demo-1".to_owned(),
        tenant_oid: 1,
        name: "Leite Integral 1L".to_owned(),
        normalized_name: "leite integral 1l".to_owned(),
        status: ProductStatus::Validated,
        gtins: vec!["7890000000001".to_owned()],
        images: vec![ProductImage {
            oid: "img-demo-1".to_owned(),
            url: "https://images.unsplash.com/photo-1550583724-b2692b85b150?auto=format&fit=crop&w=900&q=80".to_owned(),
            alt_text: Some("Caixa de leite integral".to_owned()),
            is_primary: true,
        }],
    }]
}

fn seed_integrations() -> Vec<IntegrationPartner> {
    vec![
        IntegrationPartner {
            oid: "int-erp-1".to_owned(),
            tenant_oid: 1,
            name: "ERP Parceiro Sul".to_owned(),
            access_level: IntegrationAccessLevel::CatalogWrite,
            status: IntegrationStatus::Active,
        },
        IntegrationPartner {
            oid: "int-trade-1".to_owned(),
            tenant_oid: 1,
            name: "Trade Spend Hub".to_owned(),
            access_level: IntegrationAccessLevel::ReadOnly,
            status: IntegrationStatus::Active,
        },
    ]
}
