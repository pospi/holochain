use holochain_test_conductor::TestConductor;

fn main() {
    // // TBD InlineZome defined in pure rust, no wasm compilation
    // let zome = InlineZome::new().function(|api, i| Ok(()));
    // let dna = DnaDef::from_zomes(hashmap! {"zome1" => vec![zome]});
    // // Not from DnaFile, meaning `install_dna` will not be called
    // let app = TestAppSetup::from_dna("app1", hashmap! {"cell1" => dna})
    let dna_file = DnaFile::load("path/to")?;
    let app = TestAppSetup::from_dna_files("app1", hashmap! {"cell1" => dna_file});
    let setup = TestConductorSetup { apps: vec![app] };
    let conductor1 = TestConductor::from_setup(setup);
    let conductor2 = TestConductor::from_setup(setup);

    conductor1.call_zome("app1", "cell1", "zome1", "fn1", 42)?;
    conductor1.run("app1", "cell1", "zome1", |api| {
        api.create(todo!())?;
        api.create_link(todo!())?;
    })?;
}
