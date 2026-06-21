let mut manager =
    simulator::manager::SimulatorManager::new();

for resolved in configs.resolve_all()? {
    manager.add_instance(resolved);
}

println!("Loaded simulator instances:");
println!("{:#?}", manager);

for instance in configs.instances.values() {
    manager
    .start_instance(&instance.id)
    .unwrap();
}

println!(
    "Started instance:"
);

for id in manager.ids() {
    manager.start_instance(&id)?;
}
