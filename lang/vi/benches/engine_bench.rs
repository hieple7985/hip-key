use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hip_key_core::{Engine, Keystroke, LanguagePack};
use hip_key_lang_vi::{Vietnamese, InputMethod};

fn bench_engine_process_keystroke(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_process");

    group.bench_function("single_char", |b| {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(Vietnamese::new()));
        b.iter(|| {
            engine.clear();
            let _ = engine.process(black_box(&Keystroke::char('a')));
        });
    });

    group.bench_function("telex_word", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.set_language_pack(Box::new(Vietnamese::new()));
            for ch in "chaof".chars() {
                let _ = engine.process(black_box(&Keystroke::char(ch)));
            }
        });
    });

    group.bench_function("backspace", |b| {
        let mut engine = Engine::new();
        engine.set_language_pack(Box::new(Vietnamese::new()));
        for ch in "xin chao".chars() {
            let _ = engine.process(&Keystroke::char(ch));
        }
        b.iter(|| {
            let _ = engine.process(black_box(&Keystroke::backspace()));
        });
    });

    group.finish();
}

fn bench_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer");

    group.bench_function("append", |b| {
        let mut buf = hip_key_core::Buffer::new();
        b.iter(|| {
            buf.append(black_box('a'));
        });
    });

    group.bench_function("commit", |b| {
        b.iter(|| {
            let mut buf = hip_key_core::Buffer::new();
            buf.append('x');
            buf.append('i');
            buf.append('n');
            buf.commit();
        });
    });

    group.finish();
}

fn bench_telex_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("telex");

    group.bench_function("simple_vowel", |b| {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        b.iter(|| vi.convert_telex(black_box("aw")));
    });

    group.bench_function("toned_vowel", |b| {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        b.iter(|| vi.convert_telex(black_box("aws")));
    });

    group.bench_function("word", |b| {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        b.iter(|| vi.convert_telex(black_box("chaof")));
    });

    group.bench_function("sentence", |b| {
        let vi = Vietnamese::with_method(InputMethod::Telex);
        b.iter(|| {
            vi.convert_telex(black_box("xin chaof Vietjs Nam"))
        });
    });

    group.finish();
}

fn bench_vni_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("vni");

    group.bench_function("simple_vowel", |b| {
        let vi = Vietnamese::with_method(InputMethod::VNI);
        b.iter(|| vi.convert_vni(black_box("a8")));
    });

    group.bench_function("word", |b| {
        let vi = Vietnamese::with_method(InputMethod::VNI);
        b.iter(|| vi.convert_vni(black_box("chao2")));
    });

    group.finish();
}

fn bench_dictionary_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary");

    group.bench_function("suggest_short_prefix", |b| {
        let vi = Vietnamese::new();
        b.iter(|| vi.generate_candidates(black_box("c")));
    });

    group.bench_function("suggest_medium_prefix", |b| {
        let vi = Vietnamese::new();
        b.iter(|| vi.generate_candidates(black_box("xin")));
    });

    group.bench_function("suggest_no_match", |b| {
        let vi = Vietnamese::new();
        b.iter(|| vi.generate_candidates(black_box("zzzzz")));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_engine_process_keystroke,
    bench_buffer_operations,
    bench_telex_conversion,
    bench_vni_conversion,
    bench_dictionary_lookup
);
criterion_main!(benches);
