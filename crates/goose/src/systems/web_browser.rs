use anyhow::Result as AnyhowResult;
use async_trait::async_trait;
use base64::Engine;
use headless_chrome::{
    Browser, LaunchOptions, Tab,
};
use image::{imageops::FilterType, GenericImageView};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::errors::{AgentError, AgentResult};
use crate::models::content::Content;
use crate::models::tool::Tool;
use crate::models::tool::ToolCall;
use crate::systems::{Resource, System};

pub struct WebBrowserSystem {
    tools: Vec<Tool>,
    browser: Arc<Mutex<Option<Browser>>>,
    tab: Arc<Mutex<Option<Arc<Tab>>>>,
    instructions: String,
}

impl Default for WebBrowserSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl WebBrowserSystem {
    pub fn new() -> Self {
        let navigate_tool = Tool::new(
            "navigate",
            "Navigate to a URL in the browser",
            json!({
                "type": "object",
                "required": ["url"],
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to navigate to"
                    },
                    "wait_for": {
                        "type": "string",
                        "default": null,
                        "description": "Optional CSS selector to wait for before continuing"
                    }
                }
            }),
        );

        let screenshot_tool = Tool::new(
            "screenshot",
            "Take a screenshot of the current page or element",
            json!({
                "type": "object",
                "required": [],
                "properties": {}
            }),
        );

        let click_tool = Tool::new(
            "click",
            "Click on an element in the page",
            json!({
                "type": "object",
                "required": ["selector"],
                "properties": {
                    "selector": {
                        "type": "string",
                        "description": "CSS selector for the element to click"
                    },
                    "wait_for": {
                        "type": "string",
                        "default": null,
                        "description": "Optional CSS selector to wait for after clicking"
                    }
                }
            }),
        );

        let type_tool = Tool::new(
            "type",
            "Type text into an input element",
            json!({
                "type": "object",
                "required": ["selector", "text"],
                "properties": {
                    "selector": {
                        "type": "string",
                        "description": "CSS selector for the input element"
                    },
                    "text": {
                        "type": "string",
                        "description": "Text to type into the element"
                    },
                    "clear_first": {
                        "type": "boolean",
                        "default": true,
                        "description": "Whether to clear the input before typing"
                    }
                }
            }),
        );

        let eval_tool = Tool::new(
            "eval",
            "Evaluate JavaScript in the page context",
            json!({
                "type": "object",
                "required": ["script"],
                "properties": {
                    "script": {
                        "type": "string",
                        "description": "JavaScript code to evaluate"
                    }
                }
            }),
        );

        let wait_for_tool = Tool::new(
            "wait_for",
            "Wait for an element to appear",
            json!({
                "type": "object",
                "required": ["selector"],
                "properties": {
                    "selector": {
                        "type": "string",
                        "description": "CSS selector to wait for"
                    },
                    "timeout": {
                        "type": "integer",
                        "default": 30000,
                        "description": "Maximum time to wait in milliseconds"
                    }
                }
            }),
        );

        let instructions = indoc::formatdoc! {r#"
            The web browser system provides automation capabilities using headless Chrome.
            
            Available tools:
            - navigate: Load a URL in the browser with optional wait conditions
            - screenshot: Capture the current page if needed to visually examine
            - click: Click on elements using CSS selectors
            - type: Enter text into input fields
            - eval: Execute JavaScript in the page context
            - wait_for: Wait for elements to appear
            
            Notes:
            - The browser session persists between commands
            - Screenshots are returned as base64-encoded PNG images
            - CSS selectors must be valid and match exactly one element
            - JavaScript evaluation runs in the page context
            - All commands support various wait conditions for reliability
            "#};

        let get_text_tool = Tool::new(
            "get_text",
            "Get the page content as text and save to a temporary file",
            json!({
                "type": "object",
                "required": [],
                "properties": {}
            }),
        );

        Self {
            tools: vec![
                navigate_tool,
                screenshot_tool,
                click_tool,
                type_tool,
                eval_tool,
                wait_for_tool,
                get_text_tool,
            ],
            browser: Arc::new(Mutex::new(None)),
            tab: Arc::new(Mutex::new(None)),
            instructions,
        }
    }

    async fn ensure_browser(&self) -> AgentResult<()> {
        let mut browser_guard = self.browser.lock().await;
        if browser_guard.is_none() {
            let options = LaunchOptions::default_builder()
                .window_size(Some((1920, 1080)))
                .headless(true)
                .build()
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

            let browser = Browser::new(options)
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

            // Create initial tab
            let tab = browser
                .new_tab()
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

            *browser_guard = Some(browser);
            *self.tab.lock().await = Some(tab);
        }
        Ok(())
    }

    async fn navigate(&self, url: &str, wait_for: Option<&str>) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;
        
        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();
        
        tab.navigate_to(url)
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        // Wait for page to load
        tab.wait_for_element("body")
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        // Wait for specific element if requested
        if let Some(wait_for) = wait_for {
            tab.wait_for_element(wait_for)
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;
        }

        Ok(vec![Content::text(format!("Navigated to {}", url))])
    }

    async fn screenshot(&self) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;

        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();

        let screenshot_data = tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )
        .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        // Convert the screenshot data to an image
        let img = image::load_from_memory(&screenshot_data)
            .map_err(|e| AgentError::ExecutionError(format!("Failed to load image: {}", e)))?;

        let (width, height) = img.dimensions();
        let max_width = 768;
        
        let final_image = if width > max_width {
            // Calculate new height while preserving aspect ratio
            let aspect_ratio = width as f32 / height as f32;
            let new_height = (max_width as f32 / aspect_ratio) as u32;
            
            // Resize the image
            img.resize(max_width, new_height, FilterType::Lanczos3)
        } else {
            img
        };

        // Convert the image back to PNG format
        let mut png_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_data);
        final_image
            .write_to(&mut cursor, image::ImageOutputFormat::Png)
            .map_err(|e| AgentError::ExecutionError(format!("Failed to encode image: {}", e)))?;

        let base64 = base64::prelude::BASE64_STANDARD.encode(&png_data);
        Ok(vec![Content::image(base64, "image/png")])
    }

    async fn click(&self, selector: &str, wait_for: Option<&str>) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;

        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();

        let element = tab.wait_for_element(selector)
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        element.click()
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        // Wait for specific element after click if requested
        if let Some(wait_for) = wait_for {
            tab.wait_for_element(wait_for)
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;
        }

        Ok(vec![Content::text(format!("Clicked element matching '{}'", selector))])
    }

    async fn type_text(&self, selector: &str, text: &str, clear_first: Option<bool>) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;

        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();

        let element = tab.wait_for_element(selector)
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        if clear_first.unwrap_or(true) {
            element.click()
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

            // Clear using keyboard shortcuts
            element.focus()
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;
            
            // Select all and delete
            element.type_into("a")
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;
            
            tab.press_key("Backspace")
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;
        }

        element.type_into(text)
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        Ok(vec![Content::text(format!(
            "Typed text into element matching '{}'",
            selector
        ))])
    }

    async fn eval(&self, script: &str) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;

        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();

        let result = tab.evaluate(script, false)
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;
        
        // Extract just the value from the RemoteObject
        let value = match result.value {
            Some(ref value) => format!("{}", value),
            None => "undefined".to_string(),
        };
        
        Ok(vec![Content::text(value)])
    }

    async fn wait_for(&self, selector: &str, timeout: Option<u64>) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;

        let timeout = timeout.unwrap_or(30000);

        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();

        tab.wait_for_element_with_custom_timeout(selector, Duration::from_millis(timeout))
            .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        Ok(vec![Content::text(format!(
            "Successfully waited for element matching '{}'",
            selector
        ))])
    }

    async fn get_text(&self) -> AgentResult<Vec<Content>> {
        self.ensure_browser().await?;

        let tab = self.tab.lock().await;
        let tab = tab.as_ref().unwrap();

        // Get the page content using JavaScript
        let result = tab.evaluate(
            r#"
            document.body.innerText
            "#,
            false,
        )
        .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

        let text = match result.value {
            Some(ref value) => format!("{}", value),
            None => String::new(),
        };

        // Create a temporary file with the text content
        let temp_dir = std::env::temp_dir();
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let file_name = format!("page_content_{}.txt", timestamp);
        let file_path = temp_dir.join(file_name);

        std::fs::write(&file_path, text)
            .map_err(|e| AgentError::ExecutionError(format!("Failed to write text file: {}", e)))?;

        Ok(vec![Content::text(format!(
            "Page content saved to: {}",
            file_path.display()
        ))])
    }
}

