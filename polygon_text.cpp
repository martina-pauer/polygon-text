#include <iostream>
#include <cstdint>
#include <cstdio>

// Create n edges polygons
using namespace std;
// Using uint8_t use 75% less RAM than unsigned int
void repeat(char character, uint8_t times)
{
    // Write n times a character from 1 to last 8-bits integer
    for (uint8_t time = 1; time <= times; time++)
    {
        cout << character;
    }    
}
// Step 1: Define function for draw linear functions
void line(int8_t inclination, int8_t x, int8_t y)
{
    // Plot linear function with inclination and coordenates using 8-bits integers
    if (inclination == 0)
    { // No inclination
        // Line (-)
        repeat('*', 42);
        repeat('\n', 1);
    }
    else if (inclination > 0)
    { // Positive inclination
        // Up-Slash (/)
        for (int8_t times = 10 + y; times >= 1; times--)
        {
           // Make many spaces like say linear function
            repeat(' ', times * x);
          // Plot dot  
            repeat('*', 1);
         // Write newline for plot the next dots   
            repeat('\n', 1);
        }    
    }
    else
    { // Negative inclination
        // Down-Slash (\)
        int8_t last = 1;
        for (int8_t times = 10 + y; times >= 1; times--)
        {
          // Write previous spaces each time make more
            repeat(' ', last * x);
            repeat('*', 1);
         // Write next spaces after dot each time do less   
            repeat(' ', times * x);
            repeat('\n', 1);
            last += 1;
        }    
    }
}

/*
     8-bits integers data (the quarter of RAM) less than 32-bit type int
     use 1 byte instead of 4 bytes.
*/    
int8_t inclination, edges;

int main()
{
// step 2: Get 3 edges or more for draw the right polygon
cout << "Input 3 or more edges: ";
scanf("%i", &edges);

while (edges < 3)
{
	cout << "Again but well: ";
	scanf("%i", &edges);
}
// Step 3: Draw polygon base
    line(0, 0, 0);
// Step 4: Change edges direction for shape up polygon
    inclination = (edges - 2) * 45;
// Step 5: Draw the line in the next position
    line(inclination, 1, 0);    
// Step 6: Draw line with opposite inclination in the next position
    line(-inclination, 1, 1);
// Step 7: Repeat (edges - 3) times steps 5 and 6 with other positions        
    for (int8_t times = 1; times <= (edges - 3); times++)
    {
        line(inclination, times, times + 1);
        line(-inclination, times, times + 2);
    }    
  // Restart the position for don't move terminal prompt by extra spaces  
    repeat('\r', 1);
	return 0;
}
