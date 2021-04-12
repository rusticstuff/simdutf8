#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic
)]
// We might want to revisit inline_always
#![allow(clippy::inline_always)]
#![deny(missing_docs)]
#![cfg_attr(feature = "hints", feature(core_intrinsics))]

//! UTF-8 checking crate

#[macro_use]
mod macros;
mod utf8check;

#[cfg(target_feature = "avx2")]
mod avx2;
#[cfg(target_feature = "avx2")]
use crate::avx2::stage1::{SimdInput, SIMDINPUT_LENGTH};

#[cfg(all(target_feature = "sse4.2", not(target_feature = "avx2")))]
mod sse42;
#[cfg(all(target_feature = "sse4.2", not(target_feature = "avx2")))]
use crate::sse42::stage1::{SimdInput, SIMDINPUT_LENGTH};

// We import this as generics
#[cfg(all(not(any(target_feature = "sse4.2", target_feature = "avx2"))))]
mod sse42;
#[cfg(all(not(any(target_feature = "sse4.2", target_feature = "avx2"))))]
use crate::sse42::stage1::{SimdInput, SIMDINPUT_LENGTH};

#[cfg(all(
    not(feature = "allow-non-simd"),
    not(any(target_feature = "sse4.2", target_feature = "avx2"))
))]
fn please_compile_with_a_simd_compatible_cpu_setting_read_the_simdjonsrs_readme() -> ! {}

use crate::utf8check::ProcessedUtfBytes;

use std::mem;

/// Error struct
#[derive(Debug)]
pub struct Utf8Error {}

/// Validates the UTF-8 string
/// # Errors
///
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn validate_utf8(input: &[u8]) -> std::result::Result<&str, Utf8Error> {
    unsafe {
        let len = input.len();
        let mut state = SimdInput::new_utf8_checking_state();
        let lenminus64: usize = if len < 64 { 0 } else { len as usize - 64 };
        let mut idx: usize = 0;

        while idx < lenminus64 {
            /*
            #ifndef _MSC_VER
              __builtin_prefetch(buf + idx + 128);
            #endif
             */
            let input = SimdInput::new(input.get_unchecked(idx as usize..));
            input.check_utf8(&mut state);
            idx += SIMDINPUT_LENGTH;
        }

        if idx < len {
            let mut tmpbuf: [u8; SIMDINPUT_LENGTH] = [0x20; SIMDINPUT_LENGTH];
            tmpbuf
                .as_mut_ptr()
                .copy_from(input.as_ptr().add(idx), len as usize - idx);
            let input = SimdInput::new(&tmpbuf);

            input.check_utf8(&mut state);
            if SimdInput::check_utf8_errors(&state) {
                return Err(Utf8Error {});
            }
        }
        SimdInput::check_eof(&mut state);
        if SimdInput::check_utf8_errors(&state) {
            Err(Utf8Error {})
        } else {
            Ok(std::str::from_utf8_unchecked(input))
        }
    }
}

pub(crate) type Utf8CheckingState<T> = ProcessedUtfBytes<T>;

#[cfg(test)]
mod tests {
    use super::validate_utf8;

    #[test]
    fn simple_correct() {
        assert!(validate_utf8(b"\0").is_ok());
        assert!(validate_utf8(b"The quick brown fox jumps over the lazy dog").is_ok());

        // umlauts
        assert!(validate_utf8("öäüÖÄÜß".as_bytes()).is_ok());

        // emojis
        assert!(validate_utf8("❤️✨🥺🔥😂😊✔️👍🥰".as_bytes()).is_ok());

        // Chinese
        assert!(validate_utf8("断用山昨屈内銀代意検瓶調像。情旗最投任留財夜隆年表高学送意功者。辺図掲記込真通第民国聞平。海帰傷芸記築世防橋整済歳権君注。選紙例並情夕破勢景移情誇進場豊読。景関有権米武野範随惑旬特覧刊野。相毎加共情面教地作減関絡。暖料児違歩致本感閉浦出楽赤何。時選権週邑針格事提一案質名投百定。止感右聞食三年外積文載者別。".as_bytes()).is_ok());

        // Japanese
        assert!(validate_utf8("意ざど禁23費サヒ車園オスミト規更ワエ異67事続トソキ音合岡治こ訪京ぴ日9稿がト明安イ抗的ウクロコ売一エコヨホ必噴塗ッ。索墓ー足議需レ応予ニ質県トぴン学市機だほせフ車捕コニ自校がこで極3力イい増娘汁表製ク。委セヤホネ作誌ミマクソ続新ほし月中報制どてびフ字78完りっせが村惹ヨサコ訳器りそ参受草ムタ大移ッけでつ番足ほこン質北ぽのよう応一ア輝労イ手人う再茨夕へしう。".as_bytes()).is_ok());

        // Korean
        assert!(validate_utf8("3인은 대법원장이 지명하는 자를 임명한다, 대통령은 제3항과 제4항의 사유를 지체없이 공포하여야 한다, 제한하는 경우에도 자유와 권리의 본질적인 내용을 침해할 수 없다, 국가는 전통문화의 계승·발전과 민족문화의 창달에 노력하여야 한다.".as_bytes()).is_ok());
    }

    #[test]
    fn simple_incorrect() {
        assert!(validate_utf8(b"\xFF").is_err());

        // incomplete umlaut
        assert!(validate_utf8(b"\xC3").is_err());

        // incomplete emoji
        assert!(validate_utf8(b"\xF0").is_err());
        assert!(validate_utf8(b"\xF0\x9F").is_err());
        assert!(validate_utf8(b"\xF0\x9F\x98").is_err());
    }

    #[test]
    fn incomplete_on_32nd_byte() {
        let mut invalid = b"a".repeat(31);
        invalid.push(b'\xF0');
        assert!(validate_utf8(&invalid).is_err());
    }

    #[test]
    fn incomplete_on_64th_byte() {
        let mut invalid = b"a".repeat(63);
        invalid.push(b'\xF0');
        assert!(validate_utf8(&invalid).is_err());
    }
}
