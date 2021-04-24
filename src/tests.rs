#![allow(clippy::non_ascii_literal)]

use crate::basic::from_utf8 as basic_from_utf8;
use crate::compat::from_utf8 as compat_from_utf8;

use std;

fn repeat(ch: u8, len: usize) -> std::vec::Vec<u8> {
    let mut res = std::vec::Vec::with_capacity(len);
    for _ in 0..len {
        res.push(ch);
    }
    res
}

fn test_valid(input: &[u8]) {
    assert!(basic_from_utf8(input).is_ok());
    assert!(compat_from_utf8(input).is_ok());

    #[cfg(feature = "public_imp")]
    test_valid_public_imp(input);
}

#[cfg(feature = "public_imp")]
#[allow(clippy::collapsible_if)]
fn test_valid_public_imp(input: &[u8]) {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        if cfg!(target_feature = "avx2") {
            unsafe {
                assert!(crate::basic::imp::x86::avx2::validate_utf8(input).is_ok());
                assert!(crate::compat::imp::x86::avx2::validate_utf8(input).is_ok());
            }
        }
        if cfg!(target_feature = "sse4.2") {
            unsafe {
                assert!(crate::basic::imp::x86::sse42::validate_utf8(input).is_ok());
                assert!(crate::compat::imp::x86::sse42::validate_utf8(input).is_ok());
            }
        }
    }
}

fn test_invalid(input: &[u8], valid_up_to: usize, error_len: Option<usize>) {
    assert!(basic_from_utf8(input).is_err());
    assert_eq!(
        compat_from_utf8(input).unwrap_err().valid_up_to(),
        valid_up_to
    );
    assert_eq!(compat_from_utf8(input).unwrap_err().error_len(), error_len);

    #[cfg(feature = "public_imp")]
    test_invalid_public_imp(input, valid_up_to, error_len);
}

#[cfg(feature = "public_imp")]
#[allow(clippy::collapsible_if)]
fn test_invalid_public_imp(input: &[u8], valid_up_to: usize, error_len: Option<usize>) {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        if cfg!(target_feature = "avx2") {
            unsafe {
                assert!(crate::basic::imp::x86::avx2::validate_utf8(input).is_err());
                assert_eq!(
                    crate::compat::imp::x86::avx2::validate_utf8(input)
                        .unwrap_err()
                        .valid_up_to(),
                    valid_up_to
                );
                assert_eq!(
                    crate::compat::imp::x86::avx2::validate_utf8(input)
                        .unwrap_err()
                        .error_len(),
                    error_len
                );
            }
        }
        if cfg!(target_feature = "sse4.2") {
            unsafe {
                assert!(crate::basic::imp::x86::sse42::validate_utf8(input).is_err());
                assert_eq!(
                    crate::compat::imp::x86::sse42::validate_utf8(input)
                        .unwrap_err()
                        .valid_up_to(),
                    valid_up_to
                );
                assert_eq!(
                    crate::compat::imp::x86::sse42::validate_utf8(input)
                        .unwrap_err()
                        .error_len(),
                    error_len
                );
            }
        }
    }
}

#[test]
fn simple_valid() {
    test_valid(b"\0");
    test_valid(b"The quick brown fox jumps over the lazy dog");

    // umlauts
    test_valid("öäüÖÄÜß".as_bytes());

    // emojis
    test_valid("❤️✨🥺🔥😂😊✔️👍🥰".as_bytes());

    // Chinese
    test_valid("断用山昨屈内銀代意検瓶調像。情旗最投任留財夜隆年表高学送意功者。辺図掲記込真通第民国聞平。海帰傷芸記築世防橋整済歳権君注。選紙例並情夕破勢景移情誇進場豊読。景関有権米武野範随惑旬特覧刊野。相毎加共情面教地作減関絡。暖料児違歩致本感閉浦出楽赤何。時選権週邑針格事提一案質名投百定。止感右聞食三年外積文載者別。".as_bytes());

    // Japanese
    test_valid("意ざど禁23費サヒ車園オスミト規更ワエ異67事続トソキ音合岡治こ訪京ぴ日9稿がト明安イ抗的ウクロコ売一エコヨホ必噴塗ッ。索墓ー足議需レ応予ニ質県トぴン学市機だほせフ車捕コニ自校がこで極3力イい増娘汁表製ク。委セヤホネ作誌ミマクソ続新ほし月中報制どてびフ字78完りっせが村惹ヨサコ訳器りそ参受草ムタ大移ッけでつ番足ほこン質北ぽのよう応一ア輝労イ手人う再茨夕へしう。".as_bytes());

    // Korean
    test_valid("3인은 대법원장이 지명하는 자를 임명한다, 대통령은 제3항과 제4항의 사유를 지체없이 공포하여야 한다, 제한하는 경우에도 자유와 권리의 본질적인 내용을 침해할 수 없다, 국가는 전통문화의 계승·발전과 민족문화의 창달에 노력하여야 한다.".as_bytes());
}

#[test]
fn simple_invalid() {
    test_invalid(b"\xFF", 0, Some(1));

    // incomplete umlaut
    test_invalid(b"\xC3", 0, None);

    // incomplete emoji
    test_invalid(b"\xF0", 0, None);
    test_invalid(b"\xF0\x9F", 0, None);
    test_invalid(b"\xF0\x9F\x98", 0, None);
}

#[test]
fn incomplete_on_32nd_byte() {
    let mut invalid = repeat(b'a', 31);
    invalid.push(b'\xF0');
    test_invalid(&invalid, 31, None)
}

#[test]
fn incomplete_on_64th_byte() {
    let mut invalid = repeat(b'a', 63);
    invalid.push(b'\xF0');
    test_invalid(&invalid, 63, None)
}

#[test]
fn incomplete_on_64th_byte_65_bytes_total() {
    let mut invalid = repeat(b'a', 63);
    invalid.push(b'\xF0');
    invalid.push(b'a');
    test_invalid(&invalid, 63, Some(1))
}
