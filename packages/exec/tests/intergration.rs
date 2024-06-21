use {
    wasm_bindgen::prelude::*,
    wasm_bindgen_test::{console_log, wasm_bindgen_test},
};

macro_rules! prop_builder {
    ($obj:block$(.$setter:ident($value:expr))*) => {{
        let tmp = $obj;
        $(tmp.$setter($value);)*
        tmp
    }};
}

#[wasm_bindgen_test]
async fn test_exec_bindings() {
    use actions_exec::{Env, ExecListeners, ExecOptions, ExecOutput};

    let print_stdout = Closure::new(|str: js_sys::JsString| {
        console_log!("{str:?}");
    });
    let print_stderr = Closure::new(|str: js_sys::JsString| {
        console_log!("Error: {str:?}");
    });
    let print_debug = Closure::new(|str: js_sys::JsString| {
        console_log!("Debug: {str:?}");
    });

    let env = Env::default();
    env.set("ENV_VAR", "Value");
    assert_eq!(
        actions_exec::exec(
            "/usr/bin/bash",
            Some(vec![
                "--noprofile".to_owned(),
                "--norc".to_owned(),
                "-c".to_owned(),
                "echo \"ENV_VAR: \\\"${ENV_VAR}\\\"\"".to_owned(),
            ]),
            Some(prop_builder! { { ExecOptions::default() }
                        .set_cwd(Some(env!("CARGO_TARGET_TMPDIR").to_string()))
                        .set_env(Some(env))
                        .set_silent(Some(false))
                        .set_out_stream(None)
                        .set_err_stream(None)
                        .set_windows_verbatim_arguments(Some(false))
                        .set_fail_on_std_err(Some(true))
                        .set_ignore_return_code(Some(false))
                        .set_delay(Some(10000))
                        .set_input(None)
                        .set_listeners(Some(prop_builder! {
                            { ExecListeners::default() }
                                .set_stdline(&print_stdout)
                                .set_errline(&print_stderr)
                                .set_debug(&print_debug)
                        }))
            }),
        )
        .await
        .map(|res| res.as_f64()),
        Ok(Some(0f64))
    );

    let env = Env::default();
    env.set("ENV_VAR", "Value");
    assert_eq!(
        actions_exec::get_exec_output(
            "/usr/bin/bash",
            Some(vec![
                "--noprofile".to_owned(),
                "--norc".to_owned(),
                "-c".to_owned(),
                "echo \"ENV_VAR: \\\"${ENV_VAR}\\\"\"".to_owned(),
            ]),
            Some(prop_builder! { { ExecOptions::default() }
                        .set_cwd(Some(env!("CARGO_TARGET_TMPDIR").to_string()))
                        .set_env(Some(env))
                        .set_silent(Some(false))
                        .set_out_stream(None)
                        .set_err_stream(None)
                        .set_windows_verbatim_arguments(Some(false))
                        .set_fail_on_std_err(Some(true))
                        .set_ignore_return_code(Some(false))
                        .set_delay(Some(10000))
                        .set_input(None)
                        .set_listeners(Some(prop_builder! {
                            { ExecListeners::default() }
                                .set_stdline(&print_stdout)
                                .set_errline(&print_stderr)
                                .set_debug(&print_debug)
                        }))
            }),
        )
        .await
        .map(Into::into),
        Ok(prop_builder! {
            { ExecOutput::default() }
                .set_exit_code(0)
                .set_stdout("ENV_VAR: \"Value\"\n".to_string())
                .set_stderr("".to_string())
        })
    );
}
