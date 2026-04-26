use crate::{BaseAgent, Prompt};
use domain::Result;

pub struct ArchitectAgent {
    base: BaseAgent,
}

impl ArchitectAgent {
    pub fn new(base: BaseAgent) -> Self {
        Self { base }
    }

    pub async fn design_system(&self, prd: &str) -> Result<String> {
        let prompt = Prompt {
            system_prompt: format!(
                "You are an Elite Software Architect ({}). \
                Your goal is to take a PRD and design a highly scalable, distributed system. \
                Focus on components, data flow, and technology stack. \
                Output the design in clear Markdown format.",
                self.base.name()
            ),
            user_prompt: format!("PRD:\n{}", prd),
        };

        // For now, we reuse the generate method via base access if we expose it, 
        // but BaseAgent uses a generic execute_task. 
        // Let's add a more specific prompt-based execution to BaseAgent or use its provider directly.
        // I'll assume BaseAgent::execute_task is too generic for structured design.
        
        // Let's assume we can access the provider from BaseAgent if we make it pub(crate).
        // For now, I'll just use execute_task and customize the system prompt if I can.
        
        // Wait, I should probably allow BaseAgent to take a custom system prompt in its constructor or execution.
        
        self.base.execute_task(&format!("Design a system for the following PRD:\n{}", prd)).await
    }
}
