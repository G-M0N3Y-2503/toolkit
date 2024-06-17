use {super::*, js_sys::Reflect};

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type SummaryTableCell;

    /// Cell content
    #[wasm_bindgen(method, getter)]
    pub fn data(this: &SummaryTableCell) -> String;
    /// Cell content
    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &SummaryTableCell, data: String);

    /// Render cell as header
    /// (optional) default: false
    #[wasm_bindgen(method, getter)]
    pub fn header(this: &SummaryTableCell) -> Option<bool>;
    /// Render cell as header
    /// (optional) default: false
    #[wasm_bindgen(method, setter)]
    pub fn set_header(this: &SummaryTableCell, header: Option<bool>);

    /// Number of columns the cell extends
    /// (optional) default: '1'
    #[wasm_bindgen(method, getter)]
    pub fn colspan(this: &SummaryTableCell) -> Option<usize>;
    /// Number of columns the cell extends
    /// (optional) default: '1'
    #[wasm_bindgen(method, setter)]
    pub fn set_colspan(this: &SummaryTableCell, colspan: Option<usize>);

    /// Number of rows the cell extends
    /// (optional) default: '1'
    #[wasm_bindgen(method, getter)]
    pub fn rowspan(this: &SummaryTableCell) -> Option<usize>;
    /// Number of rows the cell extends
    /// (optional) default: '1'
    #[wasm_bindgen(method, setter)]
    pub fn set_rowspan(this: &SummaryTableCell, rowspan: Option<usize>);
}
impl Default for SummaryTableCell {
    fn default() -> Self {
        let obj = js_sys::Object::new();
        let _ = Reflect::set(&obj, &JsValue::from("data"), &JsValue::UNDEFINED)
            .expect("property should be set successfully");
        Self::unchecked_from_js(JsValue::from(obj))
    }
}

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type SummaryImageOptions;

    /// The width of the image in pixels. Must be an integer without a unit.
    /// (optional)
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &SummaryImageOptions) -> Option<usize>;
    /// The width of the image in pixels. Must be an integer without a unit.
    /// (optional)
    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &SummaryImageOptions, width: Option<usize>);

    /// The height of the image in pixels. Must be an integer without a unit.
    /// (optional)
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &SummaryImageOptions) -> Option<usize>;
    /// The height of the image in pixels. Must be an integer without a unit.
    /// (optional)
    #[wasm_bindgen(method, setter)]
    pub fn set_height(this: &SummaryImageOptions, height: Option<usize>);
}
impl Default for SummaryImageOptions {
    fn default() -> Self {
        // all keys are optional so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type SummaryWriteOptions;

    /// Replace all existing content in summary file with buffer contents
    /// (optional) default: false
    #[wasm_bindgen(method, getter)]
    pub fn overwrite(this: &SummaryWriteOptions) -> Option<bool>;
    /// Replace all existing content in summary file with buffer contents
    /// (optional) default: false
    #[wasm_bindgen(method, setter)]
    pub fn set_overwrite(this: &SummaryWriteOptions, overwrite: Option<bool>);
}
impl Default for SummaryWriteOptions {
    fn default() -> Self {
        // all keys are optional so we just create an empty object
        Self::unchecked_from_js(JsValue::from(js_sys::Object::new()))
    }
}

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type SummaryTableRow;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &SummaryTableRow, prop: usize) -> SummaryTableCell;

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set(this: &SummaryTableRow, prop: usize, val: SummaryTableCell);

    #[wasm_bindgen(method, structural, indexing_deleter)]
    fn delete(this: &SummaryTableRow, prop: usize);
}
impl Default for SummaryTableRow {
    fn default() -> Self {
        Self::unchecked_from_js(JsValue::from(js_sys::Array::new()))
    }
}
impl FromIterator<SummaryTableCell> for SummaryTableRow {
    fn from_iter<I: IntoIterator<Item = SummaryTableCell>>(cells: I) -> Self {
        let row = Self::default();
        for (index, cell) in cells.into_iter().enumerate() {
            row.set(index, cell);
        }
        row
    }
}

