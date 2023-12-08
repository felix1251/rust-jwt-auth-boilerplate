for table in "$@"
do
  # Generate entity to temporary directory
  sea-orm-cli generate entity -o ./src/models_temp -t $table

  # Check if file exists in temp file
  if ! test -f ./src/models/$table.rs; then
    # Copy to models
    cp -i ./src/models_temp/$table.rs ./src/models
  else
    echo "models/$1.rs -> file already exist"
  fi

  # Append to models mod.rs
  if ! grep -q "pub mod $table;" "./src/models/mod.rs"; then
    echo "pub mod $table;" >> ./src/models/mod.rs
  else
    echo "models/mod.rs -> "$1" line already exists"
  fi

  # Append to models prelude.rs
  toCamelCase=`echo $table | sed -r 's/(^|_)([a-z])/\U\2/g'`
  if ! grep -q "pub use super::$table::Entity as $toCamelCase;" "./src/models/prelude.rs"; then
    echo "pub use super::$table::Entity as $toCamelCase;" >> ./src/models/prelude.rs
  else
    echo "models/prelude.rs -> "$toCamelCase" line already exists"
  fi

  # Delete temporary models file
  rm -rf ./src/models_temp
done
