pub const XML_TOK_TRAILING_RSQB: libc::c_int = -(5 as libc::c_int);
pub const XML_TOK_NONE: libc::c_int = -(4 as libc::c_int);
pub const XML_TOK_TRAILING_CR: libc::c_int = -(3 as libc::c_int);
pub const XML_TOK_PARTIAL_CHAR: libc::c_int = -(2 as libc::c_int);
pub const XML_TOK_PARTIAL: libc::c_int = -(1 as libc::c_int);
pub const XML_TOK_INVALID: libc::c_int = 0 as libc::c_int;
pub const XML_TOK_START_TAG_WITH_ATTS: libc::c_int = 1 as libc::c_int;
pub const XML_TOK_START_TAG_NO_ATTS: libc::c_int = 2 as libc::c_int;
pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: libc::c_int = 3 as libc::c_int;
pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: libc::c_int = 4 as libc::c_int;
pub const XML_TOK_END_TAG: libc::c_int = 5 as libc::c_int;
pub const XML_TOK_DATA_CHARS: libc::c_int = 6 as libc::c_int;
pub const XML_TOK_DATA_NEWLINE: libc::c_int = 7 as libc::c_int;
pub const XML_TOK_CDATA_SECT_OPEN: libc::c_int = 8 as libc::c_int;
pub const XML_TOK_ENTITY_REF: libc::c_int = 9 as libc::c_int;
pub const XML_TOK_CHAR_REF: libc::c_int = 10 as libc::c_int;
pub const XML_TOK_PI: libc::c_int = 11 as libc::c_int;
pub const XML_TOK_XML_DECL: libc::c_int = 12 as libc::c_int;
pub const XML_TOK_COMMENT: libc::c_int = 13 as libc::c_int;
pub const XML_TOK_BOM: libc::c_int = 14 as libc::c_int;
pub const XML_TOK_PROLOG_S: libc::c_int = 15;
pub const XML_TOK_DECL_OPEN: libc::c_int = 16 as libc::c_int;
pub const XML_TOK_DECL_CLOSE: libc::c_int = 17 as libc::c_int;
pub const XML_TOK_NAME: libc::c_int = 18 as libc::c_int;
pub const XML_TOK_NMTOKEN: libc::c_int = 19 as libc::c_int;
pub const XML_TOK_POUND_NAME: libc::c_int = 20 as libc::c_int;
pub const XML_TOK_OR: libc::c_int = 21 as libc::c_int;
pub const XML_TOK_PERCENT: libc::c_int = 22 as libc::c_int;
pub const XML_TOK_OPEN_PAREN: libc::c_int = 23 as libc::c_int;
pub const XML_TOK_CLOSE_PAREN: libc::c_int = 24 as libc::c_int;
pub const XML_TOK_OPEN_BRACKET: libc::c_int = 25 as libc::c_int;
pub const XML_TOK_CLOSE_BRACKET: libc::c_int = 26 as libc::c_int;
pub const XML_TOK_LITERAL: libc::c_int = 27 as libc::c_int;
pub const XML_TOK_PARAM_ENTITY_REF: libc::c_int = 28 as libc::c_int;
pub const XML_TOK_INSTANCE_START: libc::c_int = 29;
pub const XML_TOK_NAME_QUESTION: libc::c_int = 30 as libc::c_int;
pub const XML_TOK_NAME_ASTERISK: libc::c_int = 31 as libc::c_int;
pub const XML_TOK_NAME_PLUS: libc::c_int = 32 as libc::c_int;
pub const XML_TOK_COND_SECT_OPEN: libc::c_int = 33 as libc::c_int;
pub const XML_TOK_COND_SECT_CLOSE: libc::c_int = 34 as libc::c_int;
pub const XML_TOK_CLOSE_PAREN_QUESTION: libc::c_int = 35 as libc::c_int;
pub const XML_TOK_CLOSE_PAREN_ASTERISK: libc::c_int = 36 as libc::c_int;
pub const XML_TOK_CLOSE_PAREN_PLUS: libc::c_int = 37 as libc::c_int;
pub const XML_TOK_COMMA: libc::c_int = 38 as libc::c_int;
pub const XML_TOK_ATTRIBUTE_VALUE_S: libc::c_int = 39 as libc::c_int;
pub const XML_TOK_CDATA_SECT_CLOSE: libc::c_int = 40 as libc::c_int;
pub const XML_TOK_PREFIXED_NAME: libc::c_int = 41 as libc::c_int;
pub const XML_TOK_IGNORE_SECT: libc::c_int = 42 as libc::c_int;
