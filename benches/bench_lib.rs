use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn lib_benchmark(c: &mut Criterion) {
    let serverinfo = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\maxspectators\12\*admin\alpha <alpha@foo.com>\ktxver\1.42\sv_antilag\2\needpass\4\*gamedir\qw\mode\1on1\*qvm\so\*progs\so\maxclients\2\timelimit\10\deathmatch\3\map\aerowalk"#;

    let mut group = c.benchmark_group("lib");
    group.throughput(Throughput::Bytes(serverinfo.len() as u64));
    group.bench_function("to_hashmap", |b| {
        b.iter(|| quake_serverinfo::to_hashmap(serverinfo))
    });
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
