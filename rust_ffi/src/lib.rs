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

/**
 * parser.c
**/
pub fn get_parser_LIBXML_XINCLUDE_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_XINCLUDE_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_XINCLUDE_ENABLED");
    }
}
pub fn get_parser_LIBXML_THREAD_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_THREAD_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_THREAD_ENABLED");
    }
}
pub fn get_parser_LIBXML_HTML_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_HTML_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_HTML_ENABLED");
    }
}
pub fn get_parser_HAVE_UNISTD_H_add_cfg() {
    if unsafe { get_parser_HAVE_UNISTD_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_UNISTD_H");
    }
}
pub fn get_parser_DEBUG_PUSH_add_cfg() {
    if unsafe { get_parser_DEBUG_PUSH() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_DEBUG_PUSH");
    }
}
pub fn get_parser_LIBXML_SCHEMATRON_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_SCHEMATRON_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_SCHEMATRON_ENABLED");
    }
}
pub fn get_parser_SAX2_add_cfg() {
    if unsafe { get_parser_SAX2() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_SAX2");
    }
}
pub fn get_parser_LIBXML_ICU_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_ICU_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ICU_ENABLED");
    }
}
pub fn get_parser_LIBXML_UNICODE_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_UNICODE_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_UNICODE_ENABLED");
    }
}
pub fn get_parser_LIBXML_FTP_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_FTP_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_FTP_ENABLED");
    }
}
pub fn get_parser_LIBXML_HTTP_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_HTTP_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_HTTP_ENABLED");
    }
}
pub fn get_parser_HAVE_ATTRIBUTE_DESTRUCTOR_add_cfg() {
    if unsafe { get_parser_HAVE_ATTRIBUTE_DESTRUCTOR() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_ATTRIBUTE_DESTRUCTOR");
    }
}
pub fn get_parser_LIBXML_STATIC_add_cfg() {
    if unsafe { get_parser_LIBXML_STATIC() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_STATIC");
    }
}
pub fn get_parser_WIN32_add_cfg() {
    if unsafe { get_parser_WIN32() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_WIN32");
    }
}
pub fn get_parser_LIBXML_DEBUG_RUNTIME_add_cfg() {
    if unsafe { get_parser_LIBXML_DEBUG_RUNTIME() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_DEBUG_RUNTIME");
    }
}
pub fn get_parser_STDC_VERSION_199901_add_cfg() {
    if unsafe { get_parser_STDC_VERSION_199901() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_STDC_VERSION_199901");
    }
}
pub fn get_parser_LIBXML_CATALOG_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_CATALOG_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_CATALOG_ENABLED");
    }
}
pub fn get_parser_LIBXML_REGEXP_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_REGEXP_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_REGEXP_ENABLED");
    }
}
pub fn get_parser_LIBXML_LZMA_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_LZMA_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_LZMA_ENABLED");
    }
}
pub fn get_parser_LIBXML_SCHEMAS_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_SCHEMAS_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_SCHEMAS_ENABLED");
    }
}
pub fn get_parser_HAVE_CTYPE_H_add_cfg() {
    if unsafe { get_parser_HAVE_CTYPE_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_CTYPE_H");
    }
}
pub fn get_parser_LIBXML_STATIC_FOR_DLL_add_cfg() {
    if unsafe { get_parser_LIBXML_STATIC_FOR_DLL() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_STATIC_FOR_DLL");
    }
}
pub fn get_parser_LIBXML_READER_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_READER_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_READER_ENABLED");
    }
}
pub fn get_parser_HAVE_FCNTL_H_add_cfg() {
    if unsafe { get_parser_HAVE_FCNTL_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_FCNTL_H");
    }
}
pub fn get_parser_LIBXML_ZLIB_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_ZLIB_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ZLIB_ENABLED");
    }
}
pub fn get_parser_XML_XML_NAMESPACE_add_cfg() {
    if unsafe { get_parser_XML_XML_NAMESPACE() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_XML_XML_NAMESPACE");
    }
}
pub fn get_parser_LIBXML_PATTERN_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_PATTERN_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_PATTERN_ENABLED");
    }
}
pub fn get_parser_LIBXML_VALID_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_VALID_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_VALID_ENABLED");
    }
}
pub fn get_parser_LIBXML_C14N_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_C14N_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_C14N_ENABLED");
    }
}
pub fn get_parser_LIBXML_WRITER_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_WRITER_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_WRITER_ENABLED");
    }
}
pub fn get_parser_LIBXML_AUTOMATA_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_AUTOMATA_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_AUTOMATA_ENABLED");
    }
}
pub fn get_parser_LIBXML_TREE_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_TREE_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_TREE_ENABLED");
    }
}
pub fn get_parser_LIBXML_XPTR_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_XPTR_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_XPTR_ENABLED");
    }
}
pub fn get_parser_LIBXML_XPATH_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_XPATH_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_XPATH_ENABLED");
    }
}
pub fn get_parser_DEBUG_MEMORY_LOCATION_add_cfg() {
    if unsafe { get_parser_DEBUG_MEMORY_LOCATION() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_DEBUG_MEMORY_LOCATION");
    }
}
pub fn get_parser_LIBXML_SAX1_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_SAX1_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_SAX1_ENABLED");
    }
}
pub fn get_parser_LIBXML_DEBUG_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_DEBUG_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_DEBUG_ENABLED");
    }
}
pub fn get_parser_HAVE_SYS_STAT_H_add_cfg() {
    if unsafe { get_parser_HAVE_SYS_STAT_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_SYS_STAT_H");
    }
}
pub fn get_parser_HAVE_STDLIB_H_add_cfg() {
    if unsafe { get_parser_HAVE_STDLIB_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_HAVE_STDLIB_H");
    }
}
pub fn get_parser_LIBXML_ICONV_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_ICONV_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ICONV_ENABLED");
    }
}
pub fn get_parser_LIBXML_EXPR_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_EXPR_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_EXPR_ENABLED");
    }
}
pub fn get_parser_DEBUG_add_cfg() {
    if unsafe { get_parser_DEBUG() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_DEBUG");
    }
}
pub fn get_parser_LIBXML_PUSH_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_PUSH_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_PUSH_ENABLED");
    }
}
pub fn get_parser_LIBXML_LEGACY_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_LEGACY_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_LEGACY_ENABLED");
    }
}
pub fn get_parser_LIBXML_MODULES_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_MODULES_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_MODULES_ENABLED");
    }
}
pub fn get_parser_LIBXML_OUTPUT_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_OUTPUT_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_OUTPUT_ENABLED");
    }
}
pub fn get_parser_LIBXML_ISO8859X_ENABLED_add_cfg() {
    if unsafe { get_parser_LIBXML_ISO8859X_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parser_LIBXML_ISO8859X_ENABLED");
    }
}

