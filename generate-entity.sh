for table in "$@"
do
  # Generate entity to temporary directory
  sea-orm-cli generate entity -o ./src/models_temp -t $table

  # Check if file exists in temp file
  if ! test -f ./src/models/$table.rs; then
    # Copy to models
    cp -i ./src/models_temp/$table.rs ./src/models

    # Append to models mod.rs
    textToAppedMod="pub mod $table;"
    if ! grep -q "$textToApped1" "./src/models/mod.rs"; then
      echo $textToApped1 >> ./src/models/mod.rs
    else
      echo "mod.rs -> line already exists"
    fi

    # Append to models prelude.rs
    toCamelCase=`echo $table | sed -r 's/(^|_)([a-z])/\U\2/g'`
    textToAppedPrelude="pub use super::$table::Entity as $toCamelCase;"
    if ! grep -q "$textToApped1" "./src/models/mod.rs"; then
      echo $textToAppedPrelude >> ./src/models/prelude.rs
    else
      echo "prelude.rs -> line already exists"
    fi
  else
    echo "file already exist"
  fi

  # Delete temporary models file
  rm -rf ./src/models_temp
done
