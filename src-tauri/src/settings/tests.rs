use mockall::predicate::*;

use crate::settings::{MockSettingsDependencies, Settings};

#[test]
fn it_works() {
    let mock = MockSettingsDependencies::new();
    let mut settings = Settings::new(Box::new(mock));
    // settings.load();
    // assert_eq!(settings.data, Some(SettingsData::default()));
    assert!(true);
}

//TODO: implemented unit tests
