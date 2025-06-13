use ansx::*;

#[test]
fn roundtrip_identity() {
    let src = b"hello world";
    let mut len: u32 = 0;
    unsafe {
        let ptr = ansx_encode(src.as_ptr(), src.len() as _, &mut len as *mut _);
        // encode allocates new buffer, ensure len matches
        assert_eq!(len as usize, src.len());
        let slice = std::slice::from_raw_parts(ptr, len as usize);
        assert_eq!(&slice[..], src);
        // free
        ansx_free(ptr.cast(), len);
    }
} 