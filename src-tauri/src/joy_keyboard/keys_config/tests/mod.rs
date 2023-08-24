use crate::{joy_keyboard::keys_config::{KeyClickConfig, SingleKeyConfig}, settings_data::{self, EnigoKey}};

use super::KeysConfig;

mod utils;

fn assert_key_click_configs_translate_to_equal(
    settings_key_click_config: settings_data::KeyClickConfig,
    key_click_config: KeyClickConfig) {
    assert_eq!(key_click_config, KeyClickConfig::from(settings_key_click_config));
}


#[test]
fn key_click_config_is_correctly_initialized_key() {
    assert_key_click_configs_translate_to_equal(
        settings_data::KeyClickConfig{
            key: Some(EnigoKey::Tab),
            modifier_1: None,
            modifier_2: None,
            modifier_3: None,
            modifier_4: None,
            modifier_5: None,
            char_key: None,
        },
        KeyClickConfig {
            key: Some(enigo::Key::Tab),
            modifiers: [None,None,None,None,None],
        });
}

fn setup_key_click_config_is_correctly_initialized_char_key(char: char){
    assert_key_click_configs_translate_to_equal(
        settings_data::KeyClickConfig{
            key: None,
            modifier_1: None,
            modifier_2: None,
            modifier_3: None,
            modifier_4: None,
            modifier_5: None,
            char_key: Some(char),
        },
        KeyClickConfig {
            key: Some(enigo::Key::Layout(char)),
            modifiers: [None,None,None,None,None],
        });
}
#[test]
fn key_click_config_is_correctly_initialized_char_key() {
    setup_key_click_config_is_correctly_initialized_char_key('a');
    setup_key_click_config_is_correctly_initialized_char_key('A');
    setup_key_click_config_is_correctly_initialized_char_key('x');
    setup_key_click_config_is_correctly_initialized_char_key('&');
    setup_key_click_config_is_correctly_initialized_char_key('\'');
    setup_key_click_config_is_correctly_initialized_char_key('"');
    setup_key_click_config_is_correctly_initialized_char_key('*');
}

#[test]
fn key_click_config_is_correctly_initialized_key_with_modifiers() {
    assert_key_click_configs_translate_to_equal(
        settings_data::KeyClickConfig{
            key: Some(EnigoKey::Tab),
            modifier_1: Some(EnigoKey::Shift),
            modifier_2: None,
            modifier_3: None,
            modifier_4: None,
            modifier_5: None,
            char_key: None,
        },
        KeyClickConfig {
            key: Some(enigo::Key::Tab),
            modifiers: [Some(enigo::Key::Shift),None,None,None,None],
        });

    assert_key_click_configs_translate_to_equal(
        settings_data::KeyClickConfig{
            key: Some(EnigoKey::RightArrow),
            modifier_1: Some(EnigoKey::Shift),
            modifier_2: Some(EnigoKey::Control),
            modifier_3: None,
            modifier_4: None,
            modifier_5: None,
            char_key: None,
        },
        KeyClickConfig {
            key: Some(enigo::Key::RightArrow),
            modifiers: [Some(enigo::Key::Shift), Some(enigo::Key::Control), None, None, None],
        });
}

#[test]
fn key_click_config_get_key_works_key() {
    assert_eq!(
        KeyClickConfig::get_key(Some(EnigoKey::F1),None),
        Some(enigo::Key::F1)); 
}

#[test]
fn key_click_config_get_key_works_char_key() {
    assert_eq!(
        KeyClickConfig::get_key(None,Some('`')),
        Some(enigo::Key::Layout('`'))); 
}

