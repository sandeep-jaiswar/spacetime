use crate::BaseAgent;
use domain::Result;

pub struct CoderAgent {
    base: BaseAgent,
}

impl CoderAgent {
    pub fn new(base: BaseAgent) -> Self {
        Self { base }
    }

    pub async fn generate_code(&self, design_doc: &str, component_name: &str) -> Result<String> {
        let task = format!(
            "Based on the Design Doc below, generate production-ready Rust code for the component: {}.\n\nDesign Doc:\n{}",
            component_name, design_doc
        );

        self.base.execute_task(&task).await
    }
}
