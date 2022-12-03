#[link(name = "xml2")]
extern "C" {
    /**
     * parser.c
     **/
    fn get_parser_LIBXML_XINCLUDE_ENABLED() -> i32;

    fn get_parser_LIBXML_THREAD_ENABLED() -> i32;

    fn get_parser_LIBXML_HTML_ENABLED() -> i32;

    fn get_parser_HAVE_UNISTD_H() -> i32;

    fn get_parser_DEBUG_PUSH() -> i32;

    fn get_parser_LIBXML_SCHEMATRON_ENABLED() -> i32;

    fn get_parser_SAX2() -> i32;

    fn get_parser_LIBXML_ICU_ENABLED() -> i32;

    fn get_parser_LIBXML_UNICODE_ENABLED() -> i32;

    fn get_parser_LIBXML_FTP_ENABLED() -> i32;

    fn get_parser_LIBXML_HTTP_ENABLED() -> i32;

    fn get_parser_HAVE_ATTRIBUTE_DESTRUCTOR() -> i32;

    fn get_parser_LIBXML_STATIC() -> i32;

    fn get_parser_WIN32() -> i32;

    fn get_parser_LIBXML_DEBUG_RUNTIME() -> i32;

    fn get_parser_STDC_VERSION_199901() -> i32;

    fn get_parser_LIBXML_CATALOG_ENABLED() -> i32;

    fn get_parser_LIBXML_REGEXP_ENABLED() -> i32;

    fn get_parser_LIBXML_LZMA_ENABLED() -> i32;

    fn get_parser_LIBXML_SCHEMAS_ENABLED() -> i32;

    fn get_parser_HAVE_CTYPE_H() -> i32;

    fn get_parser_LIBXML_STATIC_FOR_DLL() -> i32;

    fn get_parser_LIBXML_READER_ENABLED() -> i32;

    fn get_parser_HAVE_FCNTL_H() -> i32;

    fn get_parser_LIBXML_ZLIB_ENABLED() -> i32;

    fn get_parser_XML_XML_NAMESPACE() -> i32;

    fn get_parser_LIBXML_PATTERN_ENABLED() -> i32;

    fn get_parser_LIBXML_VALID_ENABLED() -> i32;

    fn get_parser_LIBXML_C14N_ENABLED() -> i32;

    fn get_parser_LIBXML_WRITER_ENABLED() -> i32;

    fn get_parser_LIBXML_AUTOMATA_ENABLED() -> i32;

    fn get_parser_LIBXML_TREE_ENABLED() -> i32;

    fn get_parser_LIBXML_XPTR_ENABLED() -> i32;

    fn get_parser_LIBXML_XPATH_ENABLED() -> i32;

    fn get_parser_DEBUG_MEMORY_LOCATION() -> i32;

    fn get_parser_LIBXML_SAX1_ENABLED() -> i32;

    fn get_parser_LIBXML_DEBUG_ENABLED() -> i32;

    fn get_parser_HAVE_SYS_STAT_H() -> i32;

    fn get_parser_HAVE_STDLIB_H() -> i32;

    fn get_parser_LIBXML_ICONV_ENABLED() -> i32;

    fn get_parser_LIBXML_EXPR_ENABLED() -> i32;

    fn get_parser_DEBUG() -> i32;

    fn get_parser_LIBXML_PUSH_ENABLED() -> i32;

    fn get_parser_LIBXML_LEGACY_ENABLED() -> i32;

    fn get_parser_LIBXML_MODULES_ENABLED() -> i32;

    fn get_parser_LIBXML_OUTPUT_ENABLED() -> i32;

    fn get_parser_LIBXML_ISO8859X_ENABLED() -> i32;
    /**
     * parserInternals.c
     **/
    fn get_parserInternals_WIN32() -> i32;

    fn get_parserInternals_CYGWIN() -> i32;

    fn get_parserInternals_HAVE_CTYPE_H() -> i32;

    fn get_parserInternals_HAVE_STDLIB_H() -> i32;

    fn get_parserInternals_HAVE_SYS_STAT_H() -> i32;

    fn get_parserInternals_HAVE_FCNTL_H() -> i32;

    fn get_parserInternals_HAVE_UNISTD_H() -> i32;

    fn get_parserInternals_LIBXML_ZLIB_ENABLED() -> i32;

    fn get_parserInternals_LIBXML_CATALOG_ENABLED() -> i32;

    fn get_parserInternals_DEBUG_INPUT() -> i32;

    fn get_parserInternals_LIBXML_SAX1_ENABLED() -> i32;

    /**
     * HTMLparser.c
     **/
    fn get_libxml_push_enabled() -> i32;

    fn get_libxml_sax1_enabled() -> i32;

    fn get_libxml_iconv_enabled() -> i32;

    fn get_libxml_icu_enabled() -> i32;

    fn get_libxml_regexp_enabled() -> i32;

    fn get_debug_push() -> i32;

    /**
     * xpath.c
     **/
    fn get_xp_optimized_non_elem_comparison() -> i32;

    fn get_with_tim_sort() -> i32;

    fn get_xp_optimized_filter_first() -> i32;

    fn get_debug_eval_counts() -> i32;

    fn get_xpath_streaming() -> i32;

    fn get_libxml_thread_enabled() -> i32;

    fn get_libxml_xptr_enabled() -> i32;

    fn get_borlandc_or_msc_ver_and_msc_ver() -> i32;

    fn get_xmlxpathnodesetsort() -> i32;

    fn get_gnuc() -> i32;

    fn get_aix() -> i32;

    fn get_debug_expr() -> i32;

    fn get_libxml_docb_enabled() -> i32;

    fn get_xp_default_cache_on() -> i32;

    fn get_xp_debug_obj_usage() -> i32;

    fn get_xml_xml_namespace() -> i32;

    fn get_debug_or_debug_step() -> i32;

    fn get_debug() -> i32;

    fn get_isnan() -> i32;

    fn get_isinf() -> i32;

    fn get_libxml_debug_enabled() -> i32;

    fn get_libxml_xpath_enabled_or_libxml_schemas_enabled() -> i32;

    fn get_libxml_xpath_enabled() -> i32;

    fn get_debug_step() -> i32;

}

