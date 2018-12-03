// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::prelude::*;
use pyo3::types::*;
use std::sync::Once;

pub static mut STR_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut BYTES_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut DICT_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut LIST_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut TUPLE_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut NONE_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut BOOL_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut INT_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;
pub static mut FLOAT_PTR: *mut pyo3::ffi::PyTypeObject = 0 as *mut pyo3::ffi::PyTypeObject;

static INIT: Once = Once::new();

pub fn init_typerefs(py: Python) {
    INIT.call_once(|| unsafe {
        STR_PTR = PyUnicode::new(py, "python").as_ref(py).get_type_ptr();
        BYTES_PTR = PyBytes::new(py, b"python").as_ref(py).get_type_ptr();
        DICT_PTR = PyDict::new(py).as_ref().get_type_ptr();
        LIST_PTR = PyList::empty(py).as_ref().get_type_ptr();
        TUPLE_PTR = PyTuple::empty(py).as_ref(py).get_type_ptr();
        NONE_PTR = py.None().as_ref(py).get_type_ptr();
        BOOL_PTR = true.to_object(py).as_ref(py).get_type_ptr();
        INT_PTR = 1.to_object(py).as_ref(py).get_type_ptr();
        FLOAT_PTR = 1.0.to_object(py).as_ref(py).get_type_ptr();
    });
}
