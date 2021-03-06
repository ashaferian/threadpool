/* automatically generated by rust-bindgen */
#![allow(non_camel_case_types)]
pub type __uint64_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_cond {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_mutex {
    _unused: [u8; 0],
}
pub type pthread_mutex_t = *mut pthread_mutex;
pub type pthread_cond_t = *mut pthread_cond;
pub type atomic_bool = u8;
pub type atomic_int = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qnl_exec_t {
    pub qe_func: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub qe_arg: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_qnl_exec_t() {
    assert_eq!(::std::mem::size_of::<qnl_exec_t>(), 16usize, concat!("Size of: ", stringify!(qnl_exec_t)));
    assert_eq!(::std::mem::align_of::<qnl_exec_t>(), 8usize, concat!("Alignment of ", stringify!(qnl_exec_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<qnl_exec_t>())).qe_func as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(qnl_exec_t), "::", stringify!(qe_func)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<qnl_exec_t>())).qe_arg as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(qnl_exec_t), "::", stringify!(qe_arg)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _qnode_t {
    pub qn_data: *mut qnl_exec_t,
    pub qn_next: *mut _qnode_t,
}
#[test]
fn bindgen_test_layout__qnode_t() {
    assert_eq!(::std::mem::size_of::<_qnode_t>(), 16usize, concat!("Size of: ", stringify!(_qnode_t)));
    assert_eq!(::std::mem::align_of::<_qnode_t>(), 8usize, concat!("Alignment of ", stringify!(_qnode_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<_qnode_t>())).qn_data as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(_qnode_t), "::", stringify!(qn_data)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<_qnode_t>())).qn_next as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(_qnode_t), "::", stringify!(qn_next)));
}
pub type qnode_t = _qnode_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qend_t {
    pub qe_count: u64,
    pub qe_node: *mut qnode_t,
}
#[test]
fn bindgen_test_layout_qend_t() {
    assert_eq!(::std::mem::size_of::<qend_t>(), 16usize, concat!("Size of: ", stringify!(qend_t)));
    assert_eq!(::std::mem::align_of::<qend_t>(), 8usize, concat!("Alignment of ", stringify!(qend_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<qend_t>())).qe_count as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(qend_t), "::", stringify!(qe_count)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<qend_t>())).qe_node as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(qend_t), "::", stringify!(qe_node)));
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct queue_t {
    pub q_size: atomic_int,
    pub __bindgen_padding_0: u64,
    pub q_head: qend_t,
    pub q_tail: qend_t,
}
#[test]
fn bindgen_test_layout_queue_t() {
    assert_eq!(::std::mem::size_of::<queue_t>(), 48usize, concat!("Size of: ", stringify!(queue_t)));
    assert_eq!(::std::mem::align_of::<queue_t>(), 16usize, concat!("Alignment of ", stringify!(queue_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<queue_t>())).q_size as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(queue_t), "::", stringify!(q_size)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<queue_t>())).q_head as *const _ as usize }, 16usize, concat!("Offset of field: ", stringify!(queue_t), "::", stringify!(q_head)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<queue_t>())).q_tail as *const _ as usize }, 32usize, concat!("Offset of field: ", stringify!(queue_t), "::", stringify!(q_tail)));
}
pub type qnl_t = queue_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pool_tag {
    pub pa_threads: atomic_int,
    pub pa_kill: atomic_bool,
    pub p_work: *mut qnl_t,
    pub p_lock: *mut pthread_mutex_t,
    pub p_cond: *mut pthread_cond_t,
}
#[test]
fn bindgen_test_layout_pool_tag() {
    assert_eq!(::std::mem::size_of::<pool_tag>(), 32usize, concat!("Size of: ", stringify!(pool_tag)));
    assert_eq!(::std::mem::align_of::<pool_tag>(), 8usize, concat!("Alignment of ", stringify!(pool_tag)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<pool_tag>())).pa_threads as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(pool_tag), "::", stringify!(pa_threads)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<pool_tag>())).pa_kill as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(pool_tag), "::", stringify!(pa_kill)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<pool_tag>())).p_work as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(pool_tag), "::", stringify!(p_work)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<pool_tag>())).p_lock as *const _ as usize }, 16usize, concat!("Offset of field: ", stringify!(pool_tag), "::", stringify!(p_lock)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<pool_tag>())).p_cond as *const _ as usize }, 24usize, concat!("Offset of field: ", stringify!(pool_tag), "::", stringify!(p_cond)));
}
pub type pool_t = pool_tag;
extern "C" {
    pub fn pool_init(thread_number: ::std::os::raw::c_int) -> *mut pool_t;
}
extern "C" {
    pub fn pool_exec(in_: *mut pool_t, exec_f: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>, arg: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pool_destroy(out: *mut pool_t);
}
