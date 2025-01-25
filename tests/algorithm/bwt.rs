use cmprsd::bwt::BWT;

#[test]
pub fn bw_tranform() {
    let test_case = "abraca";
    let tranformed = BWT::transform(test_case);
    assert_eq!(test_case.len(), tranformed.transformed_string.len());
    let expected_transformed = "caraab";
    assert_eq!(tranformed.transformed_string, expected_transformed);
    assert_eq!(tranformed.index, 1)
}

#[test]
pub fn bw_tranform_back() {
    let test_case = "abraca";
    let back_to_original= BWT::transform(test_case).inverse_tranform();
    assert_eq!(test_case, back_to_original)
}
