#![no_std]

#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;

extern "C" {
    fn ocall_sgx_qe_get_quote(
        err: *mut sgx_quote3_error_t,
        report: *const sgx_report_t,
        buf: *mut u8,
        buf_len: *mut u32,
        buf_cap: u32,
    ) -> sgx_status_t;

    fn ocall_sgx_qe_get_target_info(
        err: *mut sgx_quote3_error_t,
        qe_target: *mut sgx_target_info_t,
    ) -> sgx_status_t;
}

#[no_mangle]
pub extern "C" fn ecall_new_quote(buf: *mut u8, buf_len: *mut u32, buf_cap: u32) -> u32 {
    let qe3_target = {
        let mut out = sgx_target_info_t::default();
        let mut err = sgx_quote3_error_t::SGX_QL_SUCCESS;
        let status = unsafe {
            ocall_sgx_qe_get_target_info(
                &mut err as *mut sgx_quote3_error_t,
                &mut out as *mut sgx_target_info_t,
            )
        };
        if status != sgx_status_t::SGX_SUCCESS {
            println!("ocall_sgx_qe_get_target_info with status: {:?}", status);
            return status as u32;
        }
        if err != sgx_quote3_error_t::SGX_QL_SUCCESS {
            println!("ocall_sgx_qe_get_target_info with error: {:?}", err);
            return err as u32;
        }
        out
    };

    let report = {
        let data = sgx_report_data_t::default();
        let mut out = sgx_report_t::default();

        let status = unsafe { sgx_create_report(&qe3_target, &data, &mut out) };
        if status != sgx_status_t::SGX_SUCCESS {
            println!("sgx_create_report: {:?}", status);
            return status as u32;
        }

        out
    };
    println!("1");

    {
        let mut err = sgx_quote3_error_t::default();
        let status = unsafe { ocall_sgx_qe_get_quote(&mut err, &report, buf, buf_len, buf_cap) };
        if status != sgx_status_t::SGX_SUCCESS {
            println!("ocall_sgx_qe_get_quote fail with status: {:?}", status);
            return status as u32;
        }
        if err != sgx_quote3_error_t::SGX_QL_SUCCESS {
            println!("ocall_sgx_qe_get_quote fail with error: {:?}", err);
            return err as u32;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn ecall_new_report(
    report: *mut sgx_report_t,
    qe3_target: *const sgx_target_info_t,
) -> sgx_status_t {
    let data = sgx_report_data_t::default();

    unsafe { sgx_create_report(qe3_target, &data as *const sgx_report_data_t, report) }
}
