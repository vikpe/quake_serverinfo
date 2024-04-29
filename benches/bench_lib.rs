use criterion::{Criterion, criterion_group, criterion_main, Throughput};

fn lib_benchmark(c: &mut Criterion) {
    const INFO_STR: &str = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\maxspectators\12\*admin\alpha <alpha@foo.com>\ktxver\1.42\sv_antilag\2\needpass\4\*gamedir\qw\mode\1on1\*qvm\so\*progs\so\maxclients\2\timelimit\10\deathmatch\3\map\aerowalk"#;

    let mut group = c.benchmark_group("lib");
    group.throughput(Throughput::Bytes(INFO_STR.len() as u64));

    group.bench_function("str::to_hashmap", |b| {
        b.iter(|| quake_serverinfo::str::to_hashmap(INFO_STR));
    });
    group.bench_function("Serverinfo::from_str", |b| {
        b.iter(|| quake_serverinfo::Serverinfo::from_str(INFO_STR));
    });
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
