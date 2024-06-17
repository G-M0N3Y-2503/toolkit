use js_sys::{Error, JsString, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    /// Interface for getInput options
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type InputOptions;

    /// Optional. Whether the input is required. If required and not present, will throw. Defaults to false
    #[wasm_bindgen(method, getter)]
    pub fn required(this: &InputOptions) -> Option<bool>;

    /// Optional. Whether the input is required. If required and not present, will throw. Defaults to false
    #[wasm_bindgen(method, setter)]
    pub fn set_required(this: &InputOptions, required: Option<bool>);
    /// Optional. Whether leading/trailing whitespace will be trimmed for the input. Defaults to true
    #[wasm_bindgen(method, getter = trimWhitespace)]
    pub fn trim_whitespace(this: &InputOptions) -> Option<bool>;

    /// Optional. Whether leading/trailing whitespace will be trimmed for the input. Defaults to true
    #[wasm_bindgen(method, setter = trimWhitespace)]
    pub fn set_trim_whitespace(this: &InputOptions, trim_whitespace: Option<bool>);
}
impl Default for InputOptions {
    fn default() -> Self {
        // all keys are optional so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}

/// The code to exit an action*
#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ExitCode {
    /// A code indicating that the action was successful
    Success = 0,
    /// A code indicating that the action was a failure
    Failure = 1,
}

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    /// Optional properties that can be sent with annotation commands (notice, error, and warning)
    /// See: https://docs.github.com/en/rest/reference/checks#create-a-check-run for more information about annotations.
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type AnnotationProperties;

    /// A title for the annotation.
    #[wasm_bindgen(method, getter)]
    pub fn title(this: &AnnotationProperties) -> Option<String>;

