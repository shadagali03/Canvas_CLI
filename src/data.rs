use serde::{Deserialize, Serialize};

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