pub fn add_cfg() {
    get_parser_libxml_xinclude_enabled_add_cfg();
    get_parser_libxml_thread_enabled_add_cfg();
    get_parser_libxml_html_enabled_add_cfg();
    get_parser_have_unistd_h_add_cfg();
    get_parser_debug_push_add_cfg();
    get_parser_libxml_schematron_enabled_add_cfg();
    get_parser_sax2_add_cfg();
    get_parser_libxml_icu_enabled_add_cfg();
    get_parser_libxml_unicode_enabled_add_cfg();
    get_parser_libxml_ftp_enabled_add_cfg();
    get_parser_libxml_http_enabled_add_cfg();
    get_parser_have_attribute_destructor_add_cfg();
    get_parser_libxml_static_add_cfg();
    get_parser_win32_add_cfg();
    get_parser_libxml_debug_runtime_add_cfg();
    get_parser_stdc_version_199901_add_cfg();
    get_parser_libxml_catalog_enabled_add_cfg();
    get_parser_libxml_regexp_enabled_add_cfg();
    get_parser_libxml_lzma_enabled_add_cfg();
    get_parser_libxml_schemas_enabled_add_cfg();
    get_parser_have_ctype_h_add_cfg();
    get_parser_libxml_static_for_dll_add_cfg();
    get_parser_libxml_reader_enabled_add_cfg();
    get_parser_have_fcntl_h_add_cfg();
    get_parser_libxml_zlib_enabled_add_cfg();
    get_parser_xml_xml_namespace_add_cfg();
    get_parser_libxml_pattern_enabled_add_cfg();
    get_parser_libxml_valid_enabled_add_cfg();
    get_parser_libxml_c14_n_enabled_add_cfg();
    get_parser_libxml_writer_enabled_add_cfg();
    get_parser_libxml_automata_enabled_add_cfg();
    get_parser_libxml_tree_enabled_add_cfg();
    get_parser_libxml_xptr_enabled_add_cfg();
    get_parser_libxml_xpath_enabled_add_cfg();
    get_parser_debug_memory_location_add_cfg();
    get_parser_libxml_sax1_enabled_add_cfg();
    get_parser_libxml_debug_enabled_add_cfg();
    get_parser_have_sys_stat_h_add_cfg();
    get_parser_have_stdlib_h_add_cfg();
    get_parser_libxml_iconv_enabled_add_cfg();
    get_parser_libxml_expr_enabled_add_cfg();
    get_parser_debug_add_cfg();
    get_parser_libxml_push_enabled_add_cfg();
    get_parser_libxml_legacy_enabled_add_cfg();
    get_parser_libxml_modules_enabled_add_cfg();
    get_parser_libxml_output_enabled_add_cfg();
    get_parser_libxml_iso8859_x_enabled_add_cfg();

    get_parser_internals_win32_add_cfg();
    get_parser_internals_cygwin_add_cfg();
    get_parser_internals_have_ctype_h_add_cfg();
    get_parser_internals_have_stdlib_h_add_cfg();
    get_parser_internals_have_sys_stat_h_add_cfg();
    get_parser_internals_have_fcntl_h_add_cfg();
    get_parser_internals_have_unistd_h_add_cfg();
    get_parser_internals_libxml_zlib_enabled_add_cfg();
    get_parser_internals_libxml_catalog_enabled_add_cfg();
    get_parser_internals_debug_input_add_cfg();
    get_parser_internals_libxml_sax1_enabled_add_cfg();

    libxml_push_enabled_add_cfg();
    libxml_sax1_enabled_add_cfg();
    libxml_iconv_enabled_add_cfg();
    libxml_icu_enabled_add_cfg();
    libxml_regexp_enabled_add_cfg();
    debug_push_add_cfg();

    libxml_xpath_enabled_add_cfg();
    debug_step_add_cfg();
    libxml_xpath_enabled_or_libxml_schemas_enabled_add_cfg();
    libxml_debug_enabled_add_cfg();
    isnan_add_cfg();
    isinf_add_cfg();
    debug_add_cfg();
    xp_optimized_non_elem_comparison_add_cfg();
    with_tim_sort_add_cfg();
    xp_optimized_filter_first_add_cfg();
    debug_eval_counts_add_cfg();
    xpath_streaming_add_cfg();
    libxml_thread_enabled_add_cfg();
    libxml_xptr_enabled_add_cfg();
    borlandc_or_msc_ver_and_msc_ver_add_cfg();
    xmlxpathnodesetsort_add_cfg();
    gnuc_add_cfg();
    aix_add_cfg();
    debug_expr_add_cfg();
    libxml_docb_enabled_add_cfg();
    xp_default_cache_on_add_cfg();
    xp_debug_obj_usage_add_cfg();
    xml_xml_namespace_add_cfg();
    debug_or_debug_step_add_cfg();
}

