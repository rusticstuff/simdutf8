use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simdutf8::*;

fn criterion_benchmark(c: &mut Criterion) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(*core_ids.get(2).unwrap_or(&core_ids[0]));

    c.bench_function("single ascii char", |b| {
        b.iter(|| validate_utf8(black_box(b"a")))
    });
    c.bench_function("single ascii char - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(b"a")))
    });

    let umlaut = "ö".as_bytes();
    c.bench_function("single umlaut", |b| {
        b.iter(|| validate_utf8(black_box(umlaut)))
    });
    c.bench_function("single umlaut - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(umlaut)))
    });

    let eight_umlauts_string = str::repeat("ö", 8);
    let eight_umlauts = eight_umlauts_string.as_bytes();
    c.bench_function("eight umlauts", |b| {
        b.iter(|| validate_utf8(black_box(eight_umlauts)))
    });
    c.bench_function("eight umlauts - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(eight_umlauts)))
    });

    let thirtytwo_umlauts_string = str::repeat("ö", 32);
    let thirtytwo_umlauts = thirtytwo_umlauts_string.as_bytes();
    c.bench_function("32 umlauts", |b| {
        b.iter(|| validate_utf8(black_box(thirtytwo_umlauts)))
    });
    c.bench_function("32 umlauts - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(thirtytwo_umlauts)))
    });

    let fox = b"The quick brown fox jumps over the lazy dog";
    c.bench_function("fox", |b| b.iter(|| validate_utf8(black_box(fox))));
    c.bench_function("fox - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(fox)))
    });
    let mut fox_long = fox.to_vec();
    for _ in 0..1000 {
        fox_long.write_all(fox).unwrap();
    }
    let fox_long = fox_long.as_slice();
    c.bench_function("fox_long", |b| {
        b.iter(|| validate_utf8(black_box(fox_long)))
    });
    c.bench_function("fox_long - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(fox_long)))
    });

    let german_fox = "Falsches Üben von Xylophonmusik quält jeden größeren Zwerg.".as_bytes();
    c.bench_function("german_fox", |b| {
        b.iter(|| validate_utf8(black_box(german_fox)))
    });
    c.bench_function("german_fox - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(german_fox)))
    });
    let mut german_fox_long = german_fox.to_vec();
    for _ in 0..1000 {
        german_fox_long.write_all(german_fox).unwrap();
    }
    let german_fox_long = german_fox_long.as_slice();
    c.bench_function("german_fox_long", |b| {
        b.iter(|| validate_utf8(black_box(german_fox_long)))
    });
    c.bench_function("german_fox_long - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(german_fox_long)))
    });

    let chinese = "断用山昨屈内銀代意検瓶調像。情旗最投任留財夜隆年表高学送意功者。辺図掲記込真通第民国聞平。海帰傷芸記築世防橋整済歳権君注。選紙例並情夕破勢景移情誇進場豊読。景関有権米武野範随惑旬特覧刊野。相毎加共情面教地作減関絡。暖料児違歩致本感閉浦出楽赤何。時選権週邑針格事提一案質名投百定。止感右聞食三年外積文載者別。".as_bytes();
    c.bench_function("chinese", |b| b.iter(|| validate_utf8(black_box(chinese))));
    c.bench_function("chinese - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(chinese)))
    });

    let japanese = "意ざど禁23費サヒ車園オスミト規更ワエ異67事続トソキ音合岡治こ訪京ぴ日9稿がト明安イ抗的ウクロコ売一エコヨホ必噴塗ッ。索墓ー足議需レ応予ニ質県トぴン学市機だほせフ車捕コニ自校がこで極3力イい増娘汁表製ク。委セヤホネ作誌ミマクソ続新ほし月中報制どてびフ字78完りっせが村惹ヨサコ訳器りそ参受草ムタ大移ッけでつ番足ほこン質北ぽのよう応一ア輝労イ手人う再茨夕へしう。".as_bytes();
    c.bench_function("japanese", |b| {
        b.iter(|| validate_utf8(black_box(japanese)))
    });
    c.bench_function("japanese - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(japanese)))
    });

    let korean = "3인은 대법원장이 지명하는 자를 임명한다, 대통령은 제3항과 제4항의 사유를 지체없이 공포하여야 한다, 제한하는 경우에도 자유와 권리의 본질적인 내용을 침해할 수 없다, 국가는 전통문화의 계승·발전과 민족문화의 창달에 노력하여야 한다.".as_bytes();
    c.bench_function("korean", |b| b.iter(|| validate_utf8(black_box(korean))));
    c.bench_function("korean - stdlib", |b| {
        b.iter(|| std::str::from_utf8(black_box(korean)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
