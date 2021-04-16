#![allow(clippy::non_ascii_literal)]

use super::{from_utf8, from_utf8_exact};

fn test_valid(input: &[u8]) {
    assert!(from_utf8(input).is_ok());
    assert!(from_utf8_exact(input).is_ok());
}

fn test_invalid(input: &[u8], valid_up_to: usize) {
    assert!(from_utf8(input).is_err());
    assert_eq!(
        from_utf8_exact(input).unwrap_err().valid_up_to(),
        valid_up_to
    );
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
    test_invalid(b"\xFF", 0);

    // incomplete umlaut
    test_invalid(b"\xC3", 0);

    // incomplete emoji
    test_invalid(b"\xF0", 0);
    test_invalid(b"\xF0\x9F", 0);
    test_invalid(b"\xF0\x9F\x98", 0);
}

#[test]
fn incomplete_on_32nd_byte() {
    let mut invalid = b"a".repeat(31);
    invalid.push(b'\xF0');
    test_invalid(&invalid, 31)
}

#[test]
fn incomplete_on_64th_byte() {
    let mut invalid = b"a".repeat(63);
    invalid.push(b'\xF0');
    test_invalid(&invalid, 63)
}

#[test]
fn incomplete_on_64th_byte_65_bytes_total() {
    let mut invalid = b"a".repeat(63);
    invalid.push(b'\xF0');
    invalid.push(b'a');
    test_invalid(&invalid, 63)
}
