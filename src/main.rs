// #[macro_use] extern crate prusti_contracts;
// #![no_std]
mod external_spec;
use prusti_contracts::*;

// mod first_impl;
// use first_impl::*;

// mod second_impl;
// use second_impl::*;

mod third_impl;
use third_impl::*;

mod my_layout;
use my_layout::Layout;

// use crate::external_spec::{trusted_option::*};

const U64_MAX: u64 = 18_446_744_073_709_551_615u64;

fn main() {
    // // println!("{}", u64::max_value() >> 62); // Rust caching bug? try shifting by 64, then 63, then 62, then 63 again.
    
    let base_idx = 5;
    let base_addr = 20;
    let page_size = 6;
    let metadata_size = 5;
    let layout_maybe = Layout::from_size_align(2, 4);
    if let Some(layout) = layout_maybe {
        let u = 7u64;
    //     if first_impl::is_bitfield_u64_valid(u) && u < U64_MAX {
    //         let _ = first_impl::first_fit_in_u64(u, base_idx, base_addr, layout, page_size, metadata_size);
    //     };

    //     let test_bitfield_opt = second_impl::TrustedBitfield8::new(8, 64, base_addr, layout, page_size, metadata_size);
    //     match test_bitfield_opt {
    //         Some(test_bitfield) => test_bitfield.first_fit(),
    //         None => None,
        // };

        // let test_bitfield_opt = third_impl::TrustedBitfield8::new(8, 64, base_addr, layout, page_size, metadata_size);
        // match test_bitfield_opt {
        //     Some(test_bitfield) => test_bitfield.first_fit(),
        //     None => None,
        // };
    //     // assert!(test_bitfield_opt);
    //     // let test_bitfield = test_bitfield_opt.unwrap();
    //     // let _ = test_bitfield.first_fit();
    //     // let _ = second_impl::first_fit_in_u64(u, base_idx, base_addr, layout, page_size, metadata_size);
    }
}


// #[requires(layout_size > 0)]
// #[requires(layout_align > 0)]
// #[requires(u < U64_MAX)]
// // #[ensures(result.is_some() ==> {  // The returned index should within the bounds of the current u64.
// //     let (idx, addr) = peek_option(&result);
// //     idx >= base_idx * 64 && idx < base_idx * 64 + 64
// // })]
// #[ensures(result.is_some() ==> {  // The index is mapped to the correct address.
//     let (idx, addr) = peek_option(&result);
//     addr == old(base_addr) + (idx * old(layout_size)) // Ramla: makes no sense why this wouldn't verify
// })]
// #[ensures(result.is_some() ==> {  // Check that the returned address does not overlap with metadata. 
//     let (idx, addr) = peek_option(&result);
//     addr - base_addr <= page_size - metadata_size - layout_size && addr % layout_align == 0
// })]
// fn first_fit_in_u64 (
//     u: u64,
//     base_idx: usize,
//     base_addr: usize,
//     layout_size: usize,
//     layout_align: usize,
//     page_size: usize,
//     metadata_size: usize
// ) -> Option<(usize, usize)> {
//     let first_free = first_fit_idx(u);
//     let idx = base_idx * 64 + first_free;
//     let offset = idx * layout_size;
//     let addr = base_addr + offset;
//     if offset <= (page_size - metadata_size - layout_size) && addr % layout_align == 0 {
//         Some((idx, addr))
//     } else {None}
// }

// fn first_fit_idx(val: u64) -> usize {
//     32
// }