#[async_trait]
impl System for WebBrowserSystem {
    fn name(&self) -> &str {
        "WebBrowserSystem"
    }

    fn description(&self) -> &str {
        "Browser automation system using headless Chrome"
    }

    fn instructions(&self) -> &str {
        &self.instructions
    }

    fn tools(&self) -> &[Tool] {
        &self.tools
    }

    async fn status(&self) -> AnyhowResult<Vec<Resource>> {
        Ok(Vec::new())
    }

    async fn call(&self, tool_call: ToolCall) -> AgentResult<Vec<Content>> {
        match tool_call.name.as_str() {
            "navigate" => {
                let url = tool_call.arguments.get("url")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AgentError::InvalidParameters("Missing 'url' parameter".into()))?;
                let wait_for = tool_call.arguments.get("wait_for").and_then(|v| v.as_str());
                self.navigate(url, wait_for).await
            },
            "screenshot" => self.screenshot().await,
            "click" => {
                let selector = tool_call.arguments.get("selector")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AgentError::InvalidParameters("Missing 'selector' parameter".into()))?;
                let wait_for = tool_call.arguments.get("wait_for").and_then(|v| v.as_str());
                self.click(selector, wait_for).await
            },
            "type" => {
                let selector = tool_call.arguments.get("selector")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AgentError::InvalidParameters("Missing 'selector' parameter".into()))?;
                let text = tool_call.arguments.get("text")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AgentError::InvalidParameters("Missing 'text' parameter".into()))?;
                let clear_first = tool_call.arguments.get("clear_first").and_then(|v| v.as_bool());
                self.type_text(selector, text, clear_first).await
            },
            "eval" => {
                let script = tool_call.arguments.get("script")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AgentError::InvalidParameters("Missing 'script' parameter".into()))?;
                self.eval(script).await
            },
            "wait_for" => {
                let selector = tool_call.arguments.get("selector")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AgentError::InvalidParameters("Missing 'selector' parameter".into()))?;
                let timeout = tool_call.arguments.get("timeout").and_then(|v| v.as_u64());
                self.wait_for(selector, timeout).await
            },
            "get_text" => self.get_text().await,
            _ => Err(AgentError::ToolNotFound(tool_call.name)),
        }
    }

    async fn read_resource(&self, _uri: &str) -> AgentResult<String> {
        Err(AgentError::InvalidParameters("Resource reading not supported".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};


    #[tokio::test]
    async fn test_navigate_and_screenshot() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><h1>Test Page</h1></body></html>"#))
            .mount(&mock_server)
            .await;

        let system = WebBrowserSystem::new();
        
        // Navigate to mock server
        let navigate_result = system.call(ToolCall::new(
            "navigate",
            json!({
                "url": mock_server.uri(),
            })
        )).await.unwrap();

        assert!(navigate_result[0].as_text().unwrap().contains("Navigated to"));

        // Take a screenshot
        let screenshot_result = system.call(ToolCall::new(
            "screenshot",
            json!({})
        )).await.unwrap();

        assert!(screenshot_result[0].as_image().is_some());
    }

    #[tokio::test]
    async fn test_get_text() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><h1>Test Page</h1><p>This is some test content.</p></body></html>"#))
            .mount(&mock_server)
            .await;

        let system = WebBrowserSystem::new();
        
        // Navigate to mock server
        let _ = system.call(ToolCall::new(
            "navigate",
            json!({
                "url": mock_server.uri(),
            })
        )).await.unwrap();

        // Get text content
        let text_result = system.call(ToolCall::new(
            "get_text",
            json!({})
        )).await.unwrap();

        let result_text = text_result[0].as_text().unwrap();
        assert!(result_text.contains("Page content saved to:"));
        
        // Extract the file path from the result
        let file_path = result_text.split(": ").nth(1).unwrap();
        
        // Verify the file exists and contains the expected content
        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(content.contains("Test Page"));
        assert!(content.contains("This is some test content."));
    }
}