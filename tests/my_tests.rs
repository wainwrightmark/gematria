use gematria::core::prelude::*;

use ntest::test_case;
// use rand::{prelude::StdRng, Rng};

#[test_case(1)]
#[test_case(20)]
#[test_case(88)]
fn test_solve(score: u64) {
    let map = NumberMap::bad_words_map();

    let r = map.solve(score);

    assert!(r.is_some());
    let text = r.clone().unwrap().text.clone();
    assert_eq!(r.unwrap().score, score);
    

    println!("{}",text);
}

#[test_case("Mark Wainwright")]
#[test_case("Stephanie Cheung")]
fn test_solve(input: String) {
    let map = NumberMap::bad_words_map();

    let score = Word::from(input.to_string()).score;

    let r = map.solve(score);

    assert!(r.is_some());
    let text = r.clone().unwrap().text.clone();
    assert_eq!(r.unwrap().score, score);
    

    println!("{} ({}) = {}",input, score, text);
}

// #[test_case(1)]
// #[test_case(2)]
// #[test_case(3)]
// #[test_case(4)]
// #[test_case(5)]
// fn test_invert(seed: u64) {
//     let cube = CubieCube::random_cube(seed);

//     let inverse = cube.invert();

//     let mult = cube.multiply(&inverse);

//     assert_eq!(CubieCube::default(), mult);
// }

// #[test]
// fn test_default_to_facelet_cube(){
//     let cube = CubieCube::default();

//     let fc: FaceletCube = cube.clone().into();

//     let cube2:CubieCube = fc.try_into().unwrap();

//     assert_eq!(cube, cube2);
// }

// #[test_case(1)]
// #[test_case(2)]
// #[test_case(3)]
// #[test_case(4)]
// #[test_case(5)]
// fn test_to_facelet_cube(seed: u64){
//     let cube = CubieCube::random_cube(seed);

//     let fc: FaceletCube = cube.clone().into();

//     let cube2:CubieCube = fc.try_into().unwrap();

//     assert_eq!(cube, cube2);
// }



// #[test]
// fn any_move_four_times_returns_same() {
//     for m in Move::ALLMOVES {
//         let base_cube = CubieCube::random_cube(123);
//         let mut current = base_cube.clone();
//         for _ in 0..4 {
//             current = m.apply(&current);
//         }

//         assert_eq!(base_cube, current, "Move {}", m);
//     }
// }

// #[test]
// fn any_move_inverse_is_same_as_three_times() {
//     for m in Move::ALLMOVES {
//         let base_cube = CubieCube::random_cube(123);
//         let mut current = base_cube.clone();
//         for _ in 0..3 {
//             current = m.apply(&current);
//         }

//         let inverse = m.apply(&CubieCube::default()).invert();
//         let inverse_applied = base_cube.multiply(&inverse);

//         assert_eq!(current, inverse_applied, "Move {}", m);
//     }
// }

// #[test_case(0)]
// #[test_case(1)]
// #[test_case(2)]
// #[test_case(3)]
// fn test_ud_edges(seed: u8) {
//     let mut cube = CubieCube::default();

//     let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);
//     let expected_edges = rng.gen_range(0..40320);

//     cube.set_ud_edges(expected_edges);

//     let actual = cube.get_ud_edges();
//     assert!(actual.is_some());
//     assert_eq!(actual.unwrap_or_default(), expected_edges);

//     for m in Move::PHASE2MOVES {
//         let new_cube = m.apply(&cube);

//         let new_actual = new_cube.get_ud_edges();
//         assert!(new_actual.is_some());
//     }
// }

// #[test]
// fn test_create_corners() {
//     let r = CornersProperty::create(&CornersProperty {});

//     for c in r {
//         assert!(c < 40320)
//     }
// }

// #[test]
// fn test_corner_slice_depth() {
//     let moves_source = MovesSource::create();

//     let csd = DataSource::create_corner_slice_depth(&moves_source);

//     for c in csd {
//         assert_ne!(c, u8::MAX)
//     }
// }

// #[test]
// fn test_up_down_edges_conjugation() {
//     let conj = DataSource::create_up_down_edges_conjugation();

//     for c in conj {
//         assert!(c < 40320)
//     }
// }

// #[test]
// fn test_create_corner_symmetries(){
//     let css = CornerSymmetriesSource::create();

//     for class_index in css.corner_class_index{
//         assert!(class_index < 2768)
//     }
// }

// #[test]
// fn test_create_flip_slice_symmetries(){
//     let fss = FlipSliceSource::create();

//     for class_index in fss.flip_slice_class_index{
//         assert!(class_index < 64430)
//     }
// }


// #[test]
// fn test_basic_cubes() {
//     itertools::assert_equal(
//         SYMMETRY_CUBES
//             .into_iter()
//             .map(CoordinateCube::from)
//             .duplicates(),
//         vec![],
//     );
// }

// #[test]
// fn test_inverse_moves() {
//     for m in Move::ALLMOVES {
//         let cube = m.get_cube();
//         let inverse = cube.invert();

//         let product = cube.multiply(&inverse);

//         assert_eq!(product, CubieCube::default());
//     }
// }

// #[test]
// fn test_urf_3() {
//     let cube = URF3_SYMMETRY;
//     let cube2 = cube.multiply(&cube);
//     let cube3 = cube2.multiply(&cube);
//     let inverse = cube.invert();
//     let inverse2 = cube2.invert();

//     assert_eq!(cube3, CubieCube::default(), "Multiplied by self twice");
//     assert_eq!(inverse, cube2, "Inverse");
//     assert_eq!(inverse2, cube, "Inverse2");
// }

// #[test]
// fn test_inverse_cubes() {
//     for i in 0..48 {
//         let product = SYMMETRY_CUBES[i].multiply(&SYMMETRY_CUBES_INVERTED[i]);

//         assert_eq!(product, CubieCube::default(), "Cube {}", i);
//     }
// }

// #[test]
// fn test_inverse_cubes2() {
//     itertools::assert_equal(
//         SYMMETRY_CUBES_INVERTED
//             .into_iter()
//             .map(CoordinateCube::from)
//             .duplicates(),
//         vec![],
//     );
//     let basic_cubes =
//         std::collections::BTreeSet::from_iter(SYMMETRY_CUBES.into_iter().map(CoordinateCube::from));

//     for i in 0..SYMMETRY_CUBES_INVERTED.len() {
//         let inverted_cube = SYMMETRY_CUBES_INVERTED[i].clone().into();

//         assert!(basic_cubes.contains(&inverted_cube), "Cube {}", i);
//     }
// }