#[test]
fn key_click_config_to_enigo_key_works_for_all_possible_combinations() {
// NOTE: the name of this test function will be wrong whenever more
    // variants are added to the EnigoKey enum
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Alt),enigo::Key::Alt);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Backspace),enigo::Key::Backspace);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Begin),enigo::Key::Begin);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Break),enigo::Key::Break);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Cancel),enigo::Key::Cancel);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::CapsLock),enigo::Key::CapsLock);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Clear),enigo::Key::Clear);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Control),enigo::Key::Control);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Delete),enigo::Key::Delete);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::DownArrow),enigo::Key::DownArrow);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::End),enigo::Key::End);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Escape),enigo::Key::Escape);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Execute),enigo::Key::Execute);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F1),enigo::Key::F1);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F2),enigo::Key::F2);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F3),enigo::Key::F3);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F4),enigo::Key::F4);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F5),enigo::Key::F5);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F6),enigo::Key::F6);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F7),enigo::Key::F7);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F8),enigo::Key::F8);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F9),enigo::Key::F9);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F10),enigo::Key::F10);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F11),enigo::Key::F11);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F12),enigo::Key::F12);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F13),enigo::Key::F13);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F14),enigo::Key::F14);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F15),enigo::Key::F15);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F16),enigo::Key::F16);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F17),enigo::Key::F17);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F18),enigo::Key::F18);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F19),enigo::Key::F19);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F20),enigo::Key::F20);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F21),enigo::Key::F21);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F22),enigo::Key::F22);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F23),enigo::Key::F23);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F24),enigo::Key::F24);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F25),enigo::Key::F25);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F26),enigo::Key::F26);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F27),enigo::Key::F27);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F28),enigo::Key::F28);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F29),enigo::Key::F29);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F30),enigo::Key::F30);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F31),enigo::Key::F31);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F32),enigo::Key::F32);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F33),enigo::Key::F33);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F34),enigo::Key::F34);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::F35),enigo::Key::F35);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Find),enigo::Key::Find);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Hangul),enigo::Key::Hangul);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Hanja),enigo::Key::Hanja);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Help),enigo::Key::Help);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Home),enigo::Key::Home);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Insert),enigo::Key::Insert);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Kanji),enigo::Key::Kanji);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::LControl),enigo::Key::LControl);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::LeftArrow),enigo::Key::LeftArrow);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Linefeed),enigo::Key::Linefeed);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::LMenu),enigo::Key::LMenu);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::LShift),enigo::Key::LShift);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Meta),enigo::Key::Meta);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::ModeChange),enigo::Key::ModeChange);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Numlock),enigo::Key::Numlock);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Option),enigo::Key::Option);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::PageDown),enigo::Key::PageDown);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::PageUp),enigo::Key::PageUp);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Pause),enigo::Key::Pause);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Print),enigo::Key::Print);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::RControl),enigo::Key::RControl);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Redo),enigo::Key::Redo);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Return),enigo::Key::Return);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::RightArrow),enigo::Key::RightArrow);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::RShift),enigo::Key::RShift);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::ScrollLock),enigo::Key::ScrollLock);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Select),enigo::Key::Select);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::ScriptSwitch),enigo::Key::ScriptSwitch);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Shift),enigo::Key::Shift);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::ShiftLock),enigo::Key::ShiftLock);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Space),enigo::Key::Space);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::SysReq),enigo::Key::SysReq);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Tab),enigo::Key::Tab);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::UpArrow),enigo::Key::UpArrow);
    assert_eq!(KeyClickConfig::to_enigo_key(EnigoKey::Undo),enigo::Key::Undo);
}

#[test]
fn keys_config_is_correctly_initialized(){
    assert_eq!(
        KeysConfig::from(utils::settings_data_keys_config()),
        utils::joy_keyboard_keys_config()
    );
}

#[test]
fn single_key_config_is_correctly_initialized(){
    assert_eq!(
        SingleKeyConfig::from(utils::settings_data_keys_config().south),
        utils::joy_keyboard_keys_config().south
    );
    assert_eq!(
        SingleKeyConfig::from(utils::settings_data_keys_config().east),
        utils::joy_keyboard_keys_config().east
    );
    assert_eq!(
        SingleKeyConfig::from(utils::settings_data_keys_config().north),
        utils::joy_keyboard_keys_config().north
    );
    assert_eq!(
        SingleKeyConfig::from(utils::settings_data_keys_config().west),
        utils::joy_keyboard_keys_config().west
    );
}

