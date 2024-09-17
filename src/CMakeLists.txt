# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
	MANIFEST_PATH service/Cargo.toml
	FEATURES rhai strsim
)
