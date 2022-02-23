
echo 'Starting time measurement'
alias urldecode_alias='sed "s@+@@g; s@%@\\\\x@g" | xargs -0 printf "%b"'
examplestring='http%3A%2F%2Fwww%2Eexample%2Enet%2Findex%2Ehtml%3Fsession%3DA54C6FE2%23info'

sum=0
duration=0
number_of_tests=5000

sum_rust=0
duration_rust=0

TIMEFORMAT=%R

for counter in $(seq 1 $number_of_tests); do  

    duration="$(time ( echo $examplestring | sed "s@+@@g; s@%@\\\\x@g" | xargs -0 printf "%b" ) 2>&1 1>/dev/null )"
    duration="$(echo "$duration" | tr ',' '.' | bc -l)"

    echo "[$counter] Duration $duration sec"
    sum=$(echo "$sum+$duration" | bc -l)
    #echo "sum: $sum"

done;

for counter in $(seq 1 $number_of_tests); do 

    duration_rust="$(time ( ../target/release/urldecode $examplestring ) 2>&1 1>/dev/null )"
    duration_rust="$(echo "$duration_rust" | tr ',' '.' | bc -l)"

    echo "[$counter] Duration_rust $duration_rust sec"
    sum_rust=$(echo "$sum_rust+$duration_rust" | bc -l)
    #echo "sum: $sum"

done;

echo "Number of tests: $number_of_tests"
echo "Mean duration sed:"
echo "$sum / $number_of_tests" | bc -l

echo "Mean duration Rust :"
echo "$sum_rust / $number_of_tests" | bc -l