/**
 * parser.c
**/
fn get_parser_libxml_xinclude_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_XINCLUDE_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_XINCLUDE_ENABLED");
    }
}
fn get_parser_libxml_thread_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_THREAD_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_THREAD_ENABLED");
    }
}
fn get_parser_libxml_html_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_HTML_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_HTML_ENABLED");
    }
}
fn get_parser_have_unistd_h_add_cfg() {
    if unsafe { get_parser_HAVE_UNISTD_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_UNISTD_H");
    }
}
fn get_parser_debug_push_add_cfg() {
    if unsafe { get_parser_DEBUG_PUSH() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_DEBUG_PUSH");
    }
}
fn get_parser_libxml_schematron_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_SCHEMATRON_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_SCHEMATRON_ENABLED");
    }
}
fn get_parser_sax2_add_cfg() {
    if unsafe { get_parser_SAX2() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_SAX2");
    }
}
fn get_parser_libxml_icu_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_ICU_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ICU_ENABLED");
    }
}
fn get_parser_libxml_unicode_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_UNICODE_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_UNICODE_ENABLED");
    }
}
fn get_parser_libxml_ftp_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_FTP_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_FTP_ENABLED");
    }
}
fn get_parser_libxml_http_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_HTTP_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_HTTP_ENABLED");
    }
}
fn get_parser_have_attribute_destructor_add_cfg() {
    if unsafe { get_parser_HAVE_ATTRIBUTE_DESTRUCTOR() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_ATTRIBUTE_DESTRUCTOR");
    }
}
fn get_parser_libxml_static_add_cfg() {
    if unsafe { get_parser_LIBXML_STATIC() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_STATIC");
    }
}
fn get_parser_win32_add_cfg() {
    if unsafe { get_parser_WIN32() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_WIN32");
    }
}
fn get_parser_libxml_debug_runtime_add_cfg() {
    if unsafe { get_parser_LIBXML_DEBUG_RUNTIME() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_DEBUG_RUNTIME");
    }
}
fn get_parser_stdc_version_199901_add_cfg() {
    if unsafe { get_parser_STDC_VERSION_199901() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_STDC_VERSION_199901");
    }
}
fn get_parser_libxml_catalog_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_CATALOG_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_CATALOG_ENABLED");
    }
}
fn get_parser_libxml_regexp_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_REGEXP_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_REGEXP_ENABLED");
    }
}
fn get_parser_libxml_lzma_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_LZMA_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_LZMA_ENABLED");
    }
}
fn get_parser_libxml_schemas_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_SCHEMAS_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_SCHEMAS_ENABLED");
    }
}
fn get_parser_have_ctype_h_add_cfg() {
    if unsafe { get_parser_HAVE_CTYPE_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_CTYPE_H");
    }
}
fn get_parser_libxml_static_for_dll_add_cfg() {
    if unsafe { get_parser_LIBXML_STATIC_FOR_DLL() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_STATIC_FOR_DLL");
    }
}
fn get_parser_libxml_reader_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_READER_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_READER_ENABLED");
    }
}
fn get_parser_have_fcntl_h_add_cfg() {
    if unsafe { get_parser_HAVE_FCNTL_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_FCNTL_H");
    }
}
fn get_parser_libxml_zlib_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_ZLIB_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ZLIB_ENABLED");
    }
}
fn get_parser_xml_xml_namespace_add_cfg() {
    if unsafe { get_parser_XML_XML_NAMESPACE() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_XML_XML_NAMESPACE");
    }
}
fn get_parser_libxml_pattern_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_PATTERN_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_PATTERN_ENABLED");
    }
}
fn get_parser_libxml_valid_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_VALID_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_VALID_ENABLED");
    }
}
fn get_parser_libxml_c14_n_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_C14N_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_C14N_ENABLED");
    }
}
fn get_parser_libxml_writer_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_WRITER_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_WRITER_ENABLED");
    }
}
fn get_parser_libxml_automata_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_AUTOMATA_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_AUTOMATA_ENABLED");
    }
}
fn get_parser_libxml_tree_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_TREE_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_TREE_ENABLED");
    }
}
fn get_parser_libxml_xptr_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_XPTR_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_XPTR_ENABLED");
    }
}
fn get_parser_libxml_xpath_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_XPATH_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_XPATH_ENABLED");
    }
}
fn get_parser_debug_memory_location_add_cfg() {
    if unsafe { get_parser_DEBUG_MEMORY_LOCATION() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_DEBUG_MEMORY_LOCATION");
    }
}
fn get_parser_libxml_sax1_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_SAX1_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_SAX1_ENABLED");
    }
}
fn get_parser_libxml_debug_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_DEBUG_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_DEBUG_ENABLED");
    }
}
fn get_parser_have_sys_stat_h_add_cfg() {
    if unsafe { get_parser_HAVE_SYS_STAT_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_SYS_STAT_H");
    }
}
fn get_parser_have_stdlib_h_add_cfg() {
    if unsafe { get_parser_HAVE_STDLIB_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_STDLIB_H");
    }
}
fn get_parser_libxml_iconv_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_ICONV_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ICONV_ENABLED");
    }
}
fn get_parser_libxml_expr_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_EXPR_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_EXPR_ENABLED");
    }
}
fn get_parser_debug_add_cfg() {
    if unsafe { get_parser_DEBUG() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_DEBUG");
    }
}
fn get_parser_libxml_push_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_PUSH_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_PUSH_ENABLED");
    }
}
fn get_parser_libxml_legacy_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_LEGACY_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_LEGACY_ENABLED");
    }
}
fn get_parser_libxml_modules_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_MODULES_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_MODULES_ENABLED");
    }
}
fn get_parser_libxml_output_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_OUTPUT_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_OUTPUT_ENABLED");
    }
}
fn get_parser_libxml_iso8859_x_enabled_add_cfg() {
    if unsafe { get_parser_LIBXML_ISO8859X_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ISO8859X_ENABLED");
    }
}

