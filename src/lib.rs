#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

use std::{fs, path::Path};

#[cfg(feature = "main_nro")]
use skyline_web::dialog_ok::DialogOk;

#[macro_use]
extern crate modular_bitfield;

#[macro_use]
extern crate lazy_static;

pub static mut FIGHTER_MANAGER: usize = 0;

use skyline::libc::c_char;
use skyline::nro::{self, NroInfo};
use smash::params::add_hook;
use std::sync::atomic::{AtomicBool, Ordering};
use skyline::hooks::InlineCtx;

unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    ctx.registers[8].set_x(0);
    
}

static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x3810a64, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3747b7c, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

mod util;
// mod template;
mod rayman;
mod bomberman;
mod peppy;
mod toad;




std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0

    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "ultimate-s"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1

    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);
#[no_mangle]
pub extern "C" fn is_ultimate_s() {}

#[no_mangle]
pub extern "C" fn main() {

    //allows online play with added chars
    unsafe {
        if Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libthe_csk_collection.nro").is_file() {
            extern "C" { fn allow_ui_chara_hash_online(ui_chara_hash: u64); }
            allow_ui_chara_hash_online(0xf1062d2e5); //rayman
            allow_ui_chara_hash_online(0xda4cbcb12); //toad
            allow_ui_chara_hash_online(0x12e2fb36c6); //bomberman
            allow_ui_chara_hash_online(smash::hash40("ui_chara_peppy")); //peppy
        }
    }

	util::install();

    rayman::install();
    bomberman::install();
    toad::install();
    peppy::install();

    println!("added chars installed");

    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_peppy");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_rayman");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_bomberman");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toad");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toadette");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_toadsworth");
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_captaintoad");
}



