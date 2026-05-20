//! Domain primitives for the product catalog.
//!
//! This crate intentionally has no database dependency. Persistence, queues,
//! APIs, and importers should implement the traits exposed here.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TenantOid(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductOid(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProposalOid(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BrandOid(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceOid(pub i16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitOid(pub i16);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ProductCoreError> {
        let value = value.as_ref().trim().to_ascii_uppercase();

        if value.len() != 2 || !value.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ProductCoreError::InvalidCountryCode(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gtin(String);

impl Gtin {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ProductCoreError> {
        let value = value.as_ref().trim();

        if !(8..=14).contains(&value.len()) || !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(ProductCoreError::InvalidGtin(value.to_owned()));
        }

        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Gtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductCoreError {
    InvalidCountryCode(String),
    InvalidGtin(String),
    EmptyName,
    InvalidReviewTransition {
        from: ReviewStatus,
        to: ReviewStatus,
    },
}

impl fmt::Display for ProductCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCountryCode(value) => write!(f, "invalid country code: {value}"),
            Self::InvalidGtin(value) => write!(f, "invalid GTIN: {value}"),
            Self::EmptyName => f.write_str("product name cannot be empty"),
            Self::InvalidReviewTransition { from, to } => {
                write!(f, "invalid review transition from {from:?} to {to:?}")
            }
        }
    }
}

impl std::error::Error for ProductCoreError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GtinType {
    Gtin8,
    Gtin12,
    Gtin13,
    Gtin14,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductScope {
    TenantLocal(TenantOid),
    CuratedGlobal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewStatus {
    Draft,
    Proposed,
    InSpecialistReview,
    Validated,
    Rejected,
    Deprecated,
}

impl ReviewStatus {
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Draft, Self::Proposed)
                | (Self::Proposed, Self::InSpecialistReview)
                | (Self::Proposed, Self::Rejected)
                | (Self::InSpecialistReview, Self::Validated)
                | (Self::InSpecialistReview, Self::Rejected)
                | (Self::Validated, Self::Deprecated)
                | (Self::Rejected, Self::Draft)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationDecision {
    Approve,
    Reject,
    RequestChanges,
    Deprecate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub oid: SourceOid,
    pub code: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brand {
    pub oid: BrandOid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub oid: UnitOid,
    pub code: String,
    pub description: String,
    pub symbol: String,
    pub decimal_places: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtinRecord {
    pub gtin: Gtin,
    pub gtin_type: GtinType,
    pub gs1_country_code: Option<CountryCode>,
    pub source_oid: SourceOid,
    pub indicator_digit: Option<u8>,
    pub gs1_prefix: Option<String>,
    pub item_reference: Option<String>,
    pub check_digit: Option<u8>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductName {
    pub raw: String,
    pub normalized: String,
}

impl ProductName {
    pub fn new(
        raw: impl AsRef<str>,
        normalized: impl AsRef<str>,
    ) -> Result<Self, ProductCoreError> {
        let raw = raw.as_ref().trim();
        let normalized = normalized.as_ref().trim();

        if raw.is_empty() || normalized.is_empty() {
            return Err(ProductCoreError::EmptyName);
        }

        Ok(Self {
            raw: raw.to_owned(),
            normalized: normalized.to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub oid: ProductOid,
    pub scope: ProductScope,
    pub name: ProductName,
    pub status: ReviewStatus,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductGtin {
    pub product_oid: ProductOid,
    pub gtin: Gtin,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductProposal {
    pub oid: ProposalOid,
    pub scope: ProductScope,
    pub proposed_name: ProductName,
    pub gtins: Vec<Gtin>,
    pub brand_oid: Option<BrandOid>,
    pub source_oid: SourceOid,
    pub status: ReviewStatus,
    pub evidence: Vec<ProductEvidence>,
}

impl ProductProposal {
    pub fn transition_to(&mut self, next: ReviewStatus) -> Result<(), ProductCoreError> {
        if !self.status.can_transition_to(next) {
            return Err(ProductCoreError::InvalidReviewTransition {
                from: self.status,
                to: next,
            });
        }

        self.status = next;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductEvidence {
    pub source_oid: SourceOid,
    pub external_ref: Option<String>,
    pub raw_name: Option<String>,
    pub raw_brand: Option<String>,
    pub raw_payload_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistReview {
    pub proposal_oid: ProposalOid,
    pub decision: ValidationDecision,
    pub reviewer: String,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductSearch {
    pub tenant_oid: Option<TenantOid>,
    pub gtin: Option<Gtin>,
    pub name_query: Option<String>,
    pub status: Option<ReviewStatus>,
}

pub trait ProductNormalizer {
    type Error;

    fn normalize_name(&self, raw: &str) -> Result<ProductName, Self::Error>;
    fn infer_gtin_type(&self, gtin: &Gtin) -> GtinType;
}

pub trait GtinRepository {
    type Error;

    fn get_gtin(&self, gtin: &Gtin) -> Result<Option<GtinRecord>, Self::Error>;
    fn upsert_gtin(&self, record: &GtinRecord) -> Result<(), Self::Error>;
    fn link_brand(&self, gtin: &Gtin, brand_oid: BrandOid) -> Result<(), Self::Error>;
}

pub trait ProductRepository {
    type Error;

    fn get_product(&self, oid: ProductOid) -> Result<Option<Product>, Self::Error>;
    fn search_products(&self, search: &ProductSearch) -> Result<Vec<Product>, Self::Error>;
    fn save_product(&self, product: &Product) -> Result<(), Self::Error>;
    fn link_gtin(&self, link: &ProductGtin) -> Result<(), Self::Error>;
}

pub trait ProductProposalWorkflow {
    type Error;

    fn propose_product(&self, proposal: ProductProposal) -> Result<ProposalOid, Self::Error>;
    fn get_proposal(&self, oid: ProposalOid) -> Result<Option<ProductProposal>, Self::Error>;
    fn submit_for_review(&self, oid: ProposalOid) -> Result<(), Self::Error>;
    fn apply_specialist_review(&self, review: SpecialistReview) -> Result<(), Self::Error>;
}

pub trait ProductAuditSink {
    type Error;

    fn record_event(&self, event: ProductAuditEvent) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductAuditEvent {
    pub aggregate: ProductAuditAggregate,
    pub aggregate_id: String,
    pub action: String,
    pub actor: String,
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductAuditAggregate {
    Gtin,
    Product,
    Proposal,
    Brand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_gtin() {
        let gtin = Gtin::parse("7891234567895").expect("valid GTIN should parse");

        assert_eq!(gtin.as_str(), "7891234567895");
    }

    #[test]
    fn rejects_invalid_gtin() {
        let error = Gtin::parse("abc").expect_err("invalid GTIN should fail");

        assert!(matches!(error, ProductCoreError::InvalidGtin(_)));
    }

    #[test]
    fn enforces_review_transitions() {
        let mut proposal = ProductProposal {
            oid: ProposalOid(1),
            scope: ProductScope::CuratedGlobal,
            proposed_name: ProductName::new("Leite Integral 1L", "leite integral 1l").unwrap(),
            gtins: vec![Gtin::parse("7891234567895").unwrap()],
            brand_oid: None,
            source_oid: SourceOid(1),
            status: ReviewStatus::Proposed,
            evidence: Vec::new(),
        };

        proposal
            .transition_to(ReviewStatus::InSpecialistReview)
            .expect("proposed product can enter review");

        assert_eq!(proposal.status, ReviewStatus::InSpecialistReview);
    }
}
