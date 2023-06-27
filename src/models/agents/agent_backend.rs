use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    ai_task_request, read_code_template_contents, read_exec_main_contents, save_backend_code,
};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use async_trait::async_trait;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    #[allow(dead_code)]
    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str = read_code_template_contents();

        // Concatenate Instruction
        let msg_context = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTIOM: {} \n",
            code_template_str, factsheet.project_description
        );

        dbg!(msg_context.clone());

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    #[allow(dead_code)]
    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let msg_context = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTIOM: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    #[allow(dead_code)]
    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    #[allow(dead_code)]
    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code = read_exec_main_contents();

        // Structure message context
        let msg_context = format!("CODE_INPUT: {:?}", backend_code);

        ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.update_state(AgentState::Working);
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.update_state(AgentState::UnitTesting);
                    continue;
                }
                AgentState::UnitTesting => self.attributes.update_state(AgentState::Finished),
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_writing_backend_code() {
        let mut agent = AgentBackendDeveloper::new();

        let factsheet_str = r#"
            {
                "project_description": "build a website that fetches and tracks fitness progress including timezone information.",
                "project_scope": {
                    "is_crud_required": true,
                    "is_user_login_and_logout": true,
                    "is_external_urls_required": true
                },
                "external_urls": [
                    "https://wger.de/api/v2/exerciseinfo/?language=2"
                ],
                "backend_code": null,
                "api_endpoint_schema": null
            }
        "#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer Agent");
    }
}
