use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use valhalla::{Actor, Config, LatLon, proto};

const ANDORRA_CONFIG: &str = "tests/andorra/config.json";
const ANDORRA_TEST_LOC_1: LatLon = LatLon(42.50107335756198, 1.510341967860551); // Sant Julia de Loria
const ANDORRA_TEST_LOC_2: LatLon = LatLon(42.50627089323736, 1.521734167223563); // Andorra la Vella

fn bench_route(c: &mut Criterion) {
    let config = Config::from_file(ANDORRA_CONFIG).unwrap();
    let mut actor = Actor::new(&config).unwrap();

    c.bench_function("small_route", |b| {
        let request = proto::Api {
            options: Some(proto::Options {
                costing_type: proto::costing::Type::Auto as i32,
                locations: vec![
                    proto::Location {
                        ll: Some(ANDORRA_TEST_LOC_1.into()),
                        ..Default::default()
                    },
                    proto::Location {
                        ll: Some(ANDORRA_TEST_LOC_2.into()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        };

        b.iter(|| {
            let response = actor.route(black_box(&request)).unwrap();
            black_box(response)
        });
    });
}

criterion_group!(benches, bench_route);
criterion_main!(benches);