    /// A title for the annotation.
    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &AnnotationProperties, title: Option<String>);
    /// The path of the file for which the annotation should be created.
    #[wasm_bindgen(method, getter)]
    pub fn file(this: &AnnotationProperties) -> Option<String>;

    /// The path of the file for which the annotation should be created.
    #[wasm_bindgen(method, setter)]
    pub fn set_file(this: &AnnotationProperties, file: Option<String>);
    /// The start line for the annotation.
    #[wasm_bindgen(method, getter = startLine)]
    pub fn start_line(this: &AnnotationProperties) -> Option<usize>;

    /// The start line for the annotation.
    #[wasm_bindgen(method, setter = startLine)]
    pub fn set_start_line(this: &AnnotationProperties, start_line: Option<usize>);

    /// The end line for the annotation. Defaults to `startLine` when `startLine` is provided.
    #[wasm_bindgen(method, getter = endLine)]
    pub fn end_line(this: &AnnotationProperties) -> Option<usize>;

    /// The end line for the annotation. Defaults to `startLine` when `startLine` is provided.
    #[wasm_bindgen(method, setter = endLine)]
    pub fn set_end_line(this: &AnnotationProperties, end_line: Option<usize>);
    /// The start column for the annotation. Cannot be sent when `startLine` and `endLine` are different values.
    #[wasm_bindgen(method, getter = startColumn)]
    pub fn start_column(this: &AnnotationProperties) -> Option<usize>;

    /// The start column for the annotation. Cannot be sent when `startLine` and `endLine` are different values.
    #[wasm_bindgen(method, setter = startColumn)]
    pub fn set_start_column(this: &AnnotationProperties, start_column: Option<usize>);

    /// The end column for the annotation. Cannot be sent when `startLine` and `endLine` are different values.
    /// Defaults to `startColumn` when `startColumn` is provided.
    #[wasm_bindgen(method, getter = endColumn)]
    pub fn end_column(this: &AnnotationProperties) -> Option<usize>;

    /// The end column for the annotation. Cannot be sent when `startLine` and `endLine` are different values.
    /// Defaults to `startColumn` when `startColumn` is provided.
    #[wasm_bindgen(method, setter = endColumn)]
    pub fn set_end_column(this: &AnnotationProperties, end_column: Option<usize>);
}
impl Default for AnnotationProperties {
    fn default() -> Self {
        // all keys are optional so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    /// Sets env variable for this action and future actions in the job
    /// @param name the name of the variable to set
    /// @param val the value of the variable. Non-string values will be converted to a string via JSON.stringify
    #[wasm_bindgen(js_name = "exportVariable")]
    pub fn export_variable(name: &str, val: JsValue);
    /// Registers a secret which will get masked from logs
    /// @param secret value of the secret
    #[wasm_bindgen(js_name = "setSecret")]
    pub fn set_secret(secret: &str);
    /// Prepends inputPath to the PATH (for this action and future actions)
    /// @param inputPath
    #[wasm_bindgen(js_name = "addPath")]
    pub fn add_path(input_path: &str);
    /// Gets the value of an input.
    /// Unless trimWhitespace is set to false in InputOptions, the value is also trimmed.
    /// Returns an empty string if the value is not defined.
    ///
    /// @param     name     name of the input to get
    /// @param     options  optional. See InputOptions.
    /// @returns   string
    #[wasm_bindgen(catch, js_name = "getInput")]
    pub fn get_input(name: &str, options: Option<InputOptions>) -> Result<JsString, JsValue>;
    /// Gets the values of an multiline input.
    /// Each value is also trimmed.
    ///
    /// @param     name     name of the input to get
    /// @param     options  optional. See InputOptions.
    /// @returns   string[]
    ///
    #[wasm_bindgen(catch, js_name = "getMultilineInput")]
    pub fn get_multiline_input(
        name: &str,
        options: Option<InputOptions>,
    ) -> Result<Vec<JsString>, JsValue>;
    /// Gets the input value of the boolean type in the YAML 1.2 "core schema" specification.
    /// Support boolean input list: `true | True | TRUE | false | False | FALSE` .
    /// The return value is also in boolean type.
    /// ref: https://yaml.org/spec/1.2/spec.html#id2804923
    ///
    /// @param     name     name of the input to get
    /// @param     options  optional. See InputOptions.
    /// @returns   boolean
    #[wasm_bindgen(catch, js_name = "getBooleanInput")]
    pub fn get_boolean_input(name: &str, options: Option<InputOptions>) -> Result<bool, JsValue>;
    /// Sets the value of an output.
    ///
    /// @param     name     name of the output to set
    /// @param     value    value to store. Non-string values will be converted to a string via JSON.stringify
    #[wasm_bindgen(js_name = "setOutput")]
    pub fn set_output(name: &str, value: JsValue);
    /// Enables or disables the echoing of commands into stdout for the rest of the step.
    /// Echoing is disabled by default if ACTIONS_STEP_DEBUG is not set.
    ///
    #[wasm_bindgen(js_name = "setCommandEcho")]
    pub fn set_command_echo(enabled: bool);
    /// Sets the action status to failed.
    /// When the action exits it will be with an exit code of 1
    /// @param message add error issue message
    #[wasm_bindgen(js_name = "setFailed")]
    pub fn set_failed(message: Error);
    /// Gets whether Actions Step Debug is on or not
    #[wasm_bindgen(js_name = "isDebug")]
    pub fn is_debug() -> bool;
    /// Writes debug message to user log
    /// @param message debug message
    pub fn debug(message: &str);
    /// Adds an error issue
    /// @param message error issue message. Errors will be converted to string via toString()
    /// @param properties optional properties to add to the annotation.
    pub fn error(message: JsValue, properties: Option<AnnotationProperties>);
    /// Adds a warning issue
    /// @param message warning issue message. Errors will be converted to string via toString()
    /// @param properties optional properties to add to the annotation.
    pub fn warning(message: JsValue, properties: Option<AnnotationProperties>);
    /// Adds a notice issue
    /// @param message notice issue message. Errors will be converted to string via toString()
    /// @param properties optional properties to add to the annotation.
    pub fn notice(message: JsValue, properties: Option<AnnotationProperties>);
    /// Writes info to log with console.log.
    /// @param message info message
    pub fn info(message: &str);
    /// Begin an output group.
    ///
    /// Output until the next `groupEnd` will be foldable in this group
    ///
    /// @param name The name of the output group
    #[wasm_bindgen(js_name = "startGroup")]
    pub fn start_group(name: &str);
    /// End an output group.
    #[wasm_bindgen(js_name = "endGroup")]
    pub fn end_group();
    /// Wrap an asynchronous function call in a group.
    ///
    /// Returns the same type as the function itself.
    ///
    /// @param name The name of the group
    /// @param fn The function to wrap in the group
    #[wasm_bindgen(catch)]
    pub async fn group(name: &str, r#fn: &mut dyn FnMut() -> Promise) -> Result<JsValue, JsValue>;
    /// Saves state for current action, the state can only be retrieved by this action's post job execution.
    ///
    /// @param     name     name of the state to store
    /// @param     value    value to store. Non-string values will be converted to a string via JSON.stringify
    #[wasm_bindgen(js_name = "saveState")]
    pub fn save_state(name: &str, value: JsValue);
    /// Gets the value of an state set by this action's main execution.
    ///
    /// @param     name     name of the state to get
    /// @returns   string
    #[wasm_bindgen(js_name = "getState")]
    pub fn get_state(name: &str) -> JsString;
    /// You can use these methods to interact with the GitHub OIDC provider and get a JWT ID token which would help to get access token from third party cloud providers.
    /// @returns Result<string, JsValue>
    #[wasm_bindgen(catch, js_name = "getIDToken")]
    pub async fn get_id_token(audience: Option<&str>) -> Result<JsValue, JsValue>;
}

/// Summary exports
pub mod summary;
pub use summary::SUMMARY;
/// Path exports
pub mod path_utils;
pub use path_utils::{to_platform_path, to_posix_path, to_win32_path};
