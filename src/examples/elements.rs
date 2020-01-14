#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]

use ::c2rust_out::expat_h::{XML_Bool, XML_STATUS_ERROR_0};
use ::c2rust_out::src::lib::xmlparse::{
    XML_ErrorString, XML_GetCurrentLineNumber, XML_GetErrorCode, XML_Parse, XML_ParserCreate,
    XML_ParserFree, XML_SetElementHandler, XML_SetUserData,
};
use ::c2rust_out::stddef_h::NULL;
use ::c2rust_out::stdlib::fprintf;
use ::libc::{printf, putchar};

use libc::{c_char, c_int, c_uint, c_ulong, c_void};
pub mod expat_h {

    use crate::expat_external_h::XML_Char;
    use ::c2rust_out::expat_h::XML_ParserStruct;
    use libc::{c_uint, c_void};
    pub type XML_Parser = *mut XML_ParserStruct;

    pub type XML_Status = c_uint;

    pub type XML_Error = c_uint;

    pub type XML_StartElementHandler = Option<
        unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *mut *const XML_Char) -> (),
    >;

    pub type XML_EndElementHandler =
        Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()>;
}
pub mod expat_external_h {

    use libc::{c_char, c_ulong};
    pub type XML_Char = c_char;

    pub type XML_LChar = c_char;

    pub type XML_Size = c_ulong;
}
pub mod stddef_h {
    use libc::c_ulong;
    pub type size_t = c_ulong;
}
pub mod stdlib {

    use ::c2rust_out::stdlib::_IO_FILE;
    use libc::{c_long, c_ulong, c_void};
    extern "C" {
        #[no_mangle]
        pub static mut stdin: *mut FILE;

        #[no_mangle]
        pub static mut stderr: *mut FILE;

        #[no_mangle]
        pub fn fread(_: *mut c_void, _: c_ulong, _: c_ulong, _: *mut FILE) -> c_ulong;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
    }
    pub type FILE = _IO_FILE;
    pub type _IO_lock_t = ();
    pub type __off_t = c_long;

    pub type __off64_t = c_long;
}

pub use crate::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_EndElementHandler, XML_Error, XML_Parser, XML_StartElementHandler, XML_Status,
};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, FILE,
};

/* This is simple demonstration of how to use expat. This program
   reads an XML document from standard input and writes a line with
   the name of each element to standard output indenting child
   elements by one tab stop more than their parent element.
   It must be used with Expat compiled for UTF-8 output.
                            __  __            _
                         ___\ \/ /_ __   __ _| |_
                        / _ \\  /| '_ \ / _` | __|
                       |  __//  \| |_) | (_| | |_
                        \___/_/\_\ .__/ \__,_|\__|
                                 |_| XML parser

   Copyright (c) 1997-2000 Thai Open Source Software Center Ltd
   Copyright (c) 2000-2017 Expat development team
   Licensed under the MIT license:

   Permission is  hereby granted,  free of charge,  to any  person obtaining
   a  copy  of  this  software   and  associated  documentation  files  (the
   "Software"),  to  deal in  the  Software  without restriction,  including
   without  limitation the  rights  to use,  copy,  modify, merge,  publish,
   distribute, sublicense, and/or sell copies of the Software, and to permit
   persons  to whom  the Software  is  furnished to  do so,  subject to  the
   following conditions:

   The above copyright  notice and this permission notice  shall be included
   in all copies or substantial portions of the Software.

   THE  SOFTWARE  IS  PROVIDED  "AS  IS",  WITHOUT  WARRANTY  OF  ANY  KIND,
   EXPRESS  OR IMPLIED,  INCLUDING  BUT  NOT LIMITED  TO  THE WARRANTIES  OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
   NO EVENT SHALL THE AUTHORS OR  COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
   DAMAGES OR  OTHER LIABILITY, WHETHER  IN AN  ACTION OF CONTRACT,  TORT OR
   OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
   USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

unsafe extern "C" fn startElement(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    let mut i: c_int = 0;
    let mut depthPtr: *mut c_int = userData as *mut c_int;
    i = 0;
    while i < *depthPtr {
        putchar('\t' as i32);
        i += 1
    }
    printf(b"%s\n\x00".as_ptr() as *const c_char, name);
    *depthPtr += 1;
}

unsafe extern "C" fn endElement(mut userData: *mut c_void, mut _name: *const XML_Char) {
    let mut depthPtr: *mut c_int = userData as *mut c_int;
    *depthPtr -= 1;
}

unsafe fn main_0(mut _argc: c_int, mut _argv: *mut *mut c_char) -> c_int {
    let mut buf: [c_char; 8192] = [0; 8192];
    let mut parser: XML_Parser = XML_ParserCreate(NULL as *const XML_Char);
    let mut done: XML_Bool = 0;
    let mut depth: c_int = 0;
    XML_SetUserData(parser, &mut depth as *mut c_int as *mut c_void);
    XML_SetElementHandler(
        parser,
        Some(
            startElement
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(endElement as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    loop {
        let mut len: size_t = crate::stdlib::fread(
            buf.as_mut_ptr() as *mut c_void,
            1,
            ::std::mem::size_of::<[c_char; 8192]>() as c_ulong,
            crate::stdlib::stdin,
        );
        done = (len < ::std::mem::size_of::<[c_char; 8192]>() as c_ulong) as XML_Bool;
        if XML_Parse(parser, buf.as_mut_ptr(), len as c_int, done as c_int)
            == XML_STATUS_ERROR_0 as c_uint
        {
            fprintf(
                crate::stdlib::stderr,
                
                b"%s at line %lu\n\x00".as_ptr() as *const c_char,
                XML_ErrorString(XML_GetErrorCode(parser)),
                XML_GetCurrentLineNumber(parser),
            );
            XML_ParserFree(parser);
            return 1i32;
        }
        if !(done == 0) {
            break;
        }
    }
    XML_ParserFree(parser);
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr())) }
}
