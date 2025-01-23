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