/**
 * parserInternals.c
 **/
fn get_parser_internals_win32_add_cfg() {
    if unsafe { get_parserInternals_WIN32() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_WIN32");
    }
}
fn get_parser_internals_cygwin_add_cfg() {
    if unsafe { get_parserInternals_CYGWIN() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_CYGWIN");
    }
}
fn get_parser_internals_have_ctype_h_add_cfg() {
    if unsafe { get_parserInternals_HAVE_CTYPE_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_CTYPE_H");
    }
}
fn get_parser_internals_have_stdlib_h_add_cfg() {
    if unsafe { get_parserInternals_HAVE_STDLIB_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_STDLIB_H");
    }
}
fn get_parser_internals_have_sys_stat_h_add_cfg() {
    if unsafe { get_parserInternals_HAVE_SYS_STAT_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_SYS_STAT_H");
    }
}
fn get_parser_internals_have_fcntl_h_add_cfg() {
    if unsafe { get_parserInternals_HAVE_FCNTL_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_FCNTL_H");
    }
}
fn get_parser_internals_have_unistd_h_add_cfg() {
    if unsafe { get_parserInternals_HAVE_UNISTD_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_UNISTD_H");
    }
}
fn get_parser_internals_libxml_zlib_enabled_add_cfg() {
    if unsafe { get_parserInternals_LIBXML_ZLIB_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_LIBXML_ZLIB_ENABLED");
    }
}
fn get_parser_internals_libxml_catalog_enabled_add_cfg() {
    if unsafe { get_parserInternals_LIBXML_CATALOG_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_LIBXML_CATALOG_ENABLED");
    }
}
fn get_parser_internals_debug_input_add_cfg() {
    if unsafe { get_parserInternals_DEBUG_INPUT() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_DEBUG_INPUT");
    }
}
fn get_parser_internals_libxml_sax1_enabled_add_cfg() {
    if unsafe { get_parserInternals_LIBXML_SAX1_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_LIBXML_SAX1_ENABLED");
    }
}

