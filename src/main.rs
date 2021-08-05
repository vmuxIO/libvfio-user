extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_void;
use std::str;

extern crate libvfio_user_sys;

use std::env;
use std::ptr;

use libvfio_user_sys::vfu_ctx_t;
use libvfio_user_sys::vfu_create_ctx;
use libvfio_user_sys::vfu_setup_log;
use libvfio_user_sys::vfu_pci_init;
use libvfio_user_sys::vfu_setup_region;
use libvfio_user_sys::vfu_pci_set_id;
use libvfio_user_sys::vfu_trans_t_VFU_TRANS_SOCK;
use libvfio_user_sys::vfu_dev_type_t_VFU_DEV_TYPE_PCI;
use libvfio_user_sys::vfu_pci_type_t_VFU_PCI_TYPE_CONVENTIONAL;
use libvfio_user_sys::VFU_PCI_DEV_BAR2_REGION_IDX;
use libvfio_user_sys::VFU_REGION_FLAG_RW;
use libvfio_user_sys::vfu_dev_irq_type_VFU_DEV_INTX_IRQ;
use libvfio_user_sys::vfu_realize_ctx;
use libvfio_user_sys::vfu_attach_ctx;
use libvfio_user_sys::vfu_run_ctx;

fn main() {
    let c_str = CString::new("/var/run/vfio-user.sock").unwrap();
    let sock: *const c_char = c_str.as_ptr() as *const c_char;
    let p: *mut c_void = ptr::null_mut();
    unsafe {
        // FIXME check return values
        let vfu_ctx = vfu_create_ctx(vfu_trans_t_VFU_TRANS_SOCK, sock, 0, p,
                                     vfu_dev_type_t_VFU_DEV_TYPE_PCI);
	let mut ret = vfu_pci_init(vfu_ctx,
            libvfio_user_sys::vfu_pci_type_t_VFU_PCI_TYPE_CONVENTIONAL,
            0, // FIXME PCI_HEADER_TYPE_NORMAL
            0);
        libvfio_user_sys::vfu_pci_set_id(vfu_ctx, 0x494f, 0x0dc8, 0x0, 0x0);
        // FIXME setup region
        ret = libvfio_user_sys::vfu_setup_device_nr_irqs(vfu_ctx,
            vfu_dev_irq_type_VFU_DEV_INTX_IRQ, 1);
        ret = libvfio_user_sys::vfu_realize_ctx(vfu_ctx);
        ret = libvfio_user_sys::vfu_attach_ctx(vfu_ctx);
	ret = libvfio_user_sys::vfu_run_ctx(vfu_ctx);
    }
}

