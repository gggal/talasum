use crate::state_machine::helper::FINAL;
use crate::state_machine::AutomatonNode;

lazy_static! {
    pub static ref START_WS: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (10, &FINAL),
            (1, &LEADING_CR),
            (1, &LEADING_LF),
            (1, &LEADING_TAB),
            (1, &LEADING_SPACE),
            (1, &TRAILING_CR),
            (1, &TRAILING_LF),
            (1, &TRAILING_TAB),
            (1, &TRAILING_SPACE)
        ]);
    static ref LEADING_CR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("\r{}", text))
        .set_cycle(1);
    static ref LEADING_LF: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("\n{}", text))
        .set_cycle(1);
    static ref LEADING_TAB: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("\t{}", text))
        .set_cycle(1);
    static ref LEADING_SPACE: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!(" {}", text))
        .set_cycle(1);
    static ref TRAILING_CR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("{}\r", text))
        .set_cycle(1);
    static ref TRAILING_LF: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("{}\n", text))
        .set_cycle(1);
    static ref TRAILING_TAB: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("{}\t", text))
        .set_cycle(1);
    static ref TRAILING_SPACE: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| format!("{} ", text))
        .set_cycle(1);
}
