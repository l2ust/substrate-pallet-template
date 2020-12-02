// --- substrate ---
use frame_support::{assert_noop, assert_ok};
// --- template ---
use crate::{mock::*, *};

#[test]
fn template_work_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(Template::template_call(Origin::signed(1), Ok(())));
	});
}

#[test]
fn template_fail_test() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Template::template_call(
				Origin::signed(1),
				Err(<Error<Test, _>>::TemplateError.into())
			),
			<Error<Test, _>>::TemplateError
		);
	});
}
