use std::path::PathBuf;

extern crate rust_ffi;

fn main() {
    let mut rust_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rust_path.pop();
    let archive_path: String =String::from(rust_path.to_string_lossy());
    println!("cargo:rustc-link-lib=static=xml2");
    println!("cargo:rustc-link-search=native={}", archive_path);

    /**
     * parser.c
     **/
    rust_ffi::get_parser_LIBXML_XINCLUDE_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_THREAD_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_HTML_ENABLED_add_cfg();
    rust_ffi::get_parser_HAVE_UNISTD_H_add_cfg();
    rust_ffi::get_parser_DEBUG_PUSH_add_cfg();
    rust_ffi::get_parser_LIBXML_SCHEMATRON_ENABLED_add_cfg();
    rust_ffi::get_parser_SAX2_add_cfg();
    rust_ffi::get_parser_LIBXML_ICU_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_UNICODE_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_FTP_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_HTTP_ENABLED_add_cfg();
    rust_ffi::get_parser_HAVE_ATTRIBUTE_DESTRUCTOR_add_cfg();
    rust_ffi::get_parser_LIBXML_STATIC_add_cfg();
    rust_ffi::get_parser_WIN32_add_cfg();
    rust_ffi::get_parser_LIBXML_DEBUG_RUNTIME_add_cfg();
    rust_ffi::get_parser_STDC_VERSION_199901_add_cfg();
    rust_ffi::get_parser_LIBXML_CATALOG_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_REGEXP_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_LZMA_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_SCHEMAS_ENABLED_add_cfg();
    rust_ffi::get_parser_HAVE_CTYPE_H_add_cfg();
    rust_ffi::get_parser_LIBXML_STATIC_FOR_DLL_add_cfg();
    rust_ffi::get_parser_LIBXML_READER_ENABLED_add_cfg();
    rust_ffi::get_parser_HAVE_FCNTL_H_add_cfg();
    rust_ffi::get_parser_LIBXML_ZLIB_ENABLED_add_cfg();
    rust_ffi::get_parser_XML_XML_NAMESPACE_add_cfg();
    rust_ffi::get_parser_LIBXML_PATTERN_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_VALID_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_C14N_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_WRITER_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_AUTOMATA_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_TREE_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_XPTR_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_XPATH_ENABLED_add_cfg();
    rust_ffi::get_parser_DEBUG_MEMORY_LOCATION_add_cfg();
    rust_ffi::get_parser_LIBXML_SAX1_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_DEBUG_ENABLED_add_cfg();
    rust_ffi::get_parser_HAVE_SYS_STAT_H_add_cfg();
    rust_ffi::get_parser_HAVE_STDLIB_H_add_cfg();
    rust_ffi::get_parser_LIBXML_ICONV_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_EXPR_ENABLED_add_cfg();
    rust_ffi::get_parser_DEBUG_add_cfg();
    rust_ffi::get_parser_LIBXML_PUSH_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_LEGACY_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_MODULES_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_OUTPUT_ENABLED_add_cfg();
    rust_ffi::get_parser_LIBXML_ISO8859X_ENABLED_add_cfg();
    /**
     * parserInternals.c
     **/
    rust_ffi::get_parserInternals_WIN32_add_cfg();
    rust_ffi::get_parserInternals_CYGWIN_add_cfg();
    rust_ffi::get_parserInternals_HAVE_CTYPE_H_add_cfg();
    rust_ffi::get_parserInternals_HAVE_STDLIB_H_add_cfg();
    rust_ffi::get_parserInternals_HAVE_SYS_STAT_H_add_cfg();
    rust_ffi::get_parserInternals_HAVE_FCNTL_H_add_cfg();
    rust_ffi::get_parserInternals_HAVE_UNISTD_H_add_cfg();
    rust_ffi::get_parserInternals_LIBXML_ZLIB_ENABLED_add_cfg();
    rust_ffi::get_parserInternals_LIBXML_CATALOG_ENABLED_add_cfg();
    rust_ffi::get_parserInternals_DEBUG_INPUT_add_cfg();
    rust_ffi::get_parserInternals_LIBXML_SAX1_ENABLED_add_cfg();

    /**
     * HTMLparser.c
     **/
    rust_ffi::libxml_push_enabled_add_cfg();
    rust_ffi::libxml_sax1_enabled_add_cfg();
    rust_ffi::libxml_iconv_enabled_add_cfg();
    rust_ffi::libxml_icu_enabled_add_cfg();
    rust_ffi::libxml_regexp_enabled_add_cfg();
    rust_ffi::debug_push_add_cfg();

    /**
     * xpath.c
     **/
     rust_ffi::xp_optimized_non_elem_comparison_add_cfg();
     rust_ffi::with_tim_sort_add_cfg();
     rust_ffi::xp_optimized_filter_first_add_cfg();
     rust_ffi::debug_eval_counts_add_cfg();
     rust_ffi::xpath_streaming_add_cfg();
     rust_ffi::libxml_thread_enabled_add_cfg();
     rust_ffi::libxml_xptr_enabled_add_cfg();
     rust_ffi::borlandc_or_msc_ver_and_msc_ver_add_cfg();
     rust_ffi::xmlxpathnodesetsort_add_cfg();
     rust_ffi::gnuc_add_cfg();
     rust_ffi::aix_add_cfg();
     rust_ffi::debug_expr_add_cfg();
     rust_ffi::libxml_docb_enabled_add_cfg();
     rust_ffi::xp_default_cache_on_add_cfg();
     rust_ffi::xp_debug_obj_usage_add_cfg();
     rust_ffi::xml_xml_namespace_add_cfg();
     rust_ffi::debug_or_debug_step_add_cfg();
     rust_ffi::debug_add_cfg();
     rust_ffi::isnan_add_cfg();
     rust_ffi::isinf_add_cfg();
     rust_ffi::libxml_debug_enabled_add_cfg();
     rust_ffi::libxml_xpath_enabled_or_libxml_schemas_enabled_add_cfg();
     rust_ffi::libxml_xpath_enabled_add_cfg();
     rust_ffi::debug_step_add_cfg();
}