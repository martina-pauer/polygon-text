#!/usr/bin/python3

def repeat(character: str = ' ', times: int = 1) -> str:
    '''
        Show a character (space by default)
        n times (from 1 to the last integer)
    '''
    text: str = ''
    for show in range(1, times + 1):
        text += character
    # Give text part for show right after
    return text
# Step 1: Define function for draw the lines of 2D polygon        
def line(inclination: int, x: int, y: int):
    '''
        Plot linear functions
    '''
    plot = ''
    if inclination > 0:
        # Make up inclination Up-Slash (/)
        time = y + 10
        while(time >= 1):
            # Spaces before dot
            plot += repeat(times = time * x)
            # First dot
            plot += repeat('*', 1)
            plot += '\n'
            # Use less spaces progressive way
            time -= 1
    elif inclination == 0:
      # Constant line (-)
        plot += repeat('*', 42)
        plot += '\n'
    else:
      # Down-Slash (\)
        last = 1
        first = y + 10
        while(first >= 1):
          # Before spaces each time more
            plot += repeat(times = last * x)
            plot += repeat('*', 1)
         # After spaces each time less   
            plot += repeat(times = first * x)
            plot += '\n'
        # Variable feedback for fix spaces
            last += 1
            first -= 1
  # Print text the automatic newline is so dangerous
    print(plot)       
# Step 2: Get 3 or more edges for draw polygon of that lines
edges: int = int(input('Write 3 or more edges: '))

while (edges < 3):
	edges: int = int(input('\n3 or more: '))
# Step 4: Change edges direction for shaping the polygon
inclination: int = (edges - 2) * 45
# Step 5: Draw line with that direction and next position
line(inclination, 1, 0)
# Step 6: Draw line with opposite inclination in next position
line(-inclination, 1, 1)
# Step 7: Repeat (edges - 3) times steps 5 and 6 with others positions
for times in range(1, edges - 2):
    line(inclination, times + 1, times)
    line(-inclination, times + 2, times)
# Step 8: Draw base line	
line(0, 0, 0)	