/**
 * parserInternals.c
 **/
pub fn get_parserInternals_WIN32_add_cfg() {
    if unsafe { get_parserInternals_WIN32() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_WIN32");
    }
}
pub fn get_parserInternals_CYGWIN_add_cfg() {
    if unsafe { get_parserInternals_CYGWIN() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_CYGWIN");
    }
}
pub fn get_parserInternals_HAVE_CTYPE_H_add_cfg() {
    if unsafe { get_parserInternals_HAVE_CTYPE_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_CTYPE_H");
    }
}
pub fn get_parserInternals_HAVE_STDLIB_H_add_cfg() {
    if unsafe { get_parserInternals_HAVE_STDLIB_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_STDLIB_H");
    }
}
pub fn get_parserInternals_HAVE_SYS_STAT_H_add_cfg() {
    if unsafe { get_parserInternals_HAVE_SYS_STAT_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_SYS_STAT_H");
    }
}
pub fn get_parserInternals_HAVE_FCNTL_H_add_cfg() {
    if unsafe { get_parserInternals_HAVE_FCNTL_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_FCNTL_H");
    }
}
pub fn get_parserInternals_HAVE_UNISTD_H_add_cfg() {
    if unsafe { get_parserInternals_HAVE_UNISTD_H() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_HAVE_UNISTD_H");
    }
}
pub fn get_parserInternals_LIBXML_ZLIB_ENABLED_add_cfg() {
    if unsafe { get_parserInternals_LIBXML_ZLIB_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_LIBXML_ZLIB_ENABLED");
    }
}
pub fn get_parserInternals_LIBXML_CATALOG_ENABLED_add_cfg() {
    if unsafe { get_parserInternals_LIBXML_CATALOG_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_LIBXML_CATALOG_ENABLED");
    }
}
pub fn get_parserInternals_DEBUG_INPUT_add_cfg() {
    if unsafe { get_parserInternals_DEBUG_INPUT() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_DEBUG_INPUT");
    }
}
pub fn get_parserInternals_LIBXML_SAX1_ENABLED_add_cfg() {
    if unsafe { get_parserInternals_LIBXML_SAX1_ENABLED() } == 1 {
        println!("cargo:rustc-cfg=HAVE_parserInternals_LIBXML_SAX1_ENABLED");
    }
}

/**
 * HTMLparser.c
 **/

pub fn libxml_push_enabled_add_cfg() {
    if unsafe { get_libxml_push_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_PUSH_ENABLED");
    }
}

pub fn libxml_sax1_enabled_add_cfg() {
    if unsafe { get_libxml_sax1_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_SAX1_ENABLED");
    }
}

pub fn libxml_iconv_enabled_add_cfg() {
    if unsafe { get_libxml_iconv_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_ICONV_ENABLED");
    }
}

pub fn libxml_icu_enabled_add_cfg() {
    if unsafe { get_libxml_icu_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_ICU_ENABLED");
    }
}

