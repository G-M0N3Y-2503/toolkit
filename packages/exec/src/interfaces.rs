use {
    super::*,
    js_sys::{JsString, Reflect},
};

#[wasm_bindgen]
extern "C" {
    /// Node JS Buffer type
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type Buffer;

    /// Node JS stream.Writable type
    #[wasm_bindgen(no_deref, js_namespace = stream)]
    // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type Writable;
}

#[wasm_bindgen(module = "@actions/exec")]
extern "C" {
    /// Interface for exec options
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type ExecOptions;

    /// optional working directory.
    /// defaults to current
    #[wasm_bindgen(method, getter)]
    pub fn cwd(this: &ExecOptions) -> Option<String>;
    /// optional working directory.
    /// defaults to current
    #[wasm_bindgen(method, setter)]
    pub fn set_cwd(this: &ExecOptions, cwd: Option<String>);

    /// optional envvar dictionary.
    /// defaults to current process's env
    #[wasm_bindgen(method, getter)]
    pub fn env(this: &ExecOptions) -> Option<Env>;
    /// optional envvar dictionary.
    /// defaults to current process's env
    #[wasm_bindgen(method, setter)]
    pub fn set_env(this: &ExecOptions, env: Option<Env>);

    /// optional.
    /// defaults to false
    #[wasm_bindgen(method, getter)]
    pub fn silent(this: &ExecOptions) -> Option<bool>;
    /// optional.
    /// defaults to false
    #[wasm_bindgen(method, setter)]
    pub fn set_silent(this: &ExecOptions, silent: Option<bool>);

    /// optional out stream to use. Defaults to process.stdout
    #[wasm_bindgen(method, getter = outStream)]
    pub fn out_stream(this: &ExecOptions) -> Option<Writable>;
    /// optional out stream to use. Defaults to process.stdout
    #[wasm_bindgen(method, setter = outStream)]
    pub fn set_out_stream(this: &ExecOptions, out_stream: Option<Writable>);

    /// optional err stream to use. Defaults to process.stderr
    #[wasm_bindgen(method, getter = errStream)]
    pub fn err_stream(this: &ExecOptions) -> Option<Writable>;
    /// optional err stream to use. Defaults to process.stderr
    #[wasm_bindgen(method, setter = errStream)]
    pub fn set_err_stream(this: &ExecOptions, err_stream: Option<Writable>);

    /// optional. whether to skip quoting/escaping arguments if needed.
    /// defaults to false.
    #[wasm_bindgen(method, getter = windowsVerbatimArguments)]
    pub fn windows_verbatim_arguments(this: &ExecOptions) -> Option<bool>;
    /// optional. whether to skip quoting/escaping arguments if needed.
    /// defaults to false.
    #[wasm_bindgen(method, setter = windowsVerbatimArguments)]
    pub fn set_windows_verbatim_arguments(
        this: &ExecOptions,
        windowsVerbatimArguments: Option<bool>,
    );

    /// optional.
    /// whether to fail if output to stderr.
    /// defaults to false
    #[wasm_bindgen(method, getter = failOnStdErr)]
    pub fn fail_on_std_err(this: &ExecOptions) -> Option<bool>;
    /// optional.
    /// whether to fail if output to stderr.
    /// defaults to false
    #[wasm_bindgen(method, setter = failOnStdErr)]
    pub fn set_fail_on_std_err(this: &ExecOptions, fail_on_std_err: Option<bool>);

    /// optional.
    /// defaults to failing on non zero.
    /// ignore will not fail leaving it up to the caller
    #[wasm_bindgen(method, getter = ignoreReturnCode)]
    pub fn ignore_return_code(this: &ExecOptions) -> Option<bool>;
    /// optional.
    /// defaults to failing on non zero.
    /// ignore will not fail leaving it up to the caller
    #[wasm_bindgen(method, setter = ignoreReturnCode)]
    pub fn set_ignore_return_code(this: &ExecOptions, ignore_return_code: Option<bool>);

    /// optional. How long in ms to wait for STDIO streams to close after the exit event of the process before terminating. defaults to 10000
    #[wasm_bindgen(method, getter)]
    pub fn delay(this: &ExecOptions) -> Option<usize>;
    /// optional. How long in ms to wait for STDIO streams to close after the exit event of the process before terminating. defaults to 10000
    #[wasm_bindgen(method, setter)]
    pub fn set_delay(this: &ExecOptions, delay: Option<usize>);

    /// optional. input to write to the process on STDIN.
    #[wasm_bindgen(method, getter)]
    pub fn input(this: &ExecOptions) -> Option<Buffer>;
    /// optional. input to write to the process on STDIN.
    #[wasm_bindgen(method, setter)]
    pub fn set_input(this: &ExecOptions, input: Option<Buffer>);

    /// optional. Listeners for output. Callback functions that will be called on these events
    #[wasm_bindgen(method, getter)]
    pub fn listeners(this: &ExecOptions) -> Option<ExecListeners>;
    /// optional. Listeners for output. Callback functions that will be called on these events
    #[wasm_bindgen(method, setter)]
    pub fn set_listeners(this: &ExecOptions, listeners: Option<ExecListeners>);
}
impl Default for ExecOptions {
    fn default() -> Self {
        // all keys are optional so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}

#[wasm_bindgen]
extern "C" {
    /// Environment Variable Map
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type Env;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Env, key: &str) -> js_sys::JsString;

    #[wasm_bindgen(method, structural, indexing_setter)]
    pub fn set(this: &Env, key: &str, val: &str);

    #[wasm_bindgen(method, structural, indexing_deleter)]
    pub fn delete(this: &Env, key: &str);
}
impl Default for Env {
    fn default() -> Self {
        // Has no keys so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}

#[wasm_bindgen(module = "@actions/exec")]
extern "C" {
    /// Interface for the output of getExecOutput()
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type ExecOutput;

    /// The exit code of the process
    #[wasm_bindgen(method, getter = exitCode)]
    pub fn exit_code(this: &ExecOutput) -> isize;
    /// The exit code of the process
    #[wasm_bindgen(method, setter = exitCode)]
    pub fn set_exit_code(this: &ExecOutput, exit_code: isize);

    /// The entire stdout of the process as a string
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &ExecOutput) -> String;
    /// The entire stdout of the process as a string
    #[wasm_bindgen(method, setter)]
    pub fn set_stdout(this: &ExecOutput, stdout: String);

    /// The entire stderr of the process as a string
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &ExecOutput) -> String;
    /// The entire stderr of the process as a string
    #[wasm_bindgen(method, setter)]
    pub fn set_stderr(this: &ExecOutput, stderr: String);
}
impl Default for ExecOutput {
    fn default() -> Self {
        let obj = js_sys::Object::new();
        let _ = Reflect::set(&obj, &JsValue::from("exitCode"), &JsValue::UNDEFINED)
            .expect("property should be set successfully");
        let _ = Reflect::set(&obj, &JsValue::from("stdout"), &JsValue::UNDEFINED)
            .expect("property should be set successfully");
        let _ = Reflect::set(&obj, &JsValue::from("stderr"), &JsValue::UNDEFINED)
            .expect("property should be set successfully");
        Self::unchecked_from_js(JsValue::from(obj))
    }
}
impl PartialEq for ExecOutput {
    fn eq(&self, other: &Self) -> bool {
        self.exit_code() == other.exit_code()
            && self.stdout() == other.stdout()
            && self.stderr() == other.stderr()
    }
}
impl core::fmt::Debug for ExecOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecOutput")
            .field("exit_code", &self.exit_code())
            .field("stdout", &self.stdout())
            .field("stderr", &self.stderr())
            .finish()
    }
}

