#[test]
fn docs() {
    let file = std::fs::read_to_string("../DOCS.md").unwrap();
    for block in file.split("\n\n").filter(|it| it.contains("```")) {
        let amount = block.lines().count();
        let inner = block.lines().skip(1).take(amount - 2).collect::<Vec<_>>();
        let mut context = crisp::Context::default();
        for input in inner.chunks(2) {
            let output = crisp::parse_and_eval(&input[0][3..], &mut context).unwrap().unwrap();
            assert_eq!(input[1], format!("{}", output[0]));
        }
    }
}
