input_folder="example"
input_file="${input_folder}/input.txt"
output_file="${input_folder}/output.txt"
expected_output_file="${input_folder}/expected.txt"

cargo run -- $input_file $output_file 2> /dev/null
diff_lines=$(diff $output_file $expected_output_file | wc -l)

if [ ! $diff_lines -eq 0 ]
then
    echo "Test failed: invalid file was generated"
    diff $output_file $expected_output_file
else
    echo "The output file was correct"
fi