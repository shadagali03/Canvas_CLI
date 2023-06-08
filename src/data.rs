use serde::{Deserialize, Serialize};

// Use this site to construct the structs: https://quicktype.io/

pub enum DataTypes {
    Assignment,
    Course,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ValidCourse {
    pub name: String,
    pub course_code: String,
    pub id: i64,
}

impl ValidCourse {
    pub fn new(name: String, course_code: String, id: i64) -> ValidCourse {
        ValidCourse {
            name,
            course_code,
            id,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ValidAssignment {
    pub name: String,
    pub id: i64,
    pub due_at: String,
}

impl ValidAssignment {
    pub fn new(name: String, id: i64, due_at: String) -> ValidAssignment {
        ValidAssignment { name, id, due_at }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignments {
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    id: i64,
    name: String,
    created_at: String,
    sortable_name: String,
    short_name: String,
    avatar_url: String,
    locale: Option<serde_json::Value>,
    effective_locale: String,
    permissions: Permissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    can_update_name: bool,
    can_update_avatar: bool,
    limit_parent_app_web_access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: i64,
    pub name: Option<String>,
    account_id: Option<i64>,
    uuid: Option<String>,
    start_at: Option<String>,
    grading_standard_id: Option<i64>,
    is_public: Option<bool>,
    created_at: Option<String>,
    pub course_code: Option<String>,
    default_view: Option<String>,
    root_account_id: Option<i64>,
    enrollment_term_id: Option<i64>,
    license: Option<String>,
    grade_passback_setting: Option<serde_json::Value>,
    end_at: Option<String>,
    public_syllabus: Option<bool>,
    public_syllabus_to_auth: Option<bool>,
    storage_quota_mb: Option<i64>,
    is_public_to_auth_users: Option<bool>,
    homeroom_course: Option<bool>,
    course_color: Option<serde_json::Value>,
    friendly_name: Option<serde_json::Value>,
    apply_assignment_group_weights: Option<bool>,
    calendar: Option<Calendar>,
    time_zone: Option<String>,
    blueprint: Option<bool>,
    template: Option<bool>,
    enrollments: Option<Vec<Enrollment>>,
    hide_final_grades: Option<bool>,
    workflow_state: Option<String>,
    restrict_enrollments_to_course_dates: Option<bool>,
    overridden_course_visibility: Option<String>,
    course_format: Option<String>,
    access_restricted_by_date: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    ics: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enrollment {
    #[serde(rename = "type")]
    enrollment_type: String,
    role: String,
    role_id: i64,
    user_id: i64,
    enrollment_state: String,
    limit_privileges_to_course_section: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Option<i64>,
    pub name: Option<String>,
    description: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    pub due_at: Option<String>,
    lock_at: Option<String>,
    unlock_at: Option<String>,
    has_overrides: Option<bool>,
    all_dates: Option<serde_json::Value>,
    course_id: Option<i64>,
    assignment_group_id: Option<i64>,
    due_date_required: Option<bool>,
    allowed_extensions: Option<Vec<String>>,
    max_name_length: Option<i64>,
    turnitin_enabled: Option<bool>,
    vericite_enabled: Option<bool>,
    turnitin_settings: Option<serde_json::Value>,
    grade_group_students_individually: Option<bool>,
    external_tool_tag_attributes: Option<serde_json::Value>,
    peer_reviews: Option<bool>,
    automatic_peer_reviews: Option<bool>,
    peer_review_count: Option<i64>,
    peer_reviews_assign_at: Option<String>,
    intra_group_peer_reviews: Option<bool>,
    group_category_id: Option<i64>,
    needs_grading_count: Option<i64>,
    needs_grading_count_by_section: Option<Vec<NeedsGradingCountBySection>>,
    position: Option<i64>,
    post_to_sis: Option<bool>,
    integration_id: Option<String>,
    integration_data: Option<IntegrationData>,
    points_possible: Option<f32>,
    submission_types: Option<Vec<String>>,
    has_submitted_submissions: Option<bool>,
    grading_type: Option<String>,
    grading_standard_id: Option<serde_json::Value>,
    published: Option<bool>,
    unpublishable: Option<bool>,
    only_visible_to_overrides: Option<bool>,
    locked_for_user: Option<bool>,
    lock_info: Option<serde_json::Value>,
    lock_explanation: Option<String>,
    quiz_id: Option<i64>,
    anonymous_submissions: Option<bool>,
    discussion_topic: Option<serde_json::Value>,
    freeze_on_copy: Option<bool>,
    frozen: Option<bool>,
    frozen_attributes: Option<Vec<String>>,
    submission: Option<serde_json::Value>,
    use_rubric_for_grading: Option<bool>,
    rubric_settings: Option<RubricSettings>,
    rubric: Option<Rubric>,
    assignment_visibility: Option<Vec<i64>>,
    overrides: Option<serde_json::Value>,
    omit_from_final_grade: Option<bool>,
    moderated_grading: Option<bool>,
    grader_count: Option<i64>,
    final_grader_id: Option<i64>,
    grader_comments_visible_to_graders: Option<bool>,
    graders_anonymous_to_graders: Option<bool>,
    grader_names_visible_to_final_grader: Option<bool>,
    anonymous_grading: Option<bool>,
    allowed_attempts: Option<i64>,
    post_manually: Option<bool>,
    score_statistics: Option<serde_json::Value>,
    can_submit: Option<bool>,
    annotatable_attachment_id: Option<serde_json::Value>,
    anonymize_students: Option<bool>,
    require_lockdown_browser: Option<bool>,
    important_dates: Option<bool>,
    muted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockInfo {
    asset_string: Option<String>,
    unlock_at: Option<String>,
    lock_at: Option<String>,
    context_module: Option<serde_json::Value>,
    manually_locked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rubric {
    id: Option<String>,
    points: Option<f32>,
    description: Option<String>,
    long_description: Option<String>,
    criterion_use_range: Option<bool>,
    ratings: Option<Vec<Rating>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    id: Option<String>,
    points: Option<f32>,
    description: Option<Description>,
    long_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "Full Marks")]
    FullMarks,
    #[serde(rename = "No Marks")]
    NoMarks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationData {
    #[serde(rename = "5678")]
    the_5678: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedsGradingCountBySection {
    section_id: Option<String>,
    needs_grading_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RubricSettings {
    id: Option<i64>,
    title: Option<String>,
    points_possible: Option<i64>,
    free_form_criterion_comments: Option<bool>,
    hide_score_total: Option<bool>,
    hide_points: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUpload {
    file_param: Option<String>,
    progress: Option<serde_json::Value>,
    upload_url: Option<String>,
    upload_params: Option<UploadParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadParams {
    filename: Option<String>,
    content_type: Option<String>,
}

impl FileUpload {
    pub fn new(
        file_param: String,
        upload_url: String,
        filename: String,
        content_type: String,
    ) -> FileUpload {
        FileUpload {
            file_param: Some(file_param),
            progress: None,
            upload_url: Some(upload_url),
            upload_params: Some(UploadParams {
                filename: Some(filename),
                content_type: Some(content_type),
            }),
        }
    }
}
