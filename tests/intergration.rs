mod core {
    use actions_toolkit::core as actions_core;
    include!("../packages/core/tests/intergration.rs");
}

mod exec {
    use actions_toolkit::exec as actions_exec;
    include!("../packages/exec/tests/intergration.rs");
}
