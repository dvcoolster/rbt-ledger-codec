use rbtcore::alpha_flow::{AlphaFlowEncode, NoopCoder};

#[test]
fn roundtrip_noop() {
    let data = b"RBT codec test bytes";
    let coder = NoopCoder::default();
    let encoded = coder.encode(data);
    let decoded = coder.decode(&encoded);
    assert_eq!(decoded, data);
} 