pub fn libxml_regexp_enabled_add_cfg() {
    if unsafe { get_libxml_regexp_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_REGEXP_ENABLED");
    }
}

pub fn debug_push_add_cfg() {
    if unsafe { get_debug_push() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_PUSH");
    }
}

/**
 * xpath.c
 **/

pub fn libxml_xpath_enabled_add_cfg() {
    if unsafe { get_libxml_xpath_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_XPATH_ENABLED");
    }
}

pub fn debug_step_add_cfg() {
    if unsafe { get_debug_step() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_STEP");
    }
}

pub fn libxml_xpath_enabled_or_libxml_schemas_enabled_add_cfg() {
    if unsafe { get_libxml_xpath_enabled_or_libxml_schemas_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED");
    }
}

pub fn libxml_debug_enabled_add_cfg() {
    if unsafe { get_libxml_debug_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_DEBUG_ENABLED");
    }
}

pub fn isnan_add_cfg() {
    if unsafe { get_isnan() } == 1 {
        println!("cargo:rustc-cfg=ISNAN");
    }
}

pub fn isinf_add_cfg() {
    if unsafe { get_isinf() } == 1 {
        println!("cargo:rustc-cfg=ISINF");
    }
}

pub fn debug_add_cfg() {
    if unsafe { get_debug() } == 1 {
        println!("cargo:rustc-cfg=DEBUG");
    }
}

pub fn xp_optimized_non_elem_comparison_add_cfg() {
    if unsafe { get_xp_optimized_non_elem_comparison() } == 1 {
        println!("cargo:rustc-cfg=XP_OPTIMIZED_NON_ELEM_COMPARISON");
    }
}

pub fn with_tim_sort_add_cfg() {
    if unsafe { get_with_tim_sort() } == 1 {
        println!("cargo:rustc-cfg=WITH_TIM_SORT");
    }
}

pub fn xp_optimized_filter_first_add_cfg() {
    if unsafe { get_xp_optimized_filter_first() } == 1 {
        println!("cargo:rustc-cfg=XP_OPTIMIZED_FILTER_FIRST");
    }
}

pub fn debug_eval_counts_add_cfg() {
    if unsafe { get_debug_eval_counts() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_EVAL_COUNTS");
    }
}

pub fn xpath_streaming_add_cfg() {
    if unsafe { get_xpath_streaming() } == 1 {
        println!("cargo:rustc-cfg=XPATH_STREAMING");
    }
}

pub fn libxml_thread_enabled_add_cfg() {
    if unsafe { get_libxml_thread_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_THREAD_ENABLED");
    }
}

pub fn libxml_xptr_enabled_add_cfg() {
    if unsafe { get_libxml_xptr_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_XPTR_ENABLED");
    }
}

pub fn borlandc_or_msc_ver_and_msc_ver_add_cfg() {
    if unsafe { get_borlandc_or_msc_ver_and_msc_ver() } == 1 {
        println!("cargo:rustc-cfg=BORLANDC_OR_MSC_VER_AND_MSC_VER");
    }
}

pub fn xmlxpathnodesetsort_add_cfg() {
    if unsafe { get_xmlxpathnodesetsort() } == 1 {
        println!("cargo:rustc-cfg=XMLXPATHNODESETSORT");
    }
}

pub fn gnuc_add_cfg() {
    if unsafe { get_gnuc() } == 1 {
        println!("cargo:rustc-cfg=GUNC");
    }
}

pub fn aix_add_cfg() {
    if unsafe { get_aix() } == 1 {
        println!("cargo:rustc-cfg=AIX");
    }
}

pub fn debug_expr_add_cfg() {
    if unsafe { get_debug_expr() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_EXPR");
    }
}

pub fn libxml_docb_enabled_add_cfg() {
    if unsafe { get_libxml_docb_enabled() } == 1 {
        println!("cargo:rustc-cfg=LIBXML_DOCB_ENABLED");
    }
}

pub fn xp_default_cache_on_add_cfg() {
    if unsafe { get_xp_default_cache_on() } == 1 {
        println!("cargo:rustc-cfg=XP_DEFAULT_CACHE_ON");
    }
}

pub fn xp_debug_obj_usage_add_cfg() {
    if unsafe { get_xp_debug_obj_usage() } == 1 {
        println!("cargo:rustc-cfg=XP_DEBUG_OBJ_USAGE");
    }
}

pub fn xml_xml_namespace_add_cfg() {
    if unsafe { get_xml_xml_namespace() } == 1 {
        println!("cargo:rustc-cfg=XML_XML_NAMESPACE");
    }
}

pub fn debug_or_debug_step_add_cfg() {
    if unsafe { get_debug_or_debug_step() } == 1 {
        println!("cargo:rustc-cfg=DEBUG_OR_DEBUG_STEP");
    }
}
