use crate::{
    models::attendance_model::Attendance, repositories::attendance_repository::AttendanceRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct AttendanceService {
    attendance_repository: Arc<AttendanceRepository>,
}

impl AttendanceService {
    pub fn new(attendance_repository: Arc<AttendanceRepository>) -> Self {
        Self {
            attendance_repository,
        }
    }

    pub async fn create_attendance(&self, attendance: Attendance) -> Result<Attendance> {
        self.attendance_repository
            .create_attendance(&attendance)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_all_attendances(&self) -> Result<Vec<Attendance>> {
        self.attendance_repository
            .get_all_attendances()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_attendance_by_id(&self, attendance_id: &str) -> Result<Option<Attendance>> {
        self.attendance_repository
            .find_attendance_by_id(attendance_id)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn update_attendance(
        &self,
        attendance_id: &str,
        attendance: Attendance,
    ) -> Result<Attendance> {
        self.attendance_repository
            .update_attendance(attendance_id, &attendance)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn delete_attendance(&self, attendance_id: &str) -> Result<()> {
        self.attendance_repository
            .delete_attendance(attendance_id)
            .await
            .map_err(anyhow::Error::from)
    }
}