#[wasm_bindgen(module = "@actions/core")]
extern "C" {
    #[allow(clippy::empty_docs)]
    #[wasm_bindgen(no_deref)] // TODO: https://github.com/rustwasm/wasm-bindgen/issues/3964
    pub type Summary;

    #[allow(clippy::empty_docs)]
    #[wasm_bindgen(js_name = "summary")]
    pub static SUMMARY: Summary;
    /// Writes text in the buffer to the summary buffer file and empties buffer. Will append by default.
    ///
    /// @param {SummaryWriteOptions} [options] (optional) options for write operation
    ///
    /// @returns {Promise<Summary>} summary instance
    #[wasm_bindgen(method, catch)]
    pub async fn write(
        this: &Summary,
        options: Option<SummaryWriteOptions>,
    ) -> Result<JsValue, JsValue>;
    /// Clears the summary buffer and wipes the summary file
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, catch)]
    pub async fn clear(this: &Summary) -> Result<JsValue, JsValue>;
    /// Returns the current summary buffer as a string
    ///
    /// @returns {string} string of summary buffer
    #[wasm_bindgen(method)]
    pub fn stringify(this: &Summary) -> JsString;
    /// If the summary buffer is empty
    ///
    /// @returns {boolean} true if the buffer is empty
    #[wasm_bindgen(method, js_name = "isEmptyBuffer")]
    pub fn is_empty_buffer(this: &Summary) -> bool;
    /// Resets the summary buffer without writing to summary file
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "emptyBuffer")]
    pub fn empty_buffer(this: &Summary) -> Summary;
    /// Adds raw text to the summary buffer
    ///
    /// @param {string} text content to add
    /// @param {boolean} [addEOL=false] (optional) append an EOL to the raw text (default: false)
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addRaw")]
    pub fn add_raw(this: &Summary, text: &str, add_eol: Option<bool>) -> Summary;
    /// Adds the operating system-specific end-of-line marker to the buffer
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addEOL")]
    pub fn add_eol(this: &Summary) -> Summary;
    /// Adds an HTML codeblock to the summary buffer
    ///
    /// @param {string} code content to render within fenced code block
    /// @param {string} lang (optional) language to syntax highlight code
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addCodeBlock")]
    pub fn add_code_block(this: &Summary, code: &str, lang: Option<&str>) -> Summary;
    /// Adds an HTML list to the summary buffer
    ///
    /// @param {string[]} items list of items to render
    /// @param {boolean} [ordered=false] (optional) if the rendered list should be ordered or not (default: false)
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addList")]
    pub fn add_list(this: &Summary, items: Vec<String>, ordered: Option<bool>) -> Summary;
    /// Adds an HTML table to the summary buffer
    ///
    /// @param {SummaryTableCell[]} rows table rows
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addTable")]
    pub fn add_table(this: &Summary, summary_table: Vec<SummaryTableRow>) -> Summary;
    /// Adds a collapsable HTML details element to the summary buffer
    ///
    /// @param {string} label text for the closed state
    /// @param {string} content collapsable content
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addDetails")]
    pub fn add_details(this: &Summary, label: &str, content: &str) -> Summary;
    /// Adds an HTML image tag to the summary buffer
    ///
    /// @param {string} src path to the image you to embed
    /// @param {string} alt text description of the image
    /// @param {SummaryImageOptions} options (optional) addition image attributes
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addImage")]
    pub fn add_image(
        this: &Summary,
        src: &str,
        alt: &str,
        options: Option<SummaryImageOptions>,
    ) -> Summary;
    /// Adds an HTML section heading element
    ///
    /// @param {string} text heading text
    /// @param {number | string} [level=1] (optional) the heading level, default: 1
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addHeading")]
    pub fn add_heading(this: &Summary, text: &str, level: Option<usize>) -> Summary;
    /// Adds an HTML thematic break (<hr>) to the summary buffer
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addSeparator")]
    pub fn add_separator(this: &Summary) -> Summary;
    /// Adds an HTML line break (<br>) to the summary buffer
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addBreak")]
    pub fn add_break(this: &Summary) -> Summary;
    /// Adds an HTML blockquote to the summary buffer
    ///
    /// @param {string} text quote text
    /// @param {string} cite (optional) citation url
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addQuote")]
    pub fn add_quote(this: &Summary, text: &str, cite: Option<&str>) -> Summary;
    /// Adds an HTML anchor tag to the summary buffer
    ///
    /// @param {string} text link text/content
    /// @param {string} href hyperlink
    ///
    /// @returns {Summary} summary instance
    #[wasm_bindgen(method, js_name = "addLink")]
    pub fn add_link(this: &Summary, text: &str, href: &str) -> Summary;
}
