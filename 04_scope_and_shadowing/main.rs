fn main() {
    // ========= Scope ==============
    // Variables have scope. they live in a given block. a block is a collection of statements enclosed by braces "{}".

    let variable1 = 10; // This variable live in block of main function.

    // This is a new block.
    {
        let variable2 = 20; // This variable live in this block.

        // [variable2] can be read only within this scope.
        println!("Value of variable2 is {}", variable2);

        // This block live in block of main function.
        // So [variable1] can be read in this block.
        println!("Value of variable1 is {}", variable1);
    }
    // End of block

    // [variable2] can't be read out of the given block. below line throws error.
    // println!("Value of variable2 is {}", variable2);

    // [variable1] can be read in this block.
    println!("Value of variable1 is {}", variable1);

    // ========== Shadowing ==========
    // Declaring a new variable with the same as previously declared is called "shadowing".

    // ======= Shadowing without scope ========
    let x = 10;
    println!("Value of x is {}", x);
    // Below lines just update the value of [x].
    let x = 20;
    println!("Value of x is {}", x);
    let x = x + 30;
    println!("Value of x is {}", x);

    // ========== Shadowing with scope =======
    let y = 10;

    {
        let y = 20; // Here we are declaring a variable as a previous one. but in child scope.

        // Below line prints the value of [y] within this scope.
        println!("Value of y in scope is {}", y);
    }

    // Below line prints the value of [y] which is in scope of [main] function.
    println!("Value of y out of scope is {}", y);
}