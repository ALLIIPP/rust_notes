
fn main() {
    //exercise 1
    let temperature_1_celcius = 17.9;
    let temperature_1_fahrenheit = celcius_to_fahrenheit(temperature_1_celcius);
    println!("the temperature {}C in fahrenheit is {}F",temperature_1_celcius,temperature_1_celcius);
    let temperature_2_fahrenheit = 34.93;
    let temperature_2_celcius = fahrenheit_to_celcius(temperature_2_fahrenheit);
    println!("the temperature {}F in celcius is {}C",temperature_2_fahrenheit, temperature_2_celcius);
    //exercise 2
    let fib_level = 17;
    println!("the 9th fibonacci number is {}", fibonacci(fib_level));
    //exercise 3
    sing_christmas_carol();
}

fn celcius_to_fahrenheit(temp:f32) ->f32{
     (temp*9.0/5.0)+32.0
}

fn fahrenheit_to_celcius(temp:f32) ->f32{
     (temp-32.0)*(5.0/9.0)
}

fn fibonacci(n:u32)->u32{
    if(n==0){return 0;}
    if(n==1){return 1;}
    else{
        return fibonacci(n-1)+ fibonacci(n-2)
    }
}

fn sing_christmas_carol(){
   

    let christmas_carol = [["first", "a partridge in a pear tree"],
                            ["second", "two turtle doves"],
                            ["third", "three french hens"],
                            ["fourth", "four calling birds"],
                            ["fifth", "five gold rings"],
                            ["sixth", "six geese-a-laying"],
                            ["seventh", "seven swans-a-swimming"],
                            ["eighth", "eight maids-a-milking"],
                            ["ninth", "nine ladies dancing"],
                            ["tenth","ten lords a-leaping"],
                            ["eleventh", "eleven pipers piping"],
                            ["twelfth","twelve drummers drumming"]
    ];
    println!("-------------------------------------------------------------");
    for i in (0..12){
        println!("on the {} day of christmas my true love gave to me {}",christmas_carol[i][0],christmas_carol[i][1]);
        for j in (0..i).rev(){
            if (j == 0){
                println!("and {}",christmas_carol[j][1]);            
            }else{
                println!("{}",christmas_carol[j][1]);
            }
        }    
        println!("-------------------------------------------------------------");
    }
}

    /* Data Types
        Scalar:(single valued)
            - int
                - negative numbers stored in twos compliment where leftmost bit (sign bit) is (1) for nagative numbers, and
                the value is found by inverting all bits of the positive equivalent and adding one.
                
                - int literals
                    _ decimal:       10_000
                    _ hex:           0xff
                    _ octal:         0o77
                    _ binary:        0b0000_1111
                    _ byte(u8 only): b'A'
            - float
                - floating point numbers represented according to IEEE-754
                    - (1 sign bit, 7 exponent bits, 23 mantissa bits)
                        - sign bit: 0=positive/1=negative
                        - exponent bits: 
                - f32: single precision
                - f64: double precision
            - boolean
                - 1 byte in size
            - char 
                - 4 bytes in size
       Compound:(groups multiple values)
            - tuples
                - fixed length
                - values dont have to be the same
                - elements accessed with "." syntax OR by destructuring
            - array
                - fixed length
                - all elements must be same type
                - elements accessed with hard brackets "[]"
                - "arrays are helpful when you want your data allocated on the stack rather than the heap"
                - "an array is a single chunk of memory allocated on the stack"
                - in other low level languages, when you try to access an element in an array that doesnt exist, they retrn whatever
                is at that memory address (rust doenst do this)

        Expressions/Statements
            - statements: instructions that perform some action and do not return a value
                    - ex. function definitions, assignments
            - expressions: evaluate to a resulting value
                    - does not include ending semicolon 
                    - if you add a semi colon to end of expression it becomes a statement 
                    - ex. values, calling a function, calling a macro, blocks ie.  "{ stuff  }", if blocks, loop blocks, 
                    break (sometimes)

        Funtions
            - ex. fn add_five(i:i32) -> i32{}
            - implicitly returns last expression

        Control FLow
            if Expressions
                - if blocks are expressions
                - let x = if condition {5}else {6} is valid
            loops
                - loop{} 
                    - same as (while(true){})
                - while(contition)
    */
