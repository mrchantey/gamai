use super::*;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let actions1 = my_action_single_parent();
	let str1 = serde_json::to_string_pretty(&actions1)?;
	let actions2 = serde_json::from_str::<ActionGraph>(&str1)?;
	let str2 = serde_json::to_string_pretty(&actions2)?;

	expect(&str1).to_be(&str2)?;

	Ok(())
}
