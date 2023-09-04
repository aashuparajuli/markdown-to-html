input_folder="test"
input_file="test/input.txt"
output_file="test/output.txt"
expected_output_file="test/expected.txt"

cargo run -- $input_file $output_file 1> /dev/null
diff_lines=$(diff $output_file $expected_output_file | wc -l)

if [ ! $diff_lines -eq 0 ]
then
    echo "Test failed: invalid file was generated"
    diff $output_file $expected_output_file
fi