#[wasm_bindgen(module = "@actions/exec")]
extern "C" {
    /// The user defined listeners for an exec call
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type ExecListeners;

    /// A call back for each buffer of stdout
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &ExecListeners) -> JsValue;
    /// A call back for each buffer of stdout
    #[wasm_bindgen(method, setter)]
    pub fn set_stdout(this: &ExecListeners, stdout: &Closure<dyn FnMut(Buffer)>);

    /// A call back for each buffer of stderr
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &ExecListeners) -> JsValue;
    /// A call back for each buffer of stderr
    #[wasm_bindgen(method, setter)]
    pub fn set_stderr(this: &ExecListeners, stderr: &Closure<dyn FnMut(Buffer)>);

    /// A call back for each line of stdout
    #[wasm_bindgen(method, getter)]
    pub fn stdline(this: &ExecListeners) -> JsValue;
    /// A call back for each line of stdout
    #[wasm_bindgen(method, setter)]
    pub fn set_stdline(this: &ExecListeners, stdline: &Closure<dyn FnMut(JsString)>);

    /// A call back for each line of stderr
    #[wasm_bindgen(method, getter)]
    pub fn errline(this: &ExecListeners) -> JsValue;
    /// A call back for each line of stderr
    #[wasm_bindgen(method, setter)]
    pub fn set_errline(this: &ExecListeners, errline: &Closure<dyn FnMut(JsString)>);

    /// A call back for each debug log
    #[wasm_bindgen(method, getter)]
    pub fn debug(this: &ExecListeners) -> JsValue;
    /// A call back for each debug log
    #[wasm_bindgen(method, setter)]
    pub fn set_debug(this: &ExecListeners, debug: &Closure<dyn FnMut(JsString)>);
}
impl Default for ExecListeners {
    fn default() -> Self {
        // all keys are optional so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}
