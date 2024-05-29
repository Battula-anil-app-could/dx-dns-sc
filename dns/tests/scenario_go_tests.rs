#[test]
#[ignore = "there are differences in the emitted logs (transferValueOnly)"]
fn test_mandos_main_go() {
    dharitri_sc_scenario::run_go("scenarios/main.scen.json");
}

#[test]
fn test_mandos_resolve_dharitri_go() {
    dharitri_sc_scenario::run_go("scenarios/resolve-dharitri.scen.json");
}

#[test]
#[ignore = "migrateUserName builtin function not implemented yet"]
fn test_mandos_migrate_go() {
    dharitri_sc_scenario::run_go("scenarios/migrate.scen.json");
}