/**
 * HTMLparser.c
 **/

fn libxml_push_enabled_add_cfg() {
    if unsafe { get_libxml_push_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_PUSH_ENABLED");
    }
}

fn libxml_sax1_enabled_add_cfg() {
    if unsafe { get_libxml_sax1_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_SAX1_ENABLED");
    }
}

fn libxml_iconv_enabled_add_cfg() {
    if unsafe { get_libxml_iconv_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_ICONV_ENABLED");
    }
}

fn libxml_icu_enabled_add_cfg() {
    if unsafe { get_libxml_icu_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_ICU_ENABLED");
    }
}

fn libxml_regexp_enabled_add_cfg() {
    if unsafe { get_libxml_regexp_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_REGEXP_ENABLED");
    }
}

fn debug_push_add_cfg() {
    if unsafe { get_debug_push() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_PUSH");
    }
}

/**
 * xpath.c
 **/

fn libxml_xpath_enabled_add_cfg() {
    if unsafe { get_libxml_xpath_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_XPATH_ENABLED");
    }
}

fn debug_step_add_cfg() {
    if unsafe { get_debug_step() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_STEP");
    }
}

fn libxml_xpath_enabled_or_libxml_schemas_enabled_add_cfg() {
    if unsafe { get_libxml_xpath_enabled_or_libxml_schemas_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED");
    }
}

