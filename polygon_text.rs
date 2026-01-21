use std::io;

fn repeat(character: &str, times : u8)
{
	// Repeat many times one character from 1 to more times
	let mut time : u8 = 1;
	
	while time <= times
	{
	  /*
	    Rust haven't range function or for loop C-like	
	    use while with external counter for get the same
	   */
		print!("{}", character);
		time = time + 1;
	}
}
// Step 1: Define function for plot linear functions
fn line(inclination : i8, x : u8, y : u8)
{
	// Draw a line using text in terminal
	/*

	1	2	3

	4	-------#-----------

	3	----#--------------

	2	#------------------

	1	-------------------

	line(3, 1, 2)
*/
	if inclination == 0
	{
	  // Inclination equal to zero a constant line (-)
		repeat("*", 42);
		repeat("\n", 1);
	}
	else if inclination > 0
	{
		// Inclination positive an up-slash (/)
		let mut counter : u8 = 10 + y;
		
		while counter >= 1
		{
			repeat(" ", counter * x);
			repeat("*\n", 1);
			counter = counter - 1;
		}	
		
	}
	else
	{
		// Inclination negative a down-slash (\)
		let mut times : u8 = 10 + y;
		let mut last : u8 = 1;
		
		while times >= 1
		{
			// Adjust to the linear function working
			repeat(" ", last * x);
			repeat("*", 1);
			repeat(" ", times * x);
			repeat("\n", 1);
		  // Use variable feedback interchange for get needed spaces
			last = last + 1;
			times = times - 1
	    }	
	}		
}

fn main()
{
// Step 2: Get how much edges must have the polygon to draw from 3 edges
	let mut input = String :: new();
	println!("Input edges");
	io::stdin().read_line(&mut input).expect("Bad reading");
	// Convert String to u8
	let edges : u8 = input.trim().parse().expect("Bad Type");
    // Creative Way of data validation: Use recursivity of main function
	 if edges != 3
	 {
	    // Give message when the data is not valid and recall to the function
	        println!("Must Be 3 or more edges");
	        main();
	 }
	 else
	 { 
	   // When the data is right run the rest of the algorithm
	    // Step 3: Create base line, floor of the polygon
	        line(0, 0, 0);
        // Step 4: Change line direction with (edges - 2) * 45deg
	        let inclination : i8 = (edges.wrapping_sub(2)).wrapping_mul(45) as i8;
	   // Step 5: Draw the line from next position
	        line(inclination, 0, 1);
	  // Step 6: Draw line with negative inclination in next position
	        line(inclination.wrapping_mul(0 - 1), 1, 1);
	 // Step 7: Repeat (edges - 3) times steps 5 and 6 with others position 
	        let mut times : u8 = 1;
	        
	        while times <= (edges - 3)
	        {
	                line(inclination, times + 1, times);
	                line(inclination.wrapping_mul(0 - 1), times + 2, times);
	                times = times + 1;
	        }                     
	 }   
}
