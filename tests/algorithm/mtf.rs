use cmprsd::mtf::MTF;

#[test]
pub fn mtf_tranform() {}

#[test]
pub fn mtf_tranform_back() {
    let test_case = "abraca";
    let back_to_original = MTF::transform(test_case).inverse_transform();
    assert_eq!(test_case, back_to_original)
}
