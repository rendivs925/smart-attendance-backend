use crate::models::class_model::Class;
use crate::repositories::class_repository::ClassRepository;
use mongodb::error::Result;
use std::sync::Arc;

pub struct ClassService {
    class_repository: Arc<ClassRepository>,
}

impl ClassService {
    pub fn new(class_repository: Arc<ClassRepository>) -> Self {
        Self { class_repository }
    }

    pub async fn create_class(&self, class: &Class) -> Result<Class> {
        self.class_repository.create_class(class).await
    }

    pub async fn get_all_classes(&self) -> Result<Vec<Class>> {
        self.class_repository.get_all_classes().await
    }

    pub async fn get_class_by_id(&self, class_id: &str) -> Result<Option<Class>> {
        self.class_repository.find_class_by_id(class_id).await
    }

    pub async fn update_class(&self, class_id: &str, class_data: &Class) -> Result<Class> {
        self.class_repository
            .update_class(class_id, class_data)
            .await
    }

    pub async fn delete_class(&self, class_id: &str) -> Result<()> {
        self.class_repository.delete_class(class_id).await
    }
}
