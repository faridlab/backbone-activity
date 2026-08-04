use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ActivityType;
use super::ActivityDirection;
use super::ActivityStatus;
use super::AuditMetadata;

/// Strongly-typed ID for Activity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActivityId(pub Uuid);

impl ActivityId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ActivityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ActivityId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ActivityId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ActivityId> for Uuid {
    fn from(id: ActivityId) -> Self { id.0 }
}

impl AsRef<Uuid> for ActivityId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ActivityId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Activity {
    pub id: Uuid,
    pub company_id: Uuid,
    pub subject: String,
    pub activity_type: ActivityType,
    pub direction: Option<ActivityDirection>,
    pub lead_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub party_id: Option<Uuid>,
    pub status: ActivityStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub occurred_at: Option<DateTime<Utc>>,
    pub assignee_id: Option<Uuid>,
    pub outcome: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Activity {
    /// Create a builder for Activity
    pub fn builder() -> ActivityBuilder {
        ActivityBuilder::default()
    }

    /// Create a new Activity with required fields
    pub fn new(company_id: Uuid, subject: String, activity_type: ActivityType, status: ActivityStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            subject,
            activity_type,
            direction: None,
            lead_id: None,
            opportunity_id: None,
            party_id: None,
            status,
            scheduled_at: None,
            occurred_at: None,
            assignee_id: None,
            outcome: None,
            notes: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ActivityId {
        ActivityId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &ActivityStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the direction field (chainable)
    pub fn with_direction(mut self, value: ActivityDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Set the lead_id field (chainable)
    pub fn with_lead_id(mut self, value: Uuid) -> Self {
        self.lead_id = Some(value);
        self
    }

    /// Set the opportunity_id field (chainable)
    pub fn with_opportunity_id(mut self, value: Uuid) -> Self {
        self.opportunity_id = Some(value);
        self
    }

    /// Set the party_id field (chainable)
    pub fn with_party_id(mut self, value: Uuid) -> Self {
        self.party_id = Some(value);
        self
    }

    /// Set the scheduled_at field (chainable)
    pub fn with_scheduled_at(mut self, value: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    /// Set the occurred_at field (chainable)
    pub fn with_occurred_at(mut self, value: DateTime<Utc>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    /// Set the assignee_id field (chainable)
    pub fn with_assignee_id(mut self, value: Uuid) -> Self {
        self.assignee_id = Some(value);
        self
    }

    /// Set the outcome field (chainable)
    pub fn with_outcome(mut self, value: String) -> Self {
        self.outcome = Some(value);
        self
    }

    /// Set the notes field (chainable)
    pub fn with_notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "company_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_id = v; }
                }
                "subject" => {
                    if let Ok(v) = serde_json::from_value(value) { self.subject = v; }
                }
                "activity_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.activity_type = v; }
                }
                "direction" => {
                    if let Ok(v) = serde_json::from_value(value) { self.direction = v; }
                }
                "lead_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.lead_id = v; }
                }
                "opportunity_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.opportunity_id = v; }
                }
                "party_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.party_id = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "scheduled_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scheduled_at = v; }
                }
                "occurred_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.occurred_at = v; }
                }
                "assignee_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.assignee_id = v; }
                }
                "outcome" => {
                    if let Ok(v) = serde_json::from_value(value) { self.outcome = v; }
                }
                "notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notes = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Activity {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Activity"
    }
}

impl backbone_core::PersistentEntity for Activity {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for Activity {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("lead_id".to_string(), "uuid".to_string());
        m.insert("opportunity_id".to_string(), "uuid".to_string());
        m.insert("party_id".to_string(), "uuid".to_string());
        m.insert("assignee_id".to_string(), "uuid".to_string());
        m.insert("activity_type".to_string(), "activity_type".to_string());
        m.insert("direction".to_string(), "activity_direction".to_string());
        m.insert("status".to_string(), "activity_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["subject"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for Activity entity
///
/// Provides a fluent API for constructing Activity instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ActivityBuilder {
    company_id: Option<Uuid>,
    subject: Option<String>,
    activity_type: Option<ActivityType>,
    direction: Option<ActivityDirection>,
    lead_id: Option<Uuid>,
    opportunity_id: Option<Uuid>,
    party_id: Option<Uuid>,
    status: Option<ActivityStatus>,
    scheduled_at: Option<DateTime<Utc>>,
    occurred_at: Option<DateTime<Utc>>,
    assignee_id: Option<Uuid>,
    outcome: Option<String>,
    notes: Option<String>,
}

impl ActivityBuilder {
    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the subject field (required)
    pub fn subject(mut self, value: String) -> Self {
        self.subject = Some(value);
        self
    }

    /// Set the activity_type field (default: `ActivityType::default()`)
    pub fn activity_type(mut self, value: ActivityType) -> Self {
        self.activity_type = Some(value);
        self
    }

    /// Set the direction field (optional)
    pub fn direction(mut self, value: ActivityDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Set the lead_id field (optional)
    pub fn lead_id(mut self, value: Uuid) -> Self {
        self.lead_id = Some(value);
        self
    }

    /// Set the opportunity_id field (optional)
    pub fn opportunity_id(mut self, value: Uuid) -> Self {
        self.opportunity_id = Some(value);
        self
    }

    /// Set the party_id field (optional)
    pub fn party_id(mut self, value: Uuid) -> Self {
        self.party_id = Some(value);
        self
    }

    /// Set the status field (default: `ActivityStatus::default()`)
    pub fn status(mut self, value: ActivityStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the scheduled_at field (optional)
    pub fn scheduled_at(mut self, value: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    /// Set the occurred_at field (optional)
    pub fn occurred_at(mut self, value: DateTime<Utc>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    /// Set the assignee_id field (optional)
    pub fn assignee_id(mut self, value: Uuid) -> Self {
        self.assignee_id = Some(value);
        self
    }

    /// Set the outcome field (optional)
    pub fn outcome(mut self, value: String) -> Self {
        self.outcome = Some(value);
        self
    }

    /// Set the notes field (optional)
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }

    /// Build the Activity entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Activity, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let subject = self.subject.ok_or_else(|| "subject is required".to_string())?;

        Ok(Activity {
            id: Uuid::new_v4(),
            company_id,
            subject,
            activity_type: self.activity_type.unwrap_or(ActivityType::default()),
            direction: self.direction,
            lead_id: self.lead_id,
            opportunity_id: self.opportunity_id,
            party_id: self.party_id,
            status: self.status.unwrap_or(ActivityStatus::default()),
            scheduled_at: self.scheduled_at,
            occurred_at: self.occurred_at,
            assignee_id: self.assignee_id,
            outcome: self.outcome,
            notes: self.notes,
            metadata: AuditMetadata::default(),
        })
    }
}