fn libxml_debug_enabled_add_cfg() {
    if unsafe { get_libxml_debug_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_DEBUG_ENABLED");
    }
}

fn isnan_add_cfg() {
    if unsafe { get_isnan() } == 1 {
        println!("cargo:rustc-cfg=ISNAN");
    }
}

fn isinf_add_cfg() {
    if unsafe { get_isinf() } == 1 {
        println!("cargo:rustc-cfg=ISINF");
    }
}

fn debug_add_cfg() {
    if unsafe { get_debug() } == 1 {
        println!("cargo:rustc-cfg=DEBUG");
    }
}

fn xp_optimized_non_elem_comparison_add_cfg() {
    if unsafe { get_xp_optimized_non_elem_comparison() } == 1 {
        println!("cargo:rustc-cfg=XP_OPTIMIZED_NON_ELEM_COMPARISON");
    }
}

fn with_tim_sort_add_cfg() {
    if unsafe { get_with_tim_sort() } == 1 {
        println!("cargo:rustc-cfg=WITH_TIM_SORT");
    }
}

fn xp_optimized_filter_first_add_cfg() {
    if unsafe { get_xp_optimized_filter_first() } == 1 {
        println!("cargo:rustc-cfg=XP_OPTIMIZED_FILTER_FIRST");
    }
}

fn debug_eval_counts_add_cfg() {
    if unsafe { get_debug_eval_counts() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_EVAL_COUNTS");
    }
}

fn xpath_streaming_add_cfg() {
    if unsafe { get_xpath_streaming() } == 1 {
        println!("cargo:rustc-cfg=XPATH_STREAMING");
    }
}

fn libxml_thread_enabled_add_cfg() {
    if unsafe { get_libxml_thread_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_THREAD_ENABLED");
    }
}

fn libxml_xptr_enabled_add_cfg() {
    if unsafe { get_libxml_xptr_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_XPTR_ENABLED");
    }
}

fn borlandc_or_msc_ver_and_msc_ver_add_cfg() {
    if unsafe { get_borlandc_or_msc_ver_and_msc_ver() } == 1 {
        println!("cargo:rustc-cfg=BORLANDC_OR_MSC_VER_AND_MSC_VER");
    }
}

fn xmlxpathnodesetsort_add_cfg() {
    if unsafe { get_xmlxpathnodesetsort() } == 1 {
        println!("cargo:rustc-cfg=XMLXPATHNODESETSORT");
    }
}

fn gnuc_add_cfg() {
    if unsafe { get_gnuc() } == 1 {
        println!("cargo:rustc-cfg=GUNC");
    }
}

fn aix_add_cfg() {
    if unsafe { get_aix() } == 1 {
        println!("cargo:rustc-cfg=AIX");
    }
}

fn debug_expr_add_cfg() {
    if unsafe { get_debug_expr() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_EXPR");
    }
}

fn libxml_docb_enabled_add_cfg() {
    if unsafe { get_libxml_docb_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_DOCB_ENABLED");
    }
}

fn xp_default_cache_on_add_cfg() {
    if unsafe { get_xp_default_cache_on() } == 1 {
        println!("cargo:rustc-cfg=XP_DEFAULT_CACHE_ON");
    }
}

fn xp_debug_obj_usage_add_cfg() {
    if unsafe { get_xp_debug_obj_usage() } == 1 {
        println!("cargo:rustc-cfg=XP_DEBUG_OBJ_USAGE");
    }
}

fn xml_xml_namespace_add_cfg() {
    if unsafe { get_xml_xml_namespace() } == 1 {
        println!("cargo:rustc-cfg=XML_XML_NAMESPACE");
    }
}

fn debug_or_debug_step_add_cfg() {
    if unsafe { get_debug_or_debug_step() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_OR_DEBUG_STEP");
    }
}
