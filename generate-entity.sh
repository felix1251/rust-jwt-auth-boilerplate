# Generate entity to temporary directory
sea-orm-cli generate entity -o ./src/models_temp -t $1

# Copy to models
cp -i ./src/models_temp/$1.rs ./src/models

# Append to models mod.rs
echo "pub mod $1;" >> ./src/models/mod.rs

# Append to models mod.rs
echo "pub use super::$1::Entity as $2;" >> ./src/models/prelude.rs

# Delete temporary models file
rm -rf ./src/models_temp
