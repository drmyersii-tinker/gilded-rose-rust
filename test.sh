# run the tests
printf "\n####\n##\n## Running Tests\n##\n####\n\n"
cargo test

# capture output
cargo run > output.log

# compare original to current
printf "\n####\n##\n## Running Diff\n##\n####\n\n"
diff original_output.log output.log
