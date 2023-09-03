cargo run -- $1 $2 1> /dev/null
diff_lines=$(diff $2 $3 | wc -l)

if [ ! $diff_lines -eq 0 ]
then
    echo "Test failed: invalid file was generated"
fi