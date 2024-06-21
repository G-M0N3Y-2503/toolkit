use {wasm_bindgen::prelude::*, wasm_bindgen_test::wasm_bindgen_test};

macro_rules! prop_builder {
    ($obj:block$(.$setter:ident($value:expr))*) => {{
        let tmp = $obj;
        $(tmp.$setter($value);)*
        tmp
    }};
}

#[wasm_bindgen_test]
async fn test_core_bindings() {
    use actions_core::{
        summary::{SummaryImageOptions, SummaryTableCell, SummaryTableRow, SummaryWriteOptions},
        AnnotationProperties,
    };
    use js_sys::{Error, Promise};

    actions_core::export_variable("name", "val".into());
    actions_core::set_secret("secret");
    actions_core::add_path("inputPath");
    actions_core::get_input(
        "name",
        Some(prop_builder! { { actions_core::InputOptions::default() }.set_required(Some(true)).set_trim_whitespace(Some(true))}),
    )
    .unwrap_err();
    actions_core::get_multiline_input(
        "name",
        Some(prop_builder! { { actions_core::InputOptions::default() }.set_required(Some(true)).set_trim_whitespace(Some(true))}),
    )
    .unwrap_err();
    actions_core::get_boolean_input(
        "name",
        Some(prop_builder! { { actions_core::InputOptions::default() }.set_required(Some(true)).set_trim_whitespace(Some(true))}),
    )
    .unwrap_err();
    actions_core::set_output("name", "value".into());
    actions_core::set_command_echo(true);
    assert!(!actions_core::is_debug());
    actions_core::debug("message");
    actions_core::error(
        Error::new("message").into(),
        Some(prop_builder! { { AnnotationProperties::default() }
            .set_title(Some("title".to_owned()))
            .set_file(Some(file!().to_owned()))
            .set_start_line(Some(1))
            .set_end_line(None)
            .set_start_column(Some(1))
            .set_end_column(Some(2))
        }),
    );
    actions_core::warning(
        Error::new("message").into(),
        Some(prop_builder! { { AnnotationProperties::default() }
            .set_title(Some("title".to_owned()))
            .set_file(Some(file!().to_owned()))
            .set_start_line(Some(1))
            .set_end_line(None)
            .set_start_column(Some(1))
            .set_end_column(Some(2))
        }),
    );
    actions_core::notice(
        "message".into(),
        Some(prop_builder! { { AnnotationProperties::default() }
            .set_title(Some("title".to_owned()))
            .set_file(Some(file!().to_owned()))
            .set_start_line(Some(1))
            .set_end_line(None)
            .set_start_column(Some(1))
            .set_end_column(Some(2))
        }),
    );
    actions_core::info("message");
    actions_core::start_group("name");
    actions_core::end_group();
    assert_eq!(
        actions_core::group("name", &mut || Promise::new(&mut |resolve, _reject| {
            resolve.call1(&JsValue::undefined(), &true.into()).unwrap();
        }))
        .await
        .unwrap(),
        JsValue::from(true)
    );
    if option_env!("GITHUB_ACTIONS").is_some() {
        assert_eq!(
            actions_core::group("name", &mut || Promise::new(&mut |_resolve, reject| {
                reject.call1(&JsValue::undefined(), &true.into()).unwrap();
            }))
            .await
            .unwrap_err(),
            JsValue::from(true)
        );
    }
    actions_core::save_state("name", "value".into());
    assert_eq!(actions_core::get_state("name"), "");
    actions_core::get_id_token(Some("aud")).await.unwrap_err();

    actions_core::SUMMARY
        .write(Some(
            prop_builder! { { SummaryWriteOptions::default() }.set_overwrite(Some(true))},
        ))
        .await
        .unwrap_err();
    actions_core::SUMMARY.clear().await.unwrap_err();
    assert_eq!(actions_core::SUMMARY.stringify(), "");
    assert!(actions_core::SUMMARY.is_empty_buffer());
    actions_core::SUMMARY.add_raw("text", Some(true));
    actions_core::SUMMARY.empty_buffer();
    actions_core::SUMMARY.add_eol();
    actions_core::SUMMARY.add_code_block("code", Some("lang"));
    actions_core::SUMMARY.add_list(
        vec!["1".to_owned(), "2".to_owned(), "3".to_owned()],
        Some(true),
    );
    actions_core::SUMMARY.add_table(vec![
        SummaryTableRow::from_iter(vec![prop_builder! { { SummaryTableCell::default() }
                .set_data("Title".to_owned())
                .set_header(Some(true))
                .set_colspan(Some(2))
                .set_rowspan(Some(2))
        }]),
        SummaryTableRow::from_iter(vec![
            prop_builder! {
                { SummaryTableCell::default() }
                    .set_data("data1".to_owned())
                    .set_header(None)
                    .set_colspan(None)
                    .set_rowspan(None)
            },
            prop_builder! {
                { SummaryTableCell::default() }
                    .set_data("data2".to_owned())
                    .set_header(None)
                    .set_colspan(None)
                    .set_rowspan(None)
            },
        ]),
    ]);
    actions_core::SUMMARY.add_details("label", "content");
    actions_core::SUMMARY.add_image(
        "src",
        "alt",
        Some(prop_builder! {
            { SummaryImageOptions::default() }
                .set_width(Some(500))
                .set_height(Some(500))
        }),
    );
    actions_core::SUMMARY.add_heading("text", Some(4));
    actions_core::SUMMARY.add_separator();
    actions_core::SUMMARY.add_break();
    actions_core::SUMMARY.add_quote("text", Some("cite"));
    actions_core::SUMMARY.add_link("text", "href");

    assert_eq!(
        String::from(actions_core::to_platform_path("pth")),
        "pth".to_owned()
    );
    assert_eq!(
        String::from(actions_core::to_posix_path("pth")),
        "pth".to_owned()
    );
    assert_eq!(
        String::from(actions_core::to_win32_path("pth")),
        "pth".to_owned()
    );
}

#[wasm_bindgen_test]
#[ignore = "Causes processes to exit with 1"]
fn test_core_set_failed_bindings() {
    use js_sys::Error;

    actions_core::set_failed(Error::new("message"